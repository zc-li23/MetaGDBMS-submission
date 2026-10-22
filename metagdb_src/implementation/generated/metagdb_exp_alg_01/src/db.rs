//! MVCC state shared by the transaction manager and the storage engine.
//!
//! The frozen design vector selects snapshot isolation with MVCC, write-set
//! conflict detection, write-ahead-log recovery, and a persistent adjacency
//! layout with record versions carrying begin/end visibility timestamps.  This
//! module owns that state:
//!
//! * a multi-version record store (`Chain` of versions with `begin`/`end`
//!   timestamps, `end == INF` while a version is live),
//! * per-transaction write sets, so a transaction reads its own writes before
//!   those writes exist anywhere else,
//! * commit-time write-set conflict detection, so two writers of one key cannot
//!   both commit,
//! * per-vertex outgoing and incoming adjacency records,
//! * a framed, checksummed write-ahead log plus a compaction snapshot, which
//!   together give clean-reopen persistence and abrupt-exit recovery.
//!
//! Transaction state is deliberately *not* a global mutex standing in for
//! concurrency control.  The mutex only protects the shared index structure;
//! the observable semantics — stable snapshot reads, own-write visibility, and
//! first-committer-wins conflicts — come from the version chain.

use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasherDefault, Hasher};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
use metagdb_core::l3_txn::{IsolationLevel, TxnContext};
use metagdb_core::{Error, Result};

use crate::storage::codec::{crc32, FileSink, Reader, VecSink, Writer};

/// Hasher for the database's integer-keyed maps.
///
/// The default SipHash is a poor fit for a map whose keys are already dense or
/// near-dense integers: it costs several times more per lookup than the mixing
/// below, and the edge store is probed once per materialized neighbour. This is
/// a keyed-quality mix with no attack surface that matters for an internal
/// store the query language cannot address by raw key.
#[derive(Default)]
pub struct IdHasher(u64);

impl Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = (self.0 ^ *byte as u64).wrapping_mul(0x0100_0000_01b3);
        }
    }

    fn write_u64(&mut self, value: u64) {
        let mut mixed = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        self.0 = mixed ^ (mixed >> 31);
    }
}

/// A map keyed by a 64-bit identifier.
pub type IdMap<V> = HashMap<u64, V, BuildHasherDefault<IdHasher>>;

/// An open version never ends.
pub const INF: u64 = u64::MAX;

const WAL_HEADER: &[u8; 8] = b"MGDWAL01";
const SNAPSHOT_HEADER: &[u8; 8] = b"MGDBSNP1";
const SNAPSHOT_VERSION: u32 = 1;
const RECORD_COMMIT: u8 = 1;
const OP_PUT_VERTEX: u8 = 1;
const OP_DELETE_VERTEX: u8 = 2;
const OP_PUT_EDGE: u8 = 3;
const OP_DELETE_EDGE: u8 = 4;
/// A database whose log has grown past this many operations writes a
/// compaction snapshot. Measured on `uniform_standard`: compacting a 170k
/// operation log wrote a larger, slower-to-load snapshot than the log it
/// replaced, so the floor stays high enough that only a genuinely large log
/// compacts.
const MIN_COMPACTION_OPS: usize = 500_000;

/// One committed-or-live value with its visibility interval.
#[derive(Debug, Clone, PartialEq)]
pub struct Versioned<T> {
    pub begin: u64,
    pub end: u64,
    pub value: T,
}

impl<T> Versioned<T> {
    pub fn new(begin: u64, value: T) -> Self {
        Self {
            begin,
            end: INF,
            value,
        }
    }

    pub fn visible_at(&self, snapshot: u64) -> bool {
        self.begin <= snapshot && snapshot < self.end
    }
}

/// Version chain for one key: allocation-free while a key has one version.
#[derive(Debug, Clone)]
pub enum Chain<T> {
    One(Versioned<T>),
    Many(Vec<Versioned<T>>),
}

impl<T: Clone> Chain<T> {
    fn new(version: Versioned<T>) -> Self {
        Chain::One(version)
    }

    fn push(&mut self, version: Versioned<T>) {
        match self {
            Chain::One(existing) => {
                let first = existing.clone();
                *self = Chain::Many(vec![first, version]);
            }
            Chain::Many(versions) => versions.push(version),
        }
    }

    fn visible(&self, snapshot: u64) -> Option<&T> {
        match self {
            Chain::One(version) => version.visible_at(snapshot).then_some(&version.value),
            Chain::Many(versions) => versions
                .iter()
                .rev()
                .find(|version| version.visible_at(snapshot))
                .map(|version| &version.value),
        }
    }

