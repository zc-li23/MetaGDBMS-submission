//! Persistent, MVCC, crash-safe graph storage.
//!
//! Layout (per candidate design vector):
//! * per-vertex adjacency index for out/in traversal and a point index for
//!   vertices and edges;
//! * record versions carrying begin/end visibility timestamps plus the creating
//!   transaction, giving snapshot isolation and read-your-own-writes;
//! * a per-key write lock that detects write conflicts between concurrent
//!   transactions;
//! * an append-only write-ahead log flushed on commit (crash safety) and a
//!   periodically compacted snapshot (clean reopen / disk compactness).

pub mod codec;

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{
    EdgeFilter, EdgeIter, EdgeRecord, IndexHandle, IndexKey, StorageEngine, VertexFilter, VertexIter,
    VertexRecord,
};
use metagdb_core::l3_txn::TxnContext;
use metagdb_core::{Error, Result};

use crate::schema::DynamicSchema;
use codec::{Snapshot, WalOp};

const SNAPSHOT_FILE: &str = "snapshot.bin";
const WAL_FILE: &str = "wal.log";

#[derive(Debug, Clone)]
struct Version<T> {
    rec: T,
    begin: u64,
    end: u64,
    creator: u64,
    tombstone: bool,
}

impl<T> Version<T> {
    fn new(rec: T, creator: u64) -> Self {
        Self {
            rec,
            begin: 0,
            end: 0,
            creator,
            tombstone: false,
        }
    }

}

impl Version<VertexRecord> {
    /// A tombstone created by `creator`; visible to that transaction so its own
    /// delete hides the record immediately.
    fn dead(creator: u64) -> Self {
        Self {
            rec: VertexRecord {
                id: VertexId(0),
                labels: Vec::new(),
                properties: PropertyMap::new(),
            },
            begin: 0,
            end: 0,
            creator,
            tombstone: true,
        }
    }
}

impl Version<EdgeRecord> {
    fn dead(creator: u64) -> Self {
        Self {
            rec: EdgeRecord {
                id: EdgeId(0),
                src: VertexId(0),
                dst: VertexId(0),
                etype: EdgeTypeId(0),
                properties: PropertyMap::new(),
            },
            begin: 0,
            end: 0,
            creator,
            tombstone: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    Vertex(u64),
    Edge(u64),
}

struct PendingOp {
    key: Key,
    index: usize,
    prev: Option<usize>,
    /// `Some((src, dst))` when this operation inserted a new edge, so abort can
    /// undo the adjacency entries.
    edge_insert: Option<(VertexId, VertexId)>,
}

struct Inner {
    vertices: HashMap<VertexId, Vec<Version<VertexRecord>>>,
    edges: HashMap<EdgeId, Vec<Version<EdgeRecord>>>,
    out_adj: HashMap<VertexId, Vec<EdgeId>>,
    in_adj: HashMap<VertexId, Vec<EdgeId>>,
    next_vid: u64,
    next_eid: u64,
    next_ts: u64,
    next_txn: u64,
    write_locks: HashMap<Key, u64>,
    pending: HashMap<u64, Vec<PendingOp>>,
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            out_adj: HashMap::new(),
            in_adj: HashMap::new(),
            next_vid: 1,
            next_eid: 1,
            next_ts: 1,
            next_txn: 1,
            write_locks: HashMap::new(),
            pending: HashMap::new(),
        }
    }
}

/// Write-ahead log appender.
struct WalWriter {
    file: File,
}

impl WalWriter {
    fn open(path: &Path) -> io::Result<Self> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)?;
        if file.metadata()?.len() == 0 {
            file.write_all(codec::WAL_MAGIC)?;
            file.sync_data()?;
        }
        Ok(Self { file })
    }

    fn append(&mut self, ops: &[WalOp]) -> io::Result<()> {
        let start = self.file.metadata()?.len();
        let mut buffer = Vec::new();
        for op in ops {
            let payload = codec::encode_wal_op(op);
            codec::put_u32(&mut buffer, payload.len() as u32);
            codec::put_u32(&mut buffer, codec::checksum(&payload));
            buffer.extend_from_slice(&payload);
        }
        if let Err(error) = self
            .file
            .write_all(&buffer)
            .and_then(|_| self.file.flush())
            .and_then(|_| self.file.sync_data())
        {
            let _ = self.file.set_len(start);
            return Err(error);
        }
        Ok(())
    }

    fn truncate(&mut self) -> io::Result<()> {
        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(codec::WAL_MAGIC)?;
        self.file.sync_data()?;
        Ok(())
    }
}

fn read_wal(path: &Path) -> Vec<WalOp> {
    let data = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(_) => return Vec::new(),
    };
    if data.len() < 8 || &data[..8] != codec::WAL_MAGIC {
        return Vec::new();
    }
    let mut ops = Vec::new();
    let mut pos = 8usize;
    while pos + 8 <= data.len() {
        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&data[pos..pos + 4]);
        let len = u32::from_le_bytes(len_bytes) as usize;
        let mut ck_bytes = [0u8; 4];
        ck_bytes.copy_from_slice(&data[pos + 4..pos + 8]);
        let expected = u32::from_le_bytes(ck_bytes);
        if pos + 8 + len > data.len() {
            break;
        }
        let payload = &data[pos + 8..pos + 8 + len];
        if codec::checksum(payload) != expected {
            break;
        }
        match codec::decode_wal_op(payload) {
            Ok(op) => ops.push(op),
            Err(_) => break,
        }
        pos += 8 + len;
    }
    ops
}

/// Persistent graph store implementing the L2 storage contract.
pub struct GraphStore {
    base_dir: PathBuf,
    schema: Arc<DynamicSchema>,
    inner: RwLock<Inner>,
    wal: Mutex<Option<WalWriter>>,
}

impl GraphStore {
    /// Open or create a store rooted at `base_dir`, recovering committed state.
    pub fn with_base_dir(schema: Arc<DynamicSchema>, base_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&base_dir)
            .map_err(|e| Error::Storage(format!("cannot create {}: {e}", base_dir.display())))?;

        let mut inner = Inner::default();
        if let Some(snapshot) = Self::load_snapshot(&base_dir)? {
            Self::apply_snapshot(&schema, &mut inner, snapshot);
        }
        let replayed = read_wal(&base_dir.join(WAL_FILE));
        let had_wal = !replayed.is_empty();
        for op in replayed {
            Self::apply_committed_op(&mut inner, op);
        }

        let store = Self {
            base_dir: base_dir.clone(),
            schema,
            inner: RwLock::new(inner),
            wal: Mutex::new(Some(
                WalWriter::open(&base_dir.join(WAL_FILE))
                    .map_err(|e| Error::Storage(format!("cannot open WAL: {e}")))?,
            )),
        };