    /// Close the live version at `ts`, so a newer version can replace it.
    /// Leaving the sealed version in place is what makes a delete or an update
    /// invisible to a snapshot opened before `ts`.
    fn seal(&mut self, ts: u64) {
        match self {
            Chain::One(version) => {
                if version.end == INF {
                    version.end = ts;
                }
            }
            Chain::Many(versions) => {
                if let Some(version) = versions.iter_mut().find(|version| version.end == INF) {
                    version.end = ts;
                }
            }
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Versioned<T>> + '_> {
        match self {
            Chain::One(version) => Box::new(std::iter::once(version)),
            Chain::Many(versions) => Box::new(versions.iter()),
        }
    }

    fn len(&self) -> usize {
        match self {
            Chain::One(_) => 1,
            Chain::Many(versions) => versions.len(),
        }
    }

    /// Drop versions no live reader can still observe.
    fn compact(&mut self, oldest_needed: u64) {
        if let Chain::Many(versions) = self {
            versions.retain(|version| version.end > oldest_needed || version.end == INF);
            if versions.len() == 1 {
                let only = versions.remove(0);
                *self = Chain::One(only);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TxnState {
    Active,
    Finished,
}

#[derive(Debug, Clone)]
enum Pending<T> {
    Put(T),
    Delete,
}

#[derive(Debug, Clone, Default)]
struct WriteSet {
    vertices: BTreeMap<u64, Pending<VertexRecord>>,
    edges: BTreeMap<u64, Pending<EdgeRecord>>,
}

impl WriteSet {
    fn is_empty(&self) -> bool {
        self.vertices.is_empty() && self.edges.is_empty()
    }

    fn len(&self) -> usize {
        self.vertices.len() + self.edges.len()
    }
}

#[derive(Clone)]
struct TxnMeta {
    ctx: TxnContext,
    state: TxnState,
    writes: WriteSet,
}

struct State {
    ts: u64,
    next_txn_id: u64,
    txns: IdMap<TxnMeta>,
    next_vertex_id: u64,
    next_edge_id: u64,
    vertices: BTreeMap<u64, Chain<VertexRecord>>,
    edges: IdMap<Chain<EdgeRecord>>,
    out_adjacency: IdMap<Vec<u64>>,
    in_adjacency: IdMap<Vec<u64>>,
    last_write_vertex: IdMap<u64>,
    last_write_edge: IdMap<u64>,
    directory: Option<PathBuf>,
    wal: Option<File>,
    /// Operations appended since the last compaction snapshot.
    ops_since_snapshot: usize,
    /// Live record count at the last compaction; sets the next threshold.
    records_at_snapshot: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ts: 0,
            next_txn_id: 1,
            txns: IdMap::default(),
            next_vertex_id: 1,
            next_edge_id: 1,
            vertices: BTreeMap::new(),
            edges: IdMap::default(),
            out_adjacency: IdMap::default(),
            in_adjacency: IdMap::default(),
            last_write_vertex: IdMap::default(),
            last_write_edge: IdMap::default(),
            directory: None,
            wal: None,
            ops_since_snapshot: 0,
            records_at_snapshot: 0,
        }
    }
}

impl State {
    fn snapshot_for(&self, ctx: &TxnContext) -> u64 {
        match ctx.isolation {
            IsolationLevel::ReadUncommitted | IsolationLevel::ReadCommitted => self.ts,
            _ => ctx.start_ts,
        }
    }

    fn txn(&self, ctx: &TxnContext) -> Result<&TxnMeta> {
        self.txns
            .get(&ctx.txn_id)
            .ok_or_else(|| Error::Transaction(format!("transaction {} is not open", ctx.txn_id)))
    }

    fn txn_mut(&mut self, ctx: &TxnContext) -> Result<&mut TxnMeta> {
        self.txns
            .get_mut(&ctx.txn_id)
            .ok_or_else(|| Error::Transaction(format!("transaction {} is not open", ctx.txn_id)))
    }

    fn ensure_writable(&self, ctx: &TxnContext) -> Result<()> {
        let txn = self.txn(ctx)?;
        if txn.state != TxnState::Active {
            return Err(Error::Transaction(format!(
                "transaction {} is no longer active",
                ctx.txn_id
            )));
        }
        if txn.ctx.read_only || ctx.read_only {
            return Err(Error::Transaction(format!(
                "transaction {} is read-only",
                ctx.txn_id
            )));
        }
        Ok(())
    }

    /// Resolve one record through the transaction's own writes, then the store.
    fn read_vertex(&self, ctx: &TxnContext, vid: VertexId) -> Result<Option<VertexRecord>> {
        let txn = self.txn(ctx)?;
        if let Some(pending) = txn.writes.vertices.get(&vid.0) {
            return Ok(match pending {
                Pending::Put(record) => Some(record.clone()),
                Pending::Delete => None,
            });
        }
        let snapshot = self.snapshot_for(ctx);
        Ok(self
            .vertices
            .get(&vid.0)
            .and_then(|chain| chain.visible(snapshot))
            .cloned())
    }

    fn read_edge(&self, ctx: &TxnContext, eid: EdgeId) -> Result<Option<EdgeRecord>> {
        let txn = self.txn(ctx)?;
        if let Some(pending) = txn.writes.edges.get(&eid.0) {
            return Ok(match pending {
                Pending::Put(record) => Some(record.clone()),
                Pending::Delete => None,
            });
        }
        let snapshot = self.snapshot_for(ctx);
        Ok(self
            .edges
            .get(&eid.0)
            .and_then(|chain| chain.visible(snapshot))
            .cloned())
    }

    fn scan_vertices(
        &self,
        ctx: &TxnContext,
        labels: Option<&[LabelId]>,
    ) -> Result<Vec<VertexRecord>> {
        let txn = self.txn(ctx)?;
        let snapshot = self.snapshot_for(ctx);
        let mut rows: BTreeMap<u64, VertexRecord> = BTreeMap::new();
        for (key, chain) in &self.vertices {
            if let Some(record) = chain.visible(snapshot) {
                rows.insert(*key, record.clone());
            }
        }
        for (key, pending) in &txn.writes.vertices {
            match pending {
                Pending::Put(record) => {
                    rows.insert(*key, record.clone());
                }
                Pending::Delete => {
                    rows.remove(key);
                }
            }
        }
        let matches = |record: &VertexRecord| match labels {
            Some(labels) => record.labels.iter().any(|label| labels.contains(label)),
            None => true,
        };
        Ok(rows.into_values().filter(matches).collect())
    }

    fn adjacency(&self, ctx: &TxnContext, vid: VertexId, outgoing: bool) -> Result<Vec<EdgeRecord>> {
        let txn = self.txn(ctx)?;
        let table = if outgoing {
            &self.out_adjacency
        } else {
            &self.in_adjacency
        };
        if txn.writes.edges.is_empty() {
            // Reading only: the stored list can be walked in place, which is the
            // common case for an analytical projection or a one-hop query.
            let Some(list) = table.get(&vid.0) else {
                return Ok(Vec::new());
            };
            let mut rows = Vec::with_capacity(list.len());
            for id in list {
                if let Some(record) = self.read_edge(ctx, EdgeId(*id))? {
                    rows.push(record);
                }
            }
            return Ok(rows);
        }
        // A write set is open, so the pending edges have to be folded in with
        // the stored ones. A flat vector copy is a memcpy, where cloning a tree
        // would allocate a node per neighbour; the sort afterwards also
        // restores the stable edge order the results rely on.
        let mut ids: Vec<u64> = match table.get(&vid.0) {
            Some(list) => list.clone(),
            None => Vec::new(),
        };
        for (key, pending) in &txn.writes.edges {
            if let Pending::Put(record) = pending {
                let touches = if outgoing {
                    record.src == vid
                } else {
                    record.dst == vid
                };
                if touches {
                    ids.push(*key);
                }
            }
        }
        ids.sort_unstable();
        ids.dedup();
        let mut rows = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(record) = self.read_edge(ctx, EdgeId(id))? {
                rows.push(record);
            }
        }
        Ok(rows)
    }

    fn has_visible_incident_edge(&self, ctx: &TxnContext, vid: VertexId) -> Result<bool> {
        Ok(!self.adjacency(ctx, vid, true)?.is_empty()
            || !self.adjacency(ctx, vid, false)?.is_empty())
    }

    /// First-committer-wins conflict check over the transaction's write set.
    ///
    /// The per-key commit marks catch deletes as well as updates: a delete seals
    /// the live version without adding a new one, so scanning versions alone
    /// would miss it.
    fn conflicts(&self, txn: &TxnMeta) -> Option<(String, u64)> {
        for key in txn.writes.vertices.keys() {
            if let Some(ts) = self.last_write_vertex.get(key) {
                if *ts > txn.ctx.start_ts {
                    return Some((format!("vertex {}", key), *ts));
                }
            }
        }
        for key in txn.writes.edges.keys() {
            if let Some(ts) = self.last_write_edge.get(key) {
                if *ts > txn.ctx.start_ts {
                    return Some((format!("edge {}", key), *ts));
                }
            }
        }
        None
    }
}

struct DbInner {
    state: Mutex<State>,
}

/// Shared handle to one persistent database directory.
#[derive(Clone)]
pub struct Db {
    inner: Arc<DbInner>,
}

impl Db {
    /// Open (or create) the database stored in `directory`.
    pub fn open(directory: &Path) -> Result<Self> {
        fs::create_dir_all(directory).map_err(io_error)?;
        let mut state = State::default();
        state.directory = Some(directory.to_path_buf());

        let snapshot_path = directory.join("snapshot.bin");
        if snapshot_path.exists() {
            load_snapshot(&mut state, &snapshot_path)?;
        }
        let wal_path = directory.join("wal.log");
        let wal_exists = wal_path.exists();
        if wal_exists {
            replay_wal(&mut state, &wal_path)?;
        }
        let mut wal = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&wal_path)
            .map_err(io_error)?;
        if !wal_exists {
            wal.write_all(WAL_HEADER).map_err(io_error)?;
            wal.sync_data().map_err(io_error)?;
        }
        state.wal = Some(wal);
        state.records_at_snapshot = state.vertices.len() + state.edges.len();

        Ok(Self {
            inner: Arc::new(DbInner {
                state: Mutex::new(state),
            }),
        })
    }

    /// Open a database with no disk backing; used by tests and the demo.
    pub fn in_memory() -> Self {
        Self {
            inner: Arc::new(DbInner {
                state: Mutex::new(State::default()),
            }),
        }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.inner
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Begin a transaction with the requested isolation level.
    pub fn begin(&self, isolation: IsolationLevel, read_only: bool) -> Result<TxnContext> {
        let mut state = self.lock();
        let txn_id = state.next_txn_id;
        state.next_txn_id += 1;
        let ctx = TxnContext {
            txn_id,
            isolation,
            read_only,
            start_ts: state.ts,
        };
        state.txns.insert(
            txn_id,
            TxnMeta {
                ctx: ctx.clone(),
                state: TxnState::Active,
                writes: WriteSet::default(),
            },
        );
        Ok(ctx)
    }

    /// Commit a transaction, applying its write set at a fresh commit timestamp.
    pub fn commit(&self, ctx: &TxnContext) -> Result<()> {
        let mut state = self.lock();
        // The write set is moved out of the table rather than cloned: an import
        // transaction can hold millions of records, and duplicating it would
        // double peak memory for no benefit.
        let mut txn = match state.txns.remove(&ctx.txn_id) {
            Some(txn) if txn.state == TxnState::Active => txn,
            Some(txn) => {
                state.txns.insert(ctx.txn_id, txn);
                return Err(Error::Transaction(format!(
                    "transaction {} is already finished",
                    ctx.txn_id
                )));
            }
            None => {
                return Err(Error::Transaction(format!(
                    "transaction {} is not open",
                    ctx.txn_id
                )))
            }
        };
        if txn.writes.is_empty() {
            finish(&mut state, ctx);
            return Ok(());
        }
        if let Some((key, ts)) = state.conflicts(&txn) {
            state.txns.insert(ctx.txn_id, txn);
            return Err(Error::Transaction(format!(
                "write conflict on {key}: committed at {ts} after this transaction began"
            )));
        }

        let commit_ts = state.ts + 1;
        state.ts = commit_ts;
        let ops = txn.writes.len();

        // The log record is written and flushed *before* the change becomes
        // visible, which is what makes an abrupt exit either keep the whole
        // committed write set or none of it.
        let writes = &txn.writes;
        append_wal(&mut state, |writer| {
            writer.u8(RECORD_COMMIT)?;
            writer.u64(commit_ts)?;
            writer.u32(ops as u32)?;
            for (key, pending) in &writes.vertices {
                match pending {
                    Pending::Put(record) => {
                        writer.u8(OP_PUT_VERTEX)?;
                        writer.vertex(record)?;
                    }
                    Pending::Delete => {
                        writer.u8(OP_DELETE_VERTEX)?;
                        writer.u64(*key)?;
                    }
                }
            }
            for (key, pending) in &writes.edges {
                match pending {
                    Pending::Put(record) => {
                        writer.u8(OP_PUT_EDGE)?;
                        writer.edge(record)?;
                    }
                    Pending::Delete => {
                        writer.u8(OP_DELETE_EDGE)?;
                        writer.u64(*key)?;
                    }
                }
            }
            Ok(())
        })?;

        // Draining the maps releases each record as it is applied, so peak
        // memory stays close to one copy of the write set.
        for (key, pending) in std::mem::take(&mut txn.writes.vertices) {
            state.last_write_vertex.insert(key, commit_ts);
            match pending {
                Pending::Put(record) => apply_put_vertex(&mut state.vertices, commit_ts, &record),
                Pending::Delete => {
                    if let Some(chain) = state.vertices.get_mut(&key) {
                        chain.seal(commit_ts);
                    }
                }
            }
        }
        for (key, pending) in std::mem::take(&mut txn.writes.edges) {
            state.last_write_edge.insert(key, commit_ts);
            match pending {
                Pending::Put(record) => {
                    // Only a newly created edge extends an adjacency list; a
                    // re-put of an existing edge updates it in place.
                    if !state.edges.contains_key(&record.id.0) {
                        state
                            .out_adjacency
                            .entry(record.src.0)
                            .or_default()
                            .push(record.id.0);
                        state
                            .in_adjacency
                            .entry(record.dst.0)
                            .or_default()
                            .push(record.id.0);
                    }
                    apply_put_edge(&mut state.edges, commit_ts, &record);
                }
                Pending::Delete => {
                    if let Some(chain) = state.edges.get_mut(&key) {
                        chain.seal(commit_ts);
                    }
                }
            }
        }

        state.ops_since_snapshot += ops;
        finish(&mut state, ctx);

        let threshold = state.records_at_snapshot.max(MIN_COMPACTION_OPS);
        if state.ops_since_snapshot >= threshold {
            compact(&mut state)?;
        }
        Ok(())
    }

    /// Abort a transaction, discarding its complete write set.
    pub fn abort(&self, ctx: &TxnContext) -> Result<()> {
        let mut state = self.lock();
        match state.txns.get(&ctx.txn_id) {
            Some(txn) if txn.state == TxnState::Active => {
                finish(&mut state, ctx);
                Ok(())
            }
            Some(_) => Err(Error::Transaction(format!(
                "transaction {} is already finished",
                ctx.txn_id
            ))),
            None => Err(Error::Transaction(format!(
                "transaction {} is not open",
                ctx.txn_id
            ))),
        }
    }

    pub fn read_vertex(&self, ctx: &TxnContext, vid: VertexId) -> Result<Option<VertexRecord>> {
        self.lock().read_vertex(ctx, vid)
    }

    pub fn read_edge(&self, ctx: &TxnContext, eid: EdgeId) -> Result<Option<EdgeRecord>> {
        self.lock().read_edge(ctx, eid)
    }

    pub fn scan_vertices(
        &self,
        ctx: &TxnContext,
        labels: Option<&[LabelId]>,
    ) -> Result<Vec<VertexRecord>> {
        self.lock().scan_vertices(ctx, labels)
    }

    pub fn out_edges(
        &self,
        ctx: &TxnContext,
        vid: VertexId,
        types: Option<&[EdgeTypeId]>,
    ) -> Result<Vec<EdgeRecord>> {
        let rows = self.lock().adjacency(ctx, vid, true)?;
        Ok(filter_types(rows, types))
    }

    pub fn in_edges(
        &self,
        ctx: &TxnContext,
        vid: VertexId,
        types: Option<&[EdgeTypeId]>,
    ) -> Result<Vec<EdgeRecord>> {
        let rows = self.lock().adjacency(ctx, vid, false)?;
        Ok(filter_types(rows, types))
    }

    /// Insert a vertex; `VertexId(0)` asks storage to allocate the next ID.
    pub fn insert_vertex(&self, ctx: &TxnContext, mut record: VertexRecord) -> Result<VertexId> {
        let mut state = self.lock();
        state.ensure_writable(ctx)?;
        let id = if record.id.0 == 0 {
            let id = state.next_vertex_id;
            state.next_vertex_id += 1;
            VertexId(id)
        } else {
            VertexId(record.id.0)
        };
        record.id = id;
        let duplicate_in_txn = matches!(
            state.txn(ctx)?.writes.vertices.get(&id.0),
            Some(Pending::Put(_))
        );
        if duplicate_in_txn || state.read_vertex(ctx, id)?.is_some() {
            return Err(Error::Storage(format!("vertex {} already exists", id.0)));
        }
        if state.next_vertex_id <= id.0 {
            state.next_vertex_id = id.0 + 1;
        }
        state
            .txn_mut(ctx)?
            .writes
            .vertices
            .insert(id.0, Pending::Put(record));
        Ok(id)
    }

    /// Insert an edge; both endpoints must be visible to the transaction.
    pub fn insert_edge(&self, ctx: &TxnContext, mut record: EdgeRecord) -> Result<EdgeId> {
        let mut state = self.lock();
        state.ensure_writable(ctx)?;
        if state.read_vertex(ctx, record.src)?.is_none() {
            return Err(Error::Storage(format!(
                "edge source vertex {} does not exist",
                record.src.0
            )));
        }
        if state.read_vertex(ctx, record.dst)?.is_none() {
            return Err(Error::Storage(format!(
                "edge destination vertex {} does not exist",
                record.dst.0
            )));
        }
        let id = if record.id.0 == 0 {
            let id = state.next_edge_id;
            state.next_edge_id += 1;
            EdgeId(id)
        } else {
            EdgeId(record.id.0)
        };
        record.id = id;
        if state.read_edge(ctx, id)?.is_some() {
            return Err(Error::Storage(format!("edge {} already exists", id.0)));
        }
        if state.next_edge_id <= id.0 {
            state.next_edge_id = id.0 + 1;
        }
        state
            .txn_mut(ctx)?
            .writes
            .edges
            .insert(id.0, Pending::Put(record));
        Ok(id)
    }

    pub fn update_vertex(&self, ctx: &TxnContext, record: VertexRecord) -> Result<()> {
        let mut state = self.lock();
        state.ensure_writable(ctx)?;
        if state.read_vertex(ctx, record.id)?.is_none() {
            return Err(Error::NotFound(format!("vertex {}", record.id.0)));
        }
        state
            .txn_mut(ctx)?
            .writes
            .vertices
            .insert(record.id.0, Pending::Put(record));
        Ok(())
    }

    pub fn update_edge(&self, ctx: &TxnContext, record: EdgeRecord) -> Result<()> {
        let mut state = self.lock();
        state.ensure_writable(ctx)?;
        let current = state
            .read_edge(ctx, record.id)?
            .ok_or_else(|| Error::NotFound(format!("edge {}", record.id.0)))?;
        if current.src != record.src || current.dst != record.dst || current.etype != record.etype {
            return Err(Error::Storage(format!(
                "edge {} endpoints and type are immutable",
                record.id.0
            )));
        }
        state
            .txn_mut(ctx)?
            .writes
            .edges
            .insert(record.id.0, Pending::Put(record));
        Ok(())
    }

    pub fn delete_vertex(&self, ctx: &TxnContext, vid: VertexId) -> Result<()> {
        let mut state = self.lock();
        state.ensure_writable(ctx)?;
        if state.read_vertex(ctx, vid)?.is_none() {
            return Err(Error::NotFound(format!("vertex {}", vid.0)));
        }
        if state.has_visible_incident_edge(ctx, vid)? {
            return Err(Error::Storage(format!(
                "vertex {} still has incident edges",
                vid.0
            )));
        }
        state
            .txn_mut(ctx)?
            .writes
            .vertices
            .insert(vid.0, Pending::Delete);
        Ok(())
    }

    pub fn delete_edge(&self, ctx: &TxnContext, eid: EdgeId) -> Result<()> {
        let mut state = self.lock();
        state.ensure_writable(ctx)?;
        if state.read_edge(ctx, eid)?.is_none() {
            return Err(Error::NotFound(format!("edge {}", eid.0)));
        }
        state
            .txn_mut(ctx)?
            .writes
            .edges
            .insert(eid.0, Pending::Delete);
        Ok(())
    }

    /// Number of keys currently held by the version store.
    pub fn record_counts(&self) -> (usize, usize) {
        let state = self.lock();
        (state.vertices.len(), state.edges.len())
    }

    pub fn current_timestamp(&self) -> u64 {
        self.lock().ts
    }

    pub fn open_transactions(&self) -> usize {
        self.lock()
            .txns
            .values()
            .filter(|txn| txn.state == TxnState::Active)
            .count()
    }

    /// Force a compaction snapshot; used by tests and clean shutdown paths.
    pub fn checkpoint(&self) -> Result<()> {
        let mut state = self.lock();
        compact(&mut state)
    }
}

/// Mark a transaction finished, keeping its identity so a repeated commit or
/// abort is reported as a lifecycle error instead of "not open".  The commit
/// path removes the entry while it applies the write set, so this also
/// re-inserts the finished marker.
fn finish(state: &mut State, ctx: &TxnContext) {
    let entry = state.txns.entry(ctx.txn_id).or_insert_with(|| TxnMeta {
        ctx: ctx.clone(),
        state: TxnState::Finished,
        writes: WriteSet::default(),
    });
    entry.state = TxnState::Finished;
    entry.writes = WriteSet::default();
}

fn filter_types(rows: Vec<EdgeRecord>, types: Option<&[EdgeTypeId]>) -> Vec<EdgeRecord> {
    match types {
        Some(types) => rows
            .into_iter()
            .filter(|edge| types.contains(&edge.etype))
            .collect(),
        None => rows,
    }
}

fn io_error(error: std::io::Error) -> Error {
    Error::Storage(error.to_string())
}

fn apply_put_vertex(
    table: &mut BTreeMap<u64, Chain<VertexRecord>>,
    commit_ts: u64,
    record: &VertexRecord,
) {
    match table.get_mut(&record.id.0) {
        Some(chain) => {
            chain.seal(commit_ts);
            chain.push(Versioned::new(commit_ts, record.clone()));
        }
        None => {
            table.insert(
                record.id.0,
                Chain::new(Versioned::new(commit_ts, record.clone())),
            );
        }
    }
}

fn apply_put_edge(
    table: &mut IdMap<Chain<EdgeRecord>>,
    commit_ts: u64,
    record: &EdgeRecord,
) {
    match table.get_mut(&record.id.0) {
        Some(chain) => {
            chain.seal(commit_ts);
            chain.push(Versioned::new(commit_ts, record.clone()));
        }
        None => {
            table.insert(
                record.id.0,
                Chain::new(Versioned::new(commit_ts, record.clone())),
            );
        }
    }
}

/// Append one framed, checksummed log record and flush it to stable storage.
fn append_wal<F>(state: &mut State, encode: F) -> Result<()>
where
    F: FnOnce(&mut Writer<FileSink<'_>>) -> Result<()>,
{
    let Some(wal) = state.wal.as_mut() else {
        return Ok(());
    };
    let start = wal.seek(SeekFrom::End(0)).map_err(io_error)?;
    // Reserve the frame header; the length and checksum are patched in once the
    // payload has streamed past this point.
    wal.write_all(&[0u8; 8]).map_err(io_error)?;
    let (checksum, length) = {
        let mut writer = Writer::new(FileSink::new(&mut *wal));
        encode(&mut writer)?;
        writer.into_sink().finish()?
    };
    let end = start + 8 + length;
    wal.seek(SeekFrom::Start(start)).map_err(io_error)?;
    wal.write_all(&(length as u32).to_le_bytes())
        .map_err(io_error)?;
    wal.write_all(&checksum.to_le_bytes()).map_err(io_error)?;
    wal.seek(SeekFrom::Start(end)).map_err(io_error)?;
    wal.sync_data().map_err(io_error)?;
    Ok(())
}

fn load_snapshot(state: &mut State, path: &Path) -> Result<()> {
    let mut file = File::open(path).map_err(io_error)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(io_error)?;
    if bytes.len() < 12 || &bytes[..8] != SNAPSHOT_HEADER {
        return Err(Error::Storage("snapshot header is invalid".into()));
    }
    let body = &bytes[..bytes.len() - 4];
    let mut checksum = [0u8; 4];
    checksum.copy_from_slice(&bytes[bytes.len() - 4..]);
    if crc32(body) != u32::from_le_bytes(checksum) {
        return Err(Error::Storage("snapshot checksum mismatch".into()));
    }
    let mut reader = Reader::new(&body[8..]);
    let version = reader.u32()?;
    if version != SNAPSHOT_VERSION {
        return Err(Error::Storage(format!(
            "unsupported snapshot version {version}"
        )));
    }
    state.ts = reader.u64()?;
    state.next_vertex_id = reader.u64()?;
    state.next_edge_id = reader.u64()?;
    state.next_txn_id = reader.u64()?;

    let vertex_count = reader.u32()? as usize;
    for _ in 0..vertex_count {
        let begin = reader.u64()?;
        let end = reader.u64()?;
        let record = reader.vertex()?;
        state.vertices.insert(
            record.id.0,
            Chain::new(Versioned {
                begin,
                end,
                value: record,
            }),
        );
    }
    let edge_count = reader.u32()? as usize;
    for _ in 0..edge_count {
        let begin = reader.u64()?;
        let end = reader.u64()?;
        let record = reader.edge()?;
        state
            .out_adjacency
            .entry(record.src.0)
            .or_default()
            .push(record.id.0);
        state
            .in_adjacency
            .entry(record.dst.0)
            .or_default()
            .push(record.id.0);
        state.edges.insert(
            record.id.0,
            Chain::new(Versioned {
                begin,
                end,
                value: record,
            }),
        );
    }
    let vertex_marks = reader.u32()? as usize;
    for _ in 0..vertex_marks {
        let key = reader.u64()?;
        let ts = reader.u64()?;
        state.last_write_vertex.insert(key, ts);
    }
    let edge_marks = reader.u32()? as usize;
    for _ in 0..edge_marks {
        let key = reader.u64()?;
        let ts = reader.u64()?;
        state.last_write_edge.insert(key, ts);
    }
    Ok(())
}

fn replay_wal(state: &mut State, path: &Path) -> Result<()> {
    let mut file = File::open(path).map_err(io_error)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(io_error)?;
    if bytes.len() < WAL_HEADER.len() || &bytes[..WAL_HEADER.len()] != WAL_HEADER {
        return Err(Error::Storage("write-ahead log header is invalid".into()));
    }
    let mut position = WAL_HEADER.len();
    while position + 8 <= bytes.len() {
        let mut length = [0u8; 4];
        length.copy_from_slice(&bytes[position..position + 4]);
        let length = u32::from_le_bytes(length) as usize;
        let mut checksum = [0u8; 4];
        checksum.copy_from_slice(&bytes[position + 4..position + 8]);
        let start = position + 8;
        let end = start + length;
        if end > bytes.len() {
            // Torn tail record: the process died mid-append. Discard it.
            break;
        }
        let payload = &bytes[start..end];
        if crc32(payload) != u32::from_le_bytes(checksum) {
            break;
        }
        apply_commit(state, payload)?;
        position = end;
    }
    Ok(())
}

fn apply_commit(state: &mut State, payload: &[u8]) -> Result<()> {
    let mut reader = Reader::new(payload);
    let kind = reader.u8()?;
    if kind != RECORD_COMMIT {
        return Err(Error::Storage(format!("unknown log record kind {kind}")));
    }
    let commit_ts = reader.u64()?;
    let count = reader.u32()? as usize;
    state.ts = state.ts.max(commit_ts);
    for _ in 0..count {
        match reader.u8()? {
            OP_PUT_VERTEX => {
                let record = reader.vertex()?;
                state.last_write_vertex.insert(record.id.0, commit_ts);
                state.next_vertex_id = state.next_vertex_id.max(record.id.0 + 1);
                apply_put_vertex(&mut state.vertices, commit_ts, &record);
            }
            OP_DELETE_VERTEX => {
                let key = reader.u64()?;
                state.last_write_vertex.insert(key, commit_ts);
                if let Some(chain) = state.vertices.get_mut(&key) {
                    chain.seal(commit_ts);
                }
            }
            OP_PUT_EDGE => {
                let record = reader.edge()?;
                state.last_write_edge.insert(record.id.0, commit_ts);
                state.next_edge_id = state.next_edge_id.max(record.id.0 + 1);
                if !state.edges.contains_key(&record.id.0) {
                    state
                        .out_adjacency
                        .entry(record.src.0)
                        .or_default()
                        .push(record.id.0);
                    state
                        .in_adjacency
                        .entry(record.dst.0)
                        .or_default()
                        .push(record.id.0);
                }
                apply_put_edge(&mut state.edges, commit_ts, &record);
            }
            OP_DELETE_EDGE => {
                let key = reader.u64()?;
                state.last_write_edge.insert(key, commit_ts);
                if let Some(chain) = state.edges.get_mut(&key) {
                    chain.seal(commit_ts);
                }
            }
            other => {
                return Err(Error::Storage(format!("unknown log operation {other}")));
            }
        }
    }
    Ok(())
}

/// Write a compact snapshot of the version store and truncate the log.
fn compact(state: &mut State) -> Result<()> {
    let Some(directory) = state.directory.clone() else {
        state.ops_since_snapshot = 0;
        return Ok(());
    };
    let oldest_needed = state
        .txns
        .values()
        .filter(|txn| txn.state == TxnState::Active)
        .map(|txn| txn.ctx.start_ts)
        .min()
        .unwrap_or(state.ts + 1);

    // A version is retained exactly when some live reader could still observe
    // it; the snapshot and the in-memory chains therefore end up identical.
    fn retains<T>(version: &&Versioned<T>, oldest_needed: u64) -> bool {
        version.end > oldest_needed || version.end == INF
    }
    let vertex_keys: Vec<u64> = state
        .vertices
        .iter()
        .filter(|(_, chain)| chain.iter().any(|version| retains(&version, oldest_needed)))
        .map(|(key, _)| *key)
        .collect();
    let edge_keys: Vec<u64> = state
        .edges
        .iter()
        .filter(|(_, chain)| chain.iter().any(|version| retains(&version, oldest_needed)))
        .map(|(key, _)| *key)
        .collect();
    let vertex_versions = state.vertices.values().map(Chain::len).sum::<usize>();
    let edge_versions = state.edges.values().map(Chain::len).sum::<usize>();
    let mut writer = Writer::new(VecSink::with_capacity(
        64 + vertex_versions * 24 + edge_versions * 32,
    ));
    writer.raw(SNAPSHOT_HEADER)?;
    writer.u32(SNAPSHOT_VERSION)?;
    writer.u64(state.ts)?;
    writer.u64(state.next_vertex_id)?;
    writer.u64(state.next_edge_id)?;
    writer.u64(state.next_txn_id)?;

    writer.u32(vertex_keys.len() as u32)?;
    for key in &vertex_keys {
        if let Some(chain) = state.vertices.get(key) {
            for version in chain.iter().filter(|version| retains(&version, oldest_needed)) {
                writer.u64(version.begin)?;
                writer.u64(version.end)?;
                writer.vertex(&version.value)?;
            }
        }
    }
    writer.u32(edge_keys.len() as u32)?;
    for key in &edge_keys {
        if let Some(chain) = state.edges.get(key) {
            for version in chain.iter().filter(|version| retains(&version, oldest_needed)) {
                writer.u64(version.begin)?;
                writer.u64(version.end)?;
                writer.edge(&version.value)?;
            }
        }
    }
    writer.u32(state.last_write_vertex.len() as u32)?;
    for (key, ts) in &state.last_write_vertex {
        writer.u64(*key)?;
        writer.u64(*ts)?;
    }
    writer.u32(state.last_write_edge.len() as u32)?;
    for (key, ts) in &state.last_write_edge {
        writer.u64(*key)?;
        writer.u64(*ts)?;
    }

    let body = writer.into_bytes();
    let checksum = crc32(&body).to_le_bytes();
    let temporary = directory.join("snapshot.tmp");
    let final_path = directory.join("snapshot.bin");
    {
        let mut file = File::create(&temporary).map_err(io_error)?;
        file.write_all(&body).map_err(io_error)?;
        file.write_all(&checksum).map_err(io_error)?;
        file.sync_all().map_err(io_error)?;
    }
    fs::rename(&temporary, &final_path).map_err(io_error)?;
    if let Ok(handle) = File::open(&directory) {
        let _ = handle.sync_all();
    }

    // Only now is it safe to drop the log: the snapshot already contains every
    // version any open transaction can still need.
    if let Some(wal) = state.wal.as_mut() {
        wal.set_len(0).map_err(io_error)?;
        wal.seek(SeekFrom::Start(0)).map_err(io_error)?;
        wal.write_all(WAL_HEADER).map_err(io_error)?;
        wal.sync_data().map_err(io_error)?;
    }

    for chain in state.vertices.values_mut() {
        chain.compact(oldest_needed);
    }
    for chain in state.edges.values_mut() {
        chain.compact(oldest_needed);
    }
    state.vertices.retain(|_, chain| chain.len() != 0);
    state.edges.retain(|_, chain| chain.len() != 0);
    state.ops_since_snapshot = 0;
    state.records_at_snapshot = state.vertices.len() + state.edges.len();
    Ok(())
}

/// Build a property map from key/value pairs; convenience for import and tests.
pub fn property_map(entries: &[(PropertyKeyId, PropertyValue)]) -> PropertyMap {
    entries.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    const SNAPSHOT: IsolationLevel = IsolationLevel::SnapshotIsolation;

    fn scratch_dir(tag: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "metagdb-exp-alg-01-{tag}-{}-{nanos}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    fn vertex(id: u64, value: i64) -> VertexRecord {
        VertexRecord {
            id: VertexId(id),
            labels: vec![LabelId(1)],
            properties: property_map(&[(PropertyKeyId(1), PropertyValue::Int(value))]),
        }
    }

    fn edge(src: VertexId, dst: VertexId, value: i64) -> EdgeRecord {
        EdgeRecord {
            id: EdgeId(0),
            src,
            dst,
            etype: EdgeTypeId(1),
            properties: property_map(&[(PropertyKeyId(1), PropertyValue::Int(value))]),
        }
    }

    fn value_of(record: &VertexRecord) -> Option<i64> {
        record
            .properties
            .get(&PropertyKeyId(1))
            .and_then(PropertyValue::as_i64)
    }

    #[test]
    fn snapshot_reads_are_stable_after_a_later_commit() {
        let db = Db::in_memory();
        let seed = db.begin(SNAPSHOT, false).unwrap();
        let id = db.insert_vertex(&seed, vertex(0, 1)).unwrap();
        db.commit(&seed).unwrap();

        let reader = db.begin(SNAPSHOT, true).unwrap();
        let writer = db.begin(SNAPSHOT, false).unwrap();
        db.update_vertex(&writer, vertex(id.0, 2)).unwrap();
        db.commit(&writer).unwrap();

        assert_eq!(
            db.read_vertex(&reader, id).unwrap().as_ref().and_then(value_of),
            Some(1)
        );
        db.commit(&reader).unwrap();
        let fresh = db.begin(SNAPSHOT, true).unwrap();
        assert_eq!(
            db.read_vertex(&fresh, id).unwrap().as_ref().and_then(value_of),
            Some(2)
        );
    }

    #[test]
    fn conflicting_writers_cannot_both_commit() {
        let db = Db::in_memory();
        let seed = db.begin(SNAPSHOT, false).unwrap();
        let id = db.insert_vertex(&seed, vertex(0, 0)).unwrap();
        db.commit(&seed).unwrap();

        let first = db.begin(SNAPSHOT, false).unwrap();
        let second = db.begin(SNAPSHOT, false).unwrap();
        db.update_vertex(&first, vertex(id.0, 1)).unwrap();
        db.update_vertex(&second, vertex(id.0, 2)).unwrap();
        assert!(db.commit(&first).is_ok());
        assert!(db.commit(&second).is_err());
    }

    #[test]
    fn a_delete_also_conflicts_with_a_concurrent_writer() {
        let db = Db::in_memory();
        let seed = db.begin(SNAPSHOT, false).unwrap();
        let id = db.insert_vertex(&seed, vertex(0, 0)).unwrap();
        let other = db.insert_vertex(&seed, vertex(0, 0)).unwrap();
        db.commit(&seed).unwrap();

        let deleter = db.begin(SNAPSHOT, false).unwrap();
        let updater = db.begin(SNAPSHOT, false).unwrap();
        db.delete_vertex(&deleter, other).unwrap();
        db.update_vertex(&updater, vertex(id.0, 9)).unwrap();
        assert!(db.commit(&deleter).is_ok());
        assert!(db.commit(&updater).is_ok());

        // The same key deleted and updated concurrently must not both win.
        let deleter = db.begin(SNAPSHOT, false).unwrap();
        let updater = db.begin(SNAPSHOT, false).unwrap();
        db.delete_vertex(&deleter, id).unwrap();
        db.update_vertex(&updater, vertex(id.0, 3)).unwrap();
        assert!(db.commit(&deleter).is_ok());
        assert!(db.commit(&updater).is_err());
    }

    #[test]
    fn abort_discards_the_complete_write_set() {
        let db = Db::in_memory();
        let txn = db.begin(SNAPSHOT, false).unwrap();
        let a = db.insert_vertex(&txn, vertex(0, 1)).unwrap();
        let b = db.insert_vertex(&txn, vertex(0, 2)).unwrap();
        db.abort(&txn).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert!(db.read_vertex(&reader, a).unwrap().is_none());
        assert!(db.read_vertex(&reader, b).unwrap().is_none());
        assert_eq!(db.scan_vertices(&reader, None).unwrap().len(), 0);
    }

    #[test]
    fn a_transaction_reads_its_own_writes_and_hides_them_from_others() {
        let db = Db::in_memory();
        let writer = db.begin(SNAPSHOT, false).unwrap();
        let id = db.insert_vertex(&writer, vertex(0, 5)).unwrap();
        assert_eq!(
            db.read_vertex(&writer, id).unwrap().as_ref().and_then(value_of),
            Some(5)
        );
        assert!(db
            .scan_vertices(&writer, None)
            .unwrap()
            .iter()
            .any(|v| v.id == id));

        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert!(db.read_vertex(&reader, id).unwrap().is_none());
        assert!(!db
            .scan_vertices(&reader, None)
            .unwrap()
            .iter()
            .any(|v| v.id == id));
        db.abort(&writer).unwrap();
    }

    #[test]
    fn a_transaction_sees_its_own_update_and_deletion() {
        let db = Db::in_memory();
        let seed = db.begin(SNAPSHOT, false).unwrap();
        let id = db.insert_vertex(&seed, vertex(0, 1)).unwrap();
        db.commit(&seed).unwrap();

        let txn = db.begin(SNAPSHOT, false).unwrap();
        db.update_vertex(&txn, vertex(id.0, 2)).unwrap();
        assert_eq!(
            db.read_vertex(&txn, id).unwrap().as_ref().and_then(value_of),
            Some(2)
        );
        db.delete_vertex(&txn, id).unwrap();
        assert!(db.read_vertex(&txn, id).unwrap().is_none());
        assert!(!db
            .scan_vertices(&txn, None)
            .unwrap()
            .iter()
            .any(|v| v.id == id));
        db.abort(&txn).unwrap();
    }

    #[test]
    fn an_uncommitted_update_is_not_visible_to_a_reader() {
        let db = Db::in_memory();
        let seed = db.begin(SNAPSHOT, false).unwrap();
        let id = db.insert_vertex(&seed, vertex(0, 1)).unwrap();
        db.commit(&seed).unwrap();

        let writer = db.begin(SNAPSHOT, false).unwrap();
        db.update_vertex(&writer, vertex(id.0, 2)).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert_eq!(
            db.read_vertex(&reader, id).unwrap().as_ref().and_then(value_of),
            Some(1)
        );
        db.commit(&reader).unwrap();
        db.abort(&writer).unwrap();
    }

    #[test]
    fn read_only_transactions_reject_mutations() {
        let db = Db::in_memory();
        let txn = db.begin(SNAPSHOT, true).unwrap();
        assert!(db.insert_vertex(&txn, vertex(0, 1)).is_err());
        db.abort(&txn).unwrap();
    }

    #[test]
    fn a_transaction_cannot_commit_twice() {
        let db = Db::in_memory();
        let txn = db.begin(SNAPSHOT, false).unwrap();
        let clone = txn.clone();
        db.commit(&txn).unwrap();
        assert!(db.commit(&clone).is_err());
        assert!(db.abort(&txn).is_err());
    }

    #[test]
    fn committed_data_and_id_allocation_survive_a_reopen() {
        let dir = scratch_dir("reopen");
        let first = {
            let db = Db::open(&dir).unwrap();
            let txn = db.begin(SNAPSHOT, false).unwrap();
            let id = db.insert_vertex(&txn, vertex(0, 41)).unwrap();
            db.commit(&txn).unwrap();
            id
        };
        let db = Db::open(&dir).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert_eq!(
            db.read_vertex(&reader, first).unwrap().as_ref().and_then(value_of),
            Some(41)
        );
        db.commit(&reader).unwrap();

        let txn = db.begin(SNAPSHOT, false).unwrap();
        let second = db.insert_vertex(&txn, vertex(0, 42)).unwrap();
        db.commit(&txn).unwrap();
        assert!(second.0 > first.0);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn an_aborted_transaction_leaves_nothing_behind_after_reopen() {
        let dir = scratch_dir("aborted");
        let id = {
            let db = Db::open(&dir).unwrap();
            let txn = db.begin(SNAPSHOT, false).unwrap();
            let id = db.insert_vertex(&txn, vertex(0, 41)).unwrap();
            db.abort(&txn).unwrap();
            id
        };
        let db = Db::open(&dir).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert!(db.read_vertex(&reader, id).unwrap().is_none());
        db.commit(&reader).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_torn_log_tail_is_discarded_on_replay() {
        let dir = scratch_dir("torn");
        {
            let db = Db::open(&dir).unwrap();
            let txn = db.begin(SNAPSHOT, false).unwrap();
            let first = db.insert_vertex(&txn, vertex(0, 7)).unwrap();
            db.commit(&txn).unwrap();
            assert_eq!(first.0, 1);
        }
        let wal_path = dir.join("wal.log");
        let mut bytes = fs::read(&wal_path).unwrap();
        bytes.extend_from_slice(&[0, 0, 0, 16, 1, 2, 3]);
        fs::write(&wal_path, &bytes).unwrap();

        let db = Db::open(&dir).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert_eq!(db.scan_vertices(&reader, None).unwrap().len(), 1);
        db.commit(&reader).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_corrupt_record_body_is_discarded_on_replay() {
        let dir = scratch_dir("corrupt");
        {
            let db = Db::open(&dir).unwrap();
            for value in 0..3i64 {
                let txn = db.begin(SNAPSHOT, false).unwrap();
                db.insert_vertex(&txn, vertex(0, value)).unwrap();
                db.commit(&txn).unwrap();
            }
        }
        let wal_path = dir.join("wal.log");
        let mut bytes = fs::read(&wal_path).unwrap();
        // Flip one byte inside the payload of the last record.
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        fs::write(&wal_path, &bytes).unwrap();

        let db = Db::open(&dir).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert_eq!(db.scan_vertices(&reader, None).unwrap().len(), 2);
        db.commit(&reader).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn adjacency_tracks_both_directions_and_survives_reopen() {
        let dir = scratch_dir("adjacency");
        let (a, b, e) = {
            let db = Db::open(&dir).unwrap();
            let txn = db.begin(SNAPSHOT, false).unwrap();
            let a = db.insert_vertex(&txn, vertex(0, 1)).unwrap();
            let b = db.insert_vertex(&txn, vertex(0, 2)).unwrap();
            let e = db.insert_edge(&txn, edge(a, b, 5)).unwrap();
            db.commit(&txn).unwrap();
            (a, b, e)
        };
        let db = Db::open(&dir).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert_eq!(db.out_edges(&reader, a, None).unwrap().len(), 1);
        assert_eq!(db.in_edges(&reader, b, None).unwrap().len(), 1);
        assert_eq!(db.read_edge(&reader, e).unwrap().unwrap().src, a);
        db.commit(&reader).unwrap();

        let txn = db.begin(SNAPSHOT, false).unwrap();
        db.delete_edge(&txn, e).unwrap();
        db.commit(&txn).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert!(db.read_edge(&reader, e).unwrap().is_none());
        assert!(db.out_edges(&reader, a, None).unwrap().is_empty());
        assert!(db.in_edges(&reader, b, None).unwrap().is_empty());
        db.commit(&reader).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_deleted_record_is_still_visible_to_an_older_snapshot() {
        let db = Db::in_memory();
        let seed = db.begin(SNAPSHOT, false).unwrap();
        let id = db.insert_vertex(&seed, vertex(0, 1)).unwrap();
        db.commit(&seed).unwrap();

        let reader = db.begin(SNAPSHOT, true).unwrap();
        let writer = db.begin(SNAPSHOT, false).unwrap();
        db.delete_vertex(&writer, id).unwrap();
        db.commit(&writer).unwrap();

        assert!(db.read_vertex(&reader, id).unwrap().is_some());
        db.commit(&reader).unwrap();
        let fresh = db.begin(SNAPSHOT, true).unwrap();
        assert!(db.read_vertex(&fresh, id).unwrap().is_none());
    }

    #[test]
    fn vertex_deletion_with_incident_edges_is_rejected() {
        let db = Db::in_memory();
        let txn = db.begin(SNAPSHOT, false).unwrap();
        let a = db.insert_vertex(&txn, vertex(0, 1)).unwrap();
        let b = db.insert_vertex(&txn, vertex(0, 2)).unwrap();
        db.insert_edge(&txn, edge(a, b, 1)).unwrap();
        assert!(db.delete_vertex(&txn, a).is_err());
        db.commit(&txn).unwrap();

        let txn = db.begin(SNAPSHOT, false).unwrap();
        for record in db.out_edges(&txn, a, None).unwrap() {
            db.delete_edge(&txn, record.id).unwrap();
        }
        assert!(db.delete_vertex(&txn, a).is_ok());
        db.commit(&txn).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert!(db.read_vertex(&reader, a).unwrap().is_none());
    }

    #[test]
    fn an_edge_cannot_reference_a_missing_endpoint() {
        let db = Db::in_memory();
        let txn = db.begin(SNAPSHOT, false).unwrap();
        let a = db.insert_vertex(&txn, vertex(0, 1)).unwrap();
        let result = db.insert_edge(&txn, edge(a, VertexId(a.0 + 999_999), 1));
        assert!(result.is_err());
        db.abort(&txn).unwrap();
    }

    #[test]
    fn edge_endpoints_and_type_are_immutable() {
        let db = Db::in_memory();
        let txn = db.begin(SNAPSHOT, false).unwrap();
        let a = db.insert_vertex(&txn, vertex(0, 1)).unwrap();
        let b = db.insert_vertex(&txn, vertex(0, 2)).unwrap();
        let e = db.insert_edge(&txn, edge(a, b, 1)).unwrap();
        db.commit(&txn).unwrap();

        let txn = db.begin(SNAPSHOT, false).unwrap();
        let mut changed = db.read_edge(&txn, e).unwrap().unwrap();
        changed.dst = VertexId(b.0 + 1);
        assert!(db.update_edge(&txn, changed).is_err());
        db.abort(&txn).unwrap();
    }

    #[test]
    fn compaction_preserves_the_graph_across_reopen() {
        let dir = scratch_dir("compaction");
        {
            let db = Db::open(&dir).unwrap();
            let txn = db.begin(SNAPSHOT, false).unwrap();
            let a = db.insert_vertex(&txn, vertex(0, 1)).unwrap();
            let b = db.insert_vertex(&txn, vertex(0, 2)).unwrap();
            let e = db.insert_edge(&txn, edge(a, b, 5)).unwrap();
            db.commit(&txn).unwrap();

            let txn = db.begin(SNAPSHOT, false).unwrap();
            db.delete_edge(&txn, e).unwrap();
            db.commit(&txn).unwrap();
            db.checkpoint().unwrap();
        }
        let db = Db::open(&dir).unwrap();
        let reader = db.begin(SNAPSHOT, true).unwrap();
        assert_eq!(db.scan_vertices(&reader, None).unwrap().len(), 2);
        assert_eq!(db.record_counts(), (2, 0));
        db.commit(&reader).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn deterministic_history_matches_an_independent_model() {
        let db = Db::in_memory();
        let mut model = BTreeMap::<VertexId, i64>::new();
        for step in 0..24i64 {
            let txn = db.begin(SNAPSHOT, false).unwrap();
            let inserted = db.insert_vertex(&txn, vertex(0, 100 + step)).unwrap();
            let existing = model.keys().next().copied();
            if let Some(id) = existing {
                db.update_vertex(&txn, vertex(id.0, 1_000 + step)).unwrap();
            }
            if step % 4 != 0 {
                db.commit(&txn).unwrap();
                model.insert(inserted, 100 + step);
                if let Some(id) = existing {
                    model.insert(id, 1_000 + step);
                }
            } else {
                db.abort(&txn).unwrap();
            }
            let reader = db.begin(SNAPSHOT, true).unwrap();
            let actual: BTreeMap<VertexId, i64> = db
                .scan_vertices(&reader, None)
                .unwrap()
                .iter()
                .filter_map(|record| value_of(record).map(|value| (record.id, value)))
                .collect();
            db.commit(&reader).unwrap();
            assert_eq!(actual, model, "mismatch after step {step}");
        }
    }
}