        if had_wal {
            store.checkpoint()?;
        }
        Ok(store)
    }

    fn load_snapshot(base_dir: &Path) -> Result<Option<Snapshot>> {
        let path = base_dir.join(SNAPSHOT_FILE);
        if !path.exists() {
            return Ok(None);
        }
        let bytes = fs::read(&path)
            .map_err(|e| Error::Storage(format!("cannot read {}: {e}", path.display())))?;
        Ok(Some(codec::decode_snapshot(&bytes)?))
    }

    fn apply_snapshot(schema: &DynamicSchema, inner: &mut Inner, snapshot: Snapshot) {
        for (id, name) in &snapshot.labels {
            schema.restore_label(*id, name);
        }
        for (id, name) in &snapshot.edge_types {
            schema.restore_edge_type(*id, name);
        }
        for (id, name) in &snapshot.property_keys {
            schema.restore_property_key(*id, name);
        }
        inner.next_vid = snapshot.next_vid.max(1);
        inner.next_eid = snapshot.next_eid.max(1);
        inner.next_ts = snapshot.next_ts.max(1);
        inner.next_txn = snapshot.next_txn.max(1);
        for record in snapshot.vertices {
            inner
                .vertices
                .insert(record.id, vec![Version { rec: record, begin: 1, end: 0, creator: 0, tombstone: false }]);
        }
        for record in snapshot.edges {
            inner.out_adj.entry(record.src).or_default().push(record.id);
            inner.in_adj.entry(record.dst).or_default().push(record.id);
            inner
                .edges
                .insert(record.id, vec![Version { rec: record, begin: 1, end: 0, creator: 0, tombstone: false }]);
        }
    }

    fn apply_committed_op(inner: &mut Inner, op: WalOp) {
        match op {
            WalOp::InsertVertex(record, ts) => {
                let id = record.id;
                let chain = inner.vertices.entry(id).or_default();
                chain.push(Version { rec: record, begin: ts, end: 0, creator: 0, tombstone: false });
                inner.next_vid = inner.next_vid.max(id.0 + 1);
                inner.next_ts = inner.next_ts.max(ts + 1);
            }
            WalOp::UpdateVertex(record, ts) => {
                let id = record.id;
                let chain = inner.vertices.entry(id).or_default();
                if let Some(previous) = chain.last_mut() {
                    previous.end = ts;
                }
                chain.push(Version { rec: record, begin: ts, end: 0, creator: 0, tombstone: false });
                inner.next_ts = inner.next_ts.max(ts + 1);
            }
            WalOp::DeleteVertex(id, ts) => {
                let chain = inner.vertices.entry(id).or_default();
                if let Some(previous) = chain.last_mut() {
                    previous.end = ts;
                }
                chain.push(Version::<VertexRecord>::dead(0));
                if let Some(last) = chain.last_mut() {
                    last.begin = ts;
                }
                inner.next_ts = inner.next_ts.max(ts + 1);
            }
            WalOp::InsertEdge(record, ts) => {
                let id = record.id;
                inner.out_adj.entry(record.src).or_default().push(id);
                inner.in_adj.entry(record.dst).or_default().push(id);
                let chain = inner.edges.entry(id).or_default();
                chain.push(Version { rec: record, begin: ts, end: 0, creator: 0, tombstone: false });
                inner.next_eid = inner.next_eid.max(id.0 + 1);
                inner.next_ts = inner.next_ts.max(ts + 1);
            }
            WalOp::UpdateEdge(record, ts) => {
                let id = record.id;
                let chain = inner.edges.entry(id).or_default();
                if let Some(previous) = chain.last_mut() {
                    previous.end = ts;
                }
                chain.push(Version { rec: record, begin: ts, end: 0, creator: 0, tombstone: false });
                inner.next_ts = inner.next_ts.max(ts + 1);
            }
            WalOp::DeleteEdge(id, ts) => {
                let chain = inner.edges.entry(id).or_default();
                if let Some(previous) = chain.last_mut() {
                    previous.end = ts;
                }
                chain.push(Version::<EdgeRecord>::dead(0));
                if let Some(last) = chain.last_mut() {
                    last.begin = ts;
                }
                inner.next_ts = inner.next_ts.max(ts + 1);
            }
        }
    }

    /// Allocate a transaction identity.
    pub fn alloc_txn_id(&self) -> u64 {
        let mut inner = self.inner.write().unwrap();
        let id = inner.next_txn;
        inner.next_txn += 1;
        id
    }

    /// Allocate a monotonically increasing timestamp.
    pub fn alloc_ts(&self) -> u64 {
        let mut inner = self.inner.write().unwrap();
        let ts = inner.next_ts;
        inner.next_ts += 1;
        ts
    }

    /// Current timestamp (used to seed restarted counters).
    pub fn current_ts(&self) -> u64 {
        self.inner.read().unwrap().next_ts
    }

    /// Current transaction counter.
    pub fn current_txn_id(&self) -> u64 {
        self.inner.read().unwrap().next_txn
    }

    /// Persist committed state and compact the log.
    pub fn checkpoint(&self) -> Result<()> {
        let inner = self.inner.read().unwrap();
        let snapshot = self.build_snapshot(&inner);
        drop(inner);
        let path = self.base_dir.join(SNAPSHOT_FILE);
        let temp = self.base_dir.join(format!("{SNAPSHOT_FILE}.tmp"));
        let bytes = codec::encode_snapshot(&snapshot);
        {
            let mut file = File::create(&temp)
                .map_err(|e| Error::Storage(format!("cannot write snapshot: {e}")))?;
            file.write_all(&bytes)
                .and_then(|_| file.sync_all())
                .map_err(|e| Error::Storage(format!("cannot flush snapshot: {e}")))?;
        }
        fs::rename(&temp, &path)
            .map_err(|e| Error::Storage(format!("cannot install snapshot: {e}")))?;
        if let Some(wal) = self.wal.lock().unwrap().as_mut() {
            wal.truncate()
                .map_err(|e| Error::Storage(format!("cannot truncate WAL: {e}")))?;
        }
        Ok(())
    }

    fn build_snapshot(&self, inner: &Inner) -> Snapshot {
        let vertices = inner
            .vertices
            .values()
            .filter_map(|chain| live_record(chain))
            .collect();
        let edges = inner
            .edges
            .values()
            .filter_map(|chain| live_record(chain))
            .collect();
        Snapshot {
            next_vid: inner.next_vid,
            next_eid: inner.next_eid,
            next_ts: inner.next_ts,
            next_txn: inner.next_txn,
            labels: self.schema.label_table(),
            edge_types: self.schema.edge_type_table(),
            property_keys: self.schema.property_key_table(),
            vertices,
            edges,
        }
    }

    /// Commit the buffered write set of `txn_id` at `commit_ts`.
    pub fn commit_txn(&self, txn_id: u64, commit_ts: u64) -> Result<()> {
        let mut inner = self.inner.write().unwrap();
        let ops = match inner.pending.remove(&txn_id) {
            Some(ops) => ops,
            None => {
                Self::release_locks(&mut inner, txn_id);
                return Ok(());
            }
        };
        let wal_ops: Vec<WalOp> = ops.iter().map(|op| self.wal_op(&inner, op, commit_ts)).collect();

        // Durability first: a commit that returns must survive an abrupt exit.
        {
            let mut guard = self.wal.lock().unwrap();
            if let Some(wal) = guard.as_mut() {
                wal.append(&wal_ops)
                    .map_err(|e| Error::Storage(format!("WAL append failed: {e}")))?;
            }
        }
        for op in &ops {
            self.make_visible(&mut inner, op, commit_ts);
        }
        Self::release_locks(&mut inner, txn_id);
        Ok(())
    }

    fn wal_op(&self, inner: &Inner, op: &PendingOp, ts: u64) -> WalOp {
        match op.key {
            Key::Vertex(raw) => {
                let id = VertexId(raw);
                let version = &inner.vertices[&id][op.index];
                if version.tombstone {
                    WalOp::DeleteVertex(id, ts)
                } else if op.prev.is_some() {
                    WalOp::UpdateVertex(version.rec.clone(), ts)
                } else {
                    WalOp::InsertVertex(version.rec.clone(), ts)
                }
            }
            Key::Edge(raw) => {
                let id = EdgeId(raw);
                let version = &inner.edges[&id][op.index];
                if version.tombstone {
                    WalOp::DeleteEdge(id, ts)
                } else if op.prev.is_some() {
                    WalOp::UpdateEdge(version.rec.clone(), ts)
                } else {
                    WalOp::InsertEdge(version.rec.clone(), ts)
                }
            }
        }
    }

    fn make_visible(&self, inner: &mut Inner, op: &PendingOp, ts: u64) {
        match op.key {
            Key::Vertex(raw) => {
                let chain = inner.vertices.get_mut(&VertexId(raw)).unwrap();
                chain[op.index].begin = ts;
                if let Some(prev) = op.prev {
                    chain[prev].end = ts;
                }
            }
            Key::Edge(raw) => {
                let chain = inner.edges.get_mut(&EdgeId(raw)).unwrap();
                chain[op.index].begin = ts;
                if let Some(prev) = op.prev {
                    chain[prev].end = ts;
                }
            }
        }
    }

    fn release_locks(inner: &mut Inner, txn_id: u64) {
        inner.write_locks.retain(|_, owner| *owner != txn_id);
    }

    /// Discard every buffered write of `txn_id`.
    pub fn abort_txn(&self, txn_id: u64) -> Result<()> {
        let mut inner = self.inner.write().unwrap();
        if let Some(ops) = inner.pending.remove(&txn_id) {
            for op in ops.into_iter().rev() {
                match op.key {
                    Key::Vertex(raw) => {
                        if let Some(chain) = inner.vertices.get_mut(&VertexId(raw)) {
                            if chain.len() == op.index + 1 {
                                chain.pop();
                            }
                            if chain.is_empty() {
                                inner.vertices.remove(&VertexId(raw));
                            }
                        }
                    }
                    Key::Edge(raw) => {
                        if let Some(chain) = inner.edges.get_mut(&EdgeId(raw)) {
                            if chain.len() == op.index + 1 {
                                chain.pop();
                            }
                            if chain.is_empty() {
                                inner.edges.remove(&EdgeId(raw));
                            }
                        }
                        if let Some((src, dst)) = op.edge_insert {
                            if let Some(list) = inner.out_adj.get_mut(&src) {
                                list.retain(|id| id.0 != raw);
                            }
                            if let Some(list) = inner.in_adj.get_mut(&dst) {
                                list.retain(|id| id.0 != raw);
                            }
                        }
                    }
                }
            }
        }
        Self::release_locks(&mut inner, txn_id);
        Ok(())
    }

    fn acquire_lock(inner: &mut Inner, key: Key, txn_id: u64) -> Result<()> {
        match inner.write_locks.get(&key) {
            Some(owner) if *owner != txn_id => Err(Error::Transaction(format!(
                "write conflict on {:?}: held by transaction {}",
                key, owner
            ))),
            Some(_) => Ok(()),
            None => {
                inner.write_locks.insert(key, txn_id);
                Ok(())
            }
        }
    }

    fn is_visible_vertex(version: &Version<VertexRecord>, ctx: &TxnContext) -> bool {
        if version.begin == 0 {
            return version.creator == ctx.txn_id;
        }
        version.begin <= ctx.start_ts && (version.end == 0 || version.end > ctx.start_ts)
    }

    fn is_visible_edge(version: &Version<EdgeRecord>, ctx: &TxnContext) -> bool {
        if version.begin == 0 {
            return version.creator == ctx.txn_id;
        }
        version.begin <= ctx.start_ts && (version.end == 0 || version.end > ctx.start_ts)
    }

    /// Look up the newest version visible to `ctx`, including a tombstone.
    fn visible_vertex<'a>(
        inner: &'a Inner,
        vid: VertexId,
        ctx: &TxnContext,
    ) -> Option<&'a Version<VertexRecord>> {
        let chain = inner.vertices.get(&vid)?;
        chain.iter().rev().find(|version| Self::is_visible_vertex(version, ctx))
    }

    fn visible_edge<'a>(
        inner: &'a Inner,
        eid: EdgeId,
        ctx: &TxnContext,
    ) -> Option<&'a Version<EdgeRecord>> {
        let chain = inner.edges.get(&eid)?;
        chain.iter().rev().find(|version| Self::is_visible_edge(version, ctx))
    }

    fn read_vertex(inner: &Inner, vid: VertexId, ctx: &TxnContext) -> Option<VertexRecord> {
        let version = Self::visible_vertex(inner, vid, ctx)?;
        if version.tombstone {
            None
        } else {
            Some(version.rec.clone())
        }
    }

    fn read_edge(inner: &Inner, eid: EdgeId, ctx: &TxnContext) -> Option<EdgeRecord> {
        let version = Self::visible_edge(inner, eid, ctx)?;
        if version.tombstone {
            None
        } else {
            Some(version.rec.clone())
        }
    }

    fn adjacency_edges(
        inner: &Inner,
        ids: &[EdgeId],
        filter: &EdgeFilter,
        ctx: &TxnContext,
    ) -> Vec<EdgeRecord> {
        let mut out = Vec::new();
        for eid in ids {
            if let Some(record) = Self::read_edge(inner, *eid, ctx) {
                if let Some(types) = &filter.types {
                    if !types.contains(&record.etype) {
                        continue;
                    }
                }
                out.push(record);
            }
        }
        out
    }

    /// Bulk import a `.graph` file: zero-based external ids map to `id + 1`.
    pub fn bulk_load_from_graph(
        schema: Arc<DynamicSchema>,
        base_dir: PathBuf,
        graph_path: &Path,
    ) -> Result<Self> {
        let text = fs::read_to_string(graph_path)
            .map_err(|e| Error::Storage(format!("cannot read {}: {e}", graph_path.display())))?;

        let node_label = schema.intern_label("Node");
        let edge_type = schema.intern_edge_type("EDGE");
        let label_key = schema.intern_property_key("label");
        let community_key = schema.intern_property_key("community");

        let mut vertices: Vec<VertexRecord> = Vec::new();
        let mut edges: Vec<EdgeRecord> = Vec::new();
        let mut max_vid = 0u64;
        for line in text.lines() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("v") => {
                    let vid: u64 = parts
                        .next()
                        .and_then(|token| token.parse().ok())
                        .ok_or_else(|| Error::Storage("bad vertex line".into()))?;
                    let label = parts.next().unwrap_or("0");
                    let community = parts.next().unwrap_or("0");
                    let mut properties = PropertyMap::new();
                    match label.parse::<i64>() {
                        Ok(value) => properties.insert(label_key, PropertyValue::Int(value)),
                        Err(_) => properties.insert(label_key, PropertyValue::Text(label.into())),
                    };
                    if let Ok(value) = community.parse::<i64>() {
                        properties.insert(community_key, PropertyValue::Int(value));
                    }
                    max_vid = max_vid.max(vid + 1);
                    vertices.push(VertexRecord {
                        id: VertexId(vid + 1),
                        labels: vec![node_label],
                        properties,
                    });
                }
                Some("e") => {
                    let src: u64 = parts
                        .next()
                        .and_then(|token| token.parse().ok())
                        .ok_or_else(|| Error::Storage("bad edge source".into()))?;
                    let dst: u64 = parts
                        .next()
                        .and_then(|token| token.parse().ok())
                        .ok_or_else(|| Error::Storage("bad edge dest".into()))?;
                    max_vid = max_vid.max(src + 1).max(dst + 1);
                    edges.push(EdgeRecord {
                        id: EdgeId(edges.len() as u64 + 1),
                        src: VertexId(src + 1),
                        dst: VertexId(dst + 1),
                        etype: edge_type,
                        properties: PropertyMap::new(),
                    });
                }
                _ => {}
            }
        }

        let mut inner = Inner::default();
        for record in vertices {
            inner.next_vid = inner.next_vid.max(record.id.0 + 1);
            inner.vertices.insert(
                record.id,
                vec![Version { rec: record, begin: 1, end: 0, creator: 0, tombstone: false }],
            );
        }
        let mut next_eid = 1u64;
        for record in edges {
            next_eid = next_eid.max(record.id.0 + 1);
            inner.out_adj.entry(record.src).or_default().push(record.id);
            inner.in_adj.entry(record.dst).or_default().push(record.id);
            inner.edges.insert(
                record.id,
                vec![Version { rec: record, begin: 1, end: 0, creator: 0, tombstone: false }],
            );
        }
        inner.next_eid = next_eid;
        inner.next_vid = inner.next_vid.max(max_vid + 1);

        fs::create_dir_all(&base_dir)
            .map_err(|e| Error::Storage(format!("cannot create {}: {e}", base_dir.display())))?;
        let store = Self {
            base_dir: base_dir.clone(),
            schema,
            inner: RwLock::new(inner),
            wal: Mutex::new(Some(
                WalWriter::open(&base_dir.join(WAL_FILE))
                    .map_err(|e| Error::Storage(format!("cannot open WAL: {e}")))?,
            )),
        };
        store.checkpoint()?;
        Ok(store)
    }
}

fn live_record<T: Clone>(chain: &[Version<T>]) -> Option<T> {
    chain
        .iter()
        .rev()
        .find(|version| version.begin != 0 && version.end == 0 && !version.tombstone)
        .map(|version| version.rec.clone())
}

impl Drop for GraphStore {
    fn drop(&mut self) {
        let _ = self.checkpoint();
    }
}

impl StorageEngine for GraphStore {
    fn get_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<Option<VertexRecord>> {
        let inner = self.inner.read().unwrap();
        Ok(Self::read_vertex(&inner, vid, ctx))
    }

    fn scan_vertices(&self, filter: VertexFilter, ctx: &TxnContext) -> Result<VertexIter> {
        let inner = self.inner.read().unwrap();
        let mut rows = Vec::new();
        for chain in inner.vertices.values() {
            let version = match chain.iter().rev().find(|v| Self::is_visible_vertex(v, ctx)) {
                Some(version) => version,
                None => continue,
            };
            if version.tombstone {
                continue;
            }
            if let Some(labels) = &filter.labels {
                if !version.rec.labels.iter().any(|label| labels.contains(label)) {
                    continue;
                }
            }
            rows.push(version.rec.clone());
        }
        // Deterministic iteration order: ascending vertex id (insertion order).
        rows.sort_by_key(|record| record.id.0);
        Ok(Box::new(rows.into_iter()))
    }

    fn get_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<Option<EdgeRecord>> {
        let inner = self.inner.read().unwrap();
        Ok(Self::read_edge(&inner, eid, ctx))
    }

    fn out_edges(&self, vid: VertexId, filter: EdgeFilter, ctx: &TxnContext) -> Result<EdgeIter> {
        let inner = self.inner.read().unwrap();
        let empty = Vec::new();
        let ids = inner.out_adj.get(&vid).unwrap_or(&empty);
        Ok(Box::new(Self::adjacency_edges(&inner, ids, &filter, ctx).into_iter()))
    }

    fn in_edges(&self, vid: VertexId, filter: EdgeFilter, ctx: &TxnContext) -> Result<EdgeIter> {
        let inner = self.inner.read().unwrap();
        let empty = Vec::new();
        let ids = inner.in_adj.get(&vid).unwrap_or(&empty);
        Ok(Box::new(Self::adjacency_edges(&inner, ids, &filter, ctx).into_iter()))
    }

    fn index_lookup(
        &self,
        _idx: IndexHandle,
        _key: &IndexKey,
        _ctx: &TxnContext,
    ) -> Result<VertexIter> {
        Err(Error::Unsupported(
            "index_lookup is outside the selected schemaless point/adjacency profile".into(),
        ))
    }

    fn insert_vertex(&self, record: VertexRecord, ctx: &TxnContext) -> Result<VertexId> {
        if ctx.read_only {
            return Err(Error::Transaction("read-only transaction cannot mutate".into()));
        }
        let mut inner = self.inner.write().unwrap();
        let id = if record.id.0 == 0 {
            let id = inner.next_vid;
            inner.next_vid += 1;
            id
        } else {
            inner.next_vid = inner.next_vid.max(record.id.0 + 1);
            record.id.0
        };
        let vid = VertexId(id);
        Self::acquire_lock(&mut inner, Key::Vertex(id), ctx.txn_id)?;
        if let Some(version) = inner.vertices.get(&vid).and_then(|chain| chain.last()) {
            if Self::is_visible_vertex(version, ctx) && !version.tombstone {
                return Err(Error::Transaction(format!(
                    "vertex {} already exists",
                    id
                )));
            }
        }
        let mut record = record;
        record.id = vid;
        let chain = inner.vertices.entry(vid).or_default();
        let index = chain.len();
        chain.push(Version::new(record, ctx.txn_id));
        let pending = inner.pending.entry(ctx.txn_id).or_default();
        pending.push(PendingOp {
            key: Key::Vertex(id),
            index,
            prev: if index == 0 { None } else { Some(index - 1) },
            edge_insert: None,
        });
        Ok(vid)
    }

    fn insert_edge(&self, record: EdgeRecord, ctx: &TxnContext) -> Result<EdgeId> {
        if ctx.read_only {
            return Err(Error::Transaction("read-only transaction cannot mutate".into()));
        }
        let mut inner = self.inner.write().unwrap();
        if Self::read_vertex(&inner, record.src, ctx).is_none()
            || Self::read_vertex(&inner, record.dst, ctx).is_none()
        {
            return Err(Error::Transaction(
                "edge endpoint does not reference a visible vertex".into(),
            ));
        }
        let id = if record.id.0 == 0 {
            let id = inner.next_eid;
            inner.next_eid += 1;
            id
        } else {
            inner.next_eid = inner.next_eid.max(record.id.0 + 1);
            record.id.0
        };
        let eid = EdgeId(id);
        Self::acquire_lock(&mut inner, Key::Edge(id), ctx.txn_id)?;
        if let Some(version) = inner.edges.get(&eid).and_then(|chain| chain.last()) {
            if Self::is_visible_edge(version, ctx) && !version.tombstone {
                return Err(Error::Transaction(format!("edge {} already exists", id)));
            }
        }
        let mut record = record;
        record.id = eid;
        let src = record.src;
        let dst = record.dst;
        inner.out_adj.entry(src).or_default().push(eid);
        inner.in_adj.entry(dst).or_default().push(eid);
        let chain = inner.edges.entry(eid).or_default();
        let index = chain.len();
        chain.push(Version::new(record, ctx.txn_id));
        let pending = inner.pending.entry(ctx.txn_id).or_default();
        pending.push(PendingOp {
            key: Key::Edge(id),
            index,
            prev: if index == 0 { None } else { Some(index - 1) },
            edge_insert: Some((src, dst)),
        });
        Ok(eid)
    }

    fn update_vertex(&self, record: VertexRecord, ctx: &TxnContext) -> Result<()> {
        if ctx.read_only {
            return Err(Error::Transaction("read-only transaction cannot mutate".into()));
        }
        let id = record.id;
        let mut inner = self.inner.write().unwrap();
        Self::acquire_lock(&mut inner, Key::Vertex(id.0), ctx.txn_id)?;
        if Self::read_vertex(&inner, id, ctx).is_none() {
            return Err(Error::NotFound(format!("vertex {}", id.0)));
        }
        let chain = inner.vertices.get_mut(&id).unwrap();
        let index = chain.len();
        chain.push(Version::new(record, ctx.txn_id));
        let pending = inner.pending.entry(ctx.txn_id).or_default();
        pending.push(PendingOp {
            key: Key::Vertex(id.0),
            index,
            prev: Some(index - 1),
            edge_insert: None,
        });
        Ok(())
    }

    fn delete_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<()> {
        if ctx.read_only {
            return Err(Error::Transaction("read-only transaction cannot mutate".into()));
        }
        let mut inner = self.inner.write().unwrap();
        if Self::read_vertex(&inner, vid, ctx).is_none() {
            return Err(Error::NotFound(format!("vertex {}", vid.0)));
        }
        let has_edges = {
            let out = inner.out_adj.get(&vid).cloned().unwrap_or_default();
            let inc = inner.in_adj.get(&vid).cloned().unwrap_or_default();
            out.iter()
                .chain(inc.iter())
                .any(|eid| Self::read_edge(&inner, *eid, ctx).is_some())
        };
        if has_edges {
            return Err(Error::Transaction(format!(
                "vertex {} still has incident edges; use DETACH DELETE",
                vid.0
            )));
        }
        Self::acquire_lock(&mut inner, Key::Vertex(vid.0), ctx.txn_id)?;
        let chain = inner.vertices.get_mut(&vid).unwrap();
        let index = chain.len();
        chain.push(Version::<VertexRecord>::dead(ctx.txn_id));
        let pending = inner.pending.entry(ctx.txn_id).or_default();
        pending.push(PendingOp {
            key: Key::Vertex(vid.0),
            index,
            prev: Some(index - 1),
            edge_insert: None,
        });
        Ok(())
    }

    fn update_edge(&self, record: EdgeRecord, ctx: &TxnContext) -> Result<()> {
        if ctx.read_only {
            return Err(Error::Transaction("read-only transaction cannot mutate".into()));
        }
        let mut inner = self.inner.write().unwrap();
        let existing = Self::read_edge(&inner, record.id, ctx)
            .ok_or_else(|| Error::NotFound(format!("edge {}", record.id.0)))?;
        if existing.src != record.src || existing.dst != record.dst || existing.etype != record.etype {
            return Err(Error::Transaction(
                "update_edge cannot change endpoints or type".into(),
            ));
        }
        let id = record.id;
        Self::acquire_lock(&mut inner, Key::Edge(id.0), ctx.txn_id)?;
        let chain = inner.edges.get_mut(&id).unwrap();
        let index = chain.len();
        chain.push(Version::new(record, ctx.txn_id));
        let pending = inner.pending.entry(ctx.txn_id).or_default();
        pending.push(PendingOp {
            key: Key::Edge(id.0),
            index,
            prev: Some(index - 1),
            edge_insert: None,
        });
        Ok(())
    }

    fn delete_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<()> {
        if ctx.read_only {
            return Err(Error::Transaction("read-only transaction cannot mutate".into()));
        }
        let mut inner = self.inner.write().unwrap();
        if Self::read_edge(&inner, eid, ctx).is_none() {
            return Err(Error::NotFound(format!("edge {}", eid.0)));
        }
        Self::acquire_lock(&mut inner, Key::Edge(eid.0), ctx.txn_id)?;
        let chain = inner.edges.get_mut(&eid).unwrap();
        let index = chain.len();
        chain.push(Version::<EdgeRecord>::dead(ctx.txn_id));
        let pending = inner.pending.entry(ctx.txn_id).or_default();
        pending.push(PendingOp {
            key: Key::Edge(eid.0),
            index,
            prev: Some(index - 1),
            edge_insert: None,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metagdb_core::common::ids::{LabelId, PropertyKeyId};
    use metagdb_core::l3_txn::IsolationLevel;

    fn ctx(txn_id: u64, start_ts: u64) -> TxnContext {
        TxnContext {
            txn_id,
            isolation: IsolationLevel::SnapshotIsolation,
            read_only: false,
            start_ts,
        }
    }

    #[test]
    fn mvcc_point_read_and_abort() {
        let dir = std::env::temp_dir().join(format!("mgdb_store_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let schema = Arc::new(DynamicSchema::schemaless());
        let store = GraphStore::with_base_dir(schema, dir.clone()).unwrap();

        let writer = ctx(1, 1);
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(1), PropertyValue::Int(1));
        let id = store
            .insert_vertex(
                VertexRecord {
                    id: VertexId(0),
                    labels: vec![LabelId(1)],
                    properties,
                },
                &writer,
            )
            .unwrap();

        // Not visible to a concurrent transaction before commit.
        let concurrent = ctx(2, 2);
        assert!(store.get_vertex(id, &concurrent).unwrap().is_none());
        // Visible to its own transaction.
        assert!(store.get_vertex(id, &writer).unwrap().is_some());

        store.commit_txn(1, 3).unwrap();
        // A transaction that starts after the commit observes the new version.
        let after = ctx(3, 4);
        assert!(store.get_vertex(id, &after).unwrap().is_some());
        // The old snapshot still does not see it (snapshot isolation).
        assert!(store.get_vertex(id, &concurrent).unwrap().is_none());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn write_conflict_is_detected() {
        let dir = std::env::temp_dir().join(format!("mgdb_store_conflict_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let schema = Arc::new(DynamicSchema::schemaless());
        let store = GraphStore::with_base_dir(schema, dir.clone()).unwrap();

        let seed = ctx(1, 1);
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(1), PropertyValue::Int(1));
        let id = store
            .insert_vertex(
                VertexRecord {
                    id: VertexId(0),
                    labels: vec![LabelId(1)],
                    properties: properties.clone(),
                },
                &seed,
            )
            .unwrap();
        store.commit_txn(1, 2).unwrap();

        let t1 = ctx(2, 3);
        let t2 = ctx(3, 4);
        let mut first = properties.clone();
        first.insert(PropertyKeyId(1), PropertyValue::Int(10));
        store.update_vertex(VertexRecord { id, labels: vec![LabelId(1)], properties: first.clone() }, &t1).unwrap();
        let second = store.update_vertex(VertexRecord { id, labels: vec![LabelId(1)], properties: first }, &t2);
        assert!(second.is_err(), "second writer should conflict");

        let _ = fs::remove_dir_all(&dir);
    }
}
