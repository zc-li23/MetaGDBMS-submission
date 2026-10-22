//! Persistent append-only storage with logical-ID redirection.
//!
//! Physical layout (all under the candidate's base directory):
//!
//! * `records.seg` — the append-only physical record segment.  Committed vertex
//!   and edge versions are only ever appended; a previous record is never
//!   rewritten, which is what lets older snapshots keep reading it.
//! * `wal.log` — the recovery write-ahead log, distinct from the record
//!   segment.  It holds commit markers plus the encoded records needed to
//!   re-materialise a segment whose trailing writes were lost.
//!
//! Logical identities (stable `VertexId`/`EdgeId`) are resolved through a
//! redirection index that maps each logical id to the offset of its newest
//! physical record; older versions are reached by following `prev_offset`
//! links.  The index is rebuilt from the segment at open, so it survives
//! checkpoints and process reopen.

pub mod record;
pub mod wal;

use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::{
    EdgeFilter, EdgeIter, EdgeRecord, IndexHandle, IndexKey, StorageEngine, VertexFilter,
    VertexIter, VertexRecord,
};
use metagdb_core::l3_txn::{IsolationLevel, TxnContext};
use metagdb_core::{Error, Result};

use record::{Namespace, PhysicalRecord, RecordBody, RecordKind};
use wal::{Wal, WalEntry};

const SEGMENT_FILE: &str = "records.seg";
const WAL_FILE: &str = "wal.log";

/// One buffered mutation inside an uncommitted transaction.
#[derive(Debug, Clone)]
enum PendingWrite {
    VertexPut { id: u64, record: VertexRecord },
    VertexDelete { id: u64 },
    EdgePut { id: u64, record: EdgeRecord },
    EdgeDelete { id: u64 },
}

impl PendingWrite {
    /// `(namespace, logical id)`: 0 = vertex, 1 = edge.
    fn key(&self) -> (u8, u64) {
        match self {
            PendingWrite::VertexPut { id, .. } => (0, *id),
            PendingWrite::VertexDelete { id } => (0, *id),
            PendingWrite::EdgePut { id, .. } => (1, *id),
            PendingWrite::EdgeDelete { id } => (1, *id),
        }
    }
}

/// Uncommitted state of one transaction.
#[derive(Debug, Default)]
struct PendingTxn {
    start_ts: u64,
    read_only: bool,
    writes: Vec<PendingWrite>,
    index: HashMap<(u8, u64), usize>,
}

impl PendingTxn {
    fn push(&mut self, write: PendingWrite) {
        let key = write.key();
        match self.index.get(&key) {
            Some(position) => self.writes[*position] = write,
            None => {
                self.index.insert(key, self.writes.len());
                self.writes.push(write);
            }
        }
    }

    fn get(&self, namespace: u8, id: u64) -> Option<&PendingWrite> {
        self.index.get(&(namespace, id)).map(|p| &self.writes[*p])
    }
}

/// Committed, in-memory materialisation of the physical segment.
struct Engine {
    records: Vec<PhysicalRecord>,
    vertex_head: HashMap<u64, u64>,
    edge_head: HashMap<u64, u64>,
    vertex_order: Vec<u64>,
    edge_order: Vec<u64>,
    vertex_member: HashSet<u64>,
    edge_member: HashSet<u64>,
    out_adj: HashMap<u64, Vec<u64>>,
    in_adj: HashMap<u64, Vec<u64>>,
    next_vid: u64,
    next_eid: u64,
    commit_ts: u64,
}

impl Engine {
    fn new() -> Self {
        Self {
            records: Vec::new(),
            vertex_head: HashMap::new(),
            edge_head: HashMap::new(),
            vertex_order: Vec::new(),
            edge_order: Vec::new(),
            vertex_member: HashSet::new(),
            edge_member: HashSet::new(),
            out_adj: HashMap::new(),
            in_adj: HashMap::new(),
            next_vid: 1,
            next_eid: 1,
            commit_ts: 0,
        }
    }

    fn record(&self, offset: u64) -> Option<&PhysicalRecord> {
        if offset == 0 {
            return None;
        }
        self.records.get((offset - 1) as usize)
    }

    /// Newest committed version of a logical element visible at `snapshot_ts`.
    /// Returns the physical record, which may be a tombstone.
    fn resolve(
        &self,
        namespace: Namespace,
        logical_id: u64,
        snapshot_ts: u64,
    ) -> Option<&PhysicalRecord> {
        let head = match namespace {
            Namespace::Vertex => self.vertex_head.get(&logical_id),
            Namespace::Edge => self.edge_head.get(&logical_id),
        }?;
        let mut offset = *head;
        while offset != 0 {
            let record = self.record(offset)?;
            if record.begin_ts != 0 && record.begin_ts <= snapshot_ts {
                return Some(record);
            }
            offset = record.prev_offset;
        }
        None
    }

    /// Store one physical record at its segment position.
    ///
    /// Storing is deliberately separate from linking: the physical record
    /// numbering must follow the segment itself, including records written by a
    /// transaction that never committed, so that a later append continues from
    /// the true end of the segment.  Records must be stored in ascending offset
    /// order.
    fn store(&mut self, record: PhysicalRecord) {
        debug_assert_eq!(record.offset, self.records.len() as u64 + 1);
        self.records.push(record);
    }

    /// Link a committed record into the redirection index, the element order
    /// and the adjacency maps.
    ///
    /// Records must be linked in non-decreasing `begin_ts` order so that a
    /// version's predecessor is already linked when it is superseded.  The
    /// record must already be present in `records`.
    fn link(&mut self, record: &PhysicalRecord) {
        let offset = record.offset;
        match record.kind {
            RecordKind::Vertex | RecordKind::Edge => {
                let namespace = record.kind.namespace();
                let head = match namespace {
                    Namespace::Vertex => &mut self.vertex_head,
                    Namespace::Edge => &mut self.edge_head,
                };
                head.insert(record.logical_id, offset);
                match namespace {
                    Namespace::Vertex => {
                        if self.vertex_member.insert(record.logical_id) {
                            self.vertex_order.push(record.logical_id);
                        }
                    }
                    Namespace::Edge => {
                        if self.edge_member.insert(record.logical_id) {
                            self.edge_order.push(record.logical_id);
                        }
                        if let Some(RecordBody::Edge(edge)) = &record.body {
                            push_unique(self.out_adj.entry(edge.src.0).or_default(), record.logical_id);
                            push_unique(self.in_adj.entry(edge.dst.0).or_default(), record.logical_id);
                        }
                    }
                }
            }
            RecordKind::VertexTombstone | RecordKind::EdgeTombstone => {
                let namespace = record.kind.namespace();
                if namespace == Namespace::Edge {
                    if let Some(previous) = self.edge_head.get(&record.logical_id).copied() {
                        self.detach_edge(previous, record.logical_id);
                    }
                }
                match namespace {
                    Namespace::Vertex => {
                        self.vertex_head.insert(record.logical_id, offset);
                    }
                    Namespace::Edge => {
                        self.edge_head.insert(record.logical_id, offset);
                    }
                }
            }
        }
    }

    fn detach_edge(&mut self, offset: u64, edge_id: u64) {
        if let Some(record) = self.records.get((offset - 1) as usize) {
            if let Some(RecordBody::Edge(edge)) = &record.body {
                if let Some(list) = self.out_adj.get_mut(&edge.src.0) {
                    list.retain(|candidate| *candidate != edge_id);
                }
                if let Some(list) = self.in_adj.get_mut(&edge.dst.0) {
                    list.retain(|candidate| *candidate != edge_id);
                }
            }
        }
    }
}

fn push_unique(list: &mut Vec<u64>, value: u64) {
    if !list.contains(&value) {
        list.push(value);
    }
}

/// Append handle for the record segment.
///
/// Like the WAL, the segment keeps a reservation of already-allocated blocks and
/// writes each commit's records into it with `pwrite`, so a commit never changes
/// the segment's size or block map.  That matters beyond this file: an ext4
/// journal commit is triggered by *any* pending metadata on the filesystem, so a
/// segment that grew on every commit would force the WAL's `fdatasync` to commit
/// a metadata transaction even though the WAL itself only appended data.
struct SegmentWriter {
    file: File,
    logical_len: u64,
    reserved_len: u64,
}

impl SegmentWriter {
    fn open(path: &Path, logical_len: u64, file_len: u64) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)
            .map_err(io_error)?;
        Ok(Self {
            file,
            logical_len,
            reserved_len: file_len.max(logical_len),
        })
    }

    fn append(&mut self, records: &[PhysicalRecord], durable: bool) -> Result<()> {
        let mut buffer = Vec::new();
        for record in records {
            buffer.extend_from_slice(&record.encode());
        }
        self.reserve(buffer.len() as u64)?;
        self.file
            .write_all_at(&buffer, self.logical_len)
            .map_err(io_error)?;
        self.logical_len += buffer.len() as u64;
        if durable {
            self.file.sync_data().map_err(io_error)?;
        }
        Ok(())
    }

    fn reserve(&mut self, need: u64) -> Result<()> {
        let required = self.logical_len + need;
        if required <= self.reserved_len {
            return Ok(());
        }
        let target = self.growth_target(required);
        let zeros = vec![0u8; SEGMENT_ZERO_CHUNK.min((target - self.reserved_len) as usize)];
        let mut offset = self.reserved_len;
        while offset < target {
            let len = zeros.len().min((target - offset) as usize);
            self.file
                .write_all_at(&zeros[..len], offset)
                .map_err(io_error)?;
            offset += len as u64;
        }
        self.reserved_len = target;
        self.file.sync_data().map_err(io_error)?;
        Ok(())
    }

    /// See `Wal::growth_target`; the same policy applies to the segment.
    fn growth_target(&self, required: u64) -> u64 {
        required + (required / 16).max(SEGMENT_MIN_HEADROOM)
    }
}

const SEGMENT_MIN_HEADROOM: u64 = 64 * 1024;
const SEGMENT_ZERO_CHUNK: usize = 1024 * 1024;

/// Durable storage engine used by the candidate.
pub struct PersistentStorage {
    schema: Arc<dyn GraphSchema>,
    base_dir: PathBuf,
    engine: Mutex<Engine>,
    pending: Mutex<HashMap<u64, PendingTxn>>,
    wal: Mutex<Wal>,
    /// Long-lived append handle for the record segment.  Re-opening the file on
    /// every commit costs an `open`/`close` pair per transaction, which is a
    /// measurable share of a small commit.
    segment: Mutex<SegmentWriter>,
}

impl PersistentStorage {
    /// Open (or create) a database rooted at `base_dir`.
    pub fn open(schema: Arc<dyn GraphSchema>, base_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&base_dir).map_err(io_error)?;
        let segment_path = base_dir.join(SEGMENT_FILE);
        let wal_path = base_dir.join(WAL_FILE);

        let (records, disk_len, clean_tail) = read_segment(&segment_path)?;
        let file_len = std::fs::metadata(&segment_path).map(|m| m.len()).unwrap_or(0);
        // A trailing partial write is by definition not part of any committed
        // transaction, so clear it before anything else: without this a later
        // append would start inside the torn bytes and every record after it
        // would be unreadable.  A zeroed region is an intact reservation and is
        // left alone so its blocks stay allocated.
        let mut segment = SegmentWriter::open(&segment_path, disk_len, file_len)?;
        if !clean_tail {
            zero_range(&mut segment.file, disk_len, file_len)?;
        }
        let entries = Wal::read_all(&wal_path)?;
        let mut engine = Engine::new();
        replay(&mut engine, records, entries, &mut segment)?;

        let wal = Wal::open(&wal_path)?;
        Ok(Self {
            schema,
            base_dir,
            engine: Mutex::new(engine),
            pending: Mutex::new(HashMap::new()),
            wal: Mutex::new(wal),
            segment: Mutex::new(segment),
        })
    }

    /// Register a freshly begun transaction so that its writes stay private.
    pub fn begin_txn(&self, txn_id: u64, start_ts: u64, read_only: bool) {
        let mut pending = self.pending.lock().unwrap();
        pending.insert(
            txn_id,
            PendingTxn {
                start_ts,
                read_only,
                writes: Vec::new(),
                index: HashMap::new(),
            },
        );
    }

    /// Discard every buffered write of `txn_id`.
    pub fn abort_txn(&self, txn_id: u64) -> Result<()> {
        let mut pending = self.pending.lock().unwrap();
        match pending.remove(&txn_id) {
            Some(_) => Ok(()),
            None => Err(Error::Transaction(format!(
                "abort of unknown or already completed transaction {txn_id}"
            ))),
        }
    }

    /// Apply the transaction's write set and make it durable.
    pub fn commit_txn(&self, txn_id: u64) -> Result<()> {
        let mut pending_guard = self.pending.lock().unwrap();
        let txn = pending_guard.remove(&txn_id).ok_or_else(|| {
            Error::Transaction(format!(
                "commit of unknown or already completed transaction {txn_id}"
            ))
        })?;

        let mut engine = self.engine.lock().unwrap();

        // Snapshot-isolation conflict detection: a write is invalid when the
        // current head version was produced after our snapshot.
        for write in &txn.writes {
            let (namespace, id) = write.key();
            let head = if namespace == 0 {
                engine.vertex_head.get(&id).copied()
            } else {
                engine.edge_head.get(&id).copied()
            };
            if let Some(offset) = head {
                if let Some(record) = engine.record(offset) {
                    if record.begin_ts > txn.start_ts {
                        return Err(Error::Transaction(format!(
                            "write conflict on {} {id}: committed at {} after snapshot {}",
                            if namespace == 0 { "vertex" } else { "edge" },
                            record.begin_ts,
                            txn.start_ts
                        )));
                    }
                }
            }
        }

        if txn.writes.is_empty() {
            return Ok(());
        }

        let commit_ts = engine.commit_ts + 1;
        let mut physical = Vec::with_capacity(txn.writes.len());
        let mut next_offset = engine.records.len() as u64 + 1;
        for write in &txn.writes {
            let (namespace, id) = write.key();
            let previous = if namespace == 0 {
                engine.vertex_head.get(&id).copied().unwrap_or(0)
            } else {
                engine.edge_head.get(&id).copied().unwrap_or(0)
            };
            let (kind, body) = match write {
                PendingWrite::VertexPut { record, .. } => {
                    (RecordKind::Vertex, Some(RecordBody::Vertex(record.clone())))
                }
                PendingWrite::EdgePut { record, .. } => {
                    (RecordKind::Edge, Some(RecordBody::Edge(record.clone())))
                }
                PendingWrite::VertexDelete { .. } => (RecordKind::VertexTombstone, None),
                PendingWrite::EdgeDelete { .. } => (RecordKind::EdgeTombstone, None),
            };
            physical.push(PhysicalRecord {
                offset: next_offset,
                kind,
                logical_id: id,
                begin_ts: commit_ts,
                prev_offset: previous,
                txn_id,
                body,
            });
            next_offset += 1;
        }

        // 1. Data first: append the immutable physical records to the segment.
        //    The segment write is not fsynced on its own; the fsynced commit
        //    marker below carries the same encoded records, so it is the single
        //    durability point and recovery can re-materialise any segment write
        //    that did not reach stable storage.
        self.append_segment(&physical)?;

        // 2. Then the commit marker, fsynced, which makes the write set durable.
        {
            let mut wal = self.wal.lock().unwrap();
            wal.append_commit(
                txn_id,
                commit_ts,
                engine.next_vid,
                engine.next_eid,
                &physical,
            )?;
        }

        // 3. Finally publish in memory: place every record at its segment
        //    position first, then link the committed versions.
        for record in &physical {
            engine.store(record.clone());
        }
        for record in &physical {
            engine.link(record);
        }
        engine.commit_ts = commit_ts;
        drop(pending_guard);
        Ok(())
    }

    /// Append immutable physical records to the segment through the long-lived
    /// handle.  Durability comes from the WAL commit marker, not from fsyncing
    /// this file on every commit.
    fn append_segment(&self, records: &[PhysicalRecord]) -> Result<()> {
        // Not synced here: the WAL commit marker is the single durability point
        // and it carries the same records, so recovery can re-materialise them.
        self.segment.lock().unwrap().append(records, false)
    }

    /// Write an explicit checkpoint marker capturing counters and segment size.
    pub fn checkpoint(&self) -> Result<()> {
        let engine = self.engine.lock().unwrap();
        let mut wal = self.wal.lock().unwrap();
        wal.append(&WalEntry::Checkpoint {
            commit_ts: engine.commit_ts,
            next_vid: engine.next_vid,
            next_eid: engine.next_eid,
            segment_len: engine.records.len() as u64,
        })
    }

    /// Snapshot timestamp used for visibility checks under this transaction.
    fn snapshot_ts(&self, ctx: &TxnContext, engine: &Engine) -> u64 {
        match ctx.isolation {
            IsolationLevel::ReadCommitted => engine.commit_ts,
            _ => ctx.start_ts,
        }
    }

    fn ensure_writable(&self, ctx: &TxnContext) -> Result<()> {
        let pending = self.pending.lock().unwrap();
        let txn = pending.get(&ctx.txn_id).ok_or_else(|| {
            Error::Transaction(format!("transaction {} is not active", ctx.txn_id))
        })?;
        if txn.read_only {
            return Err(Error::Transaction(format!(
                "read-only transaction {} cannot mutate data",
                ctx.txn_id
            )));
        }
        Ok(())
    }

    fn with_pending<R>(&self, txn_id: u64, f: impl FnOnce(&mut PendingTxn) -> R) -> Result<R> {
        let mut pending = self.pending.lock().unwrap();
        let txn = pending
            .get_mut(&txn_id)
            .ok_or_else(|| Error::Transaction(format!("transaction {txn_id} is not active")))?;
        Ok(f(txn))
    }

    fn get_vertex_inner(&self, vid: VertexId, ctx: &TxnContext) -> Result<Option<VertexRecord>> {
        {
            let pending = self.pending.lock().unwrap();
            if let Some(txn) = pending.get(&ctx.txn_id) {
                match txn.get(0, vid.0) {
                    Some(PendingWrite::VertexPut { record, .. }) => return Ok(Some(record.clone())),
                    Some(PendingWrite::VertexDelete { .. }) => return Ok(None),
                    _ => {}
                }
            }
        }
        let engine = self.engine.lock().unwrap();
        let snapshot = self.snapshot_ts(ctx, &engine);
        match engine.resolve(Namespace::Vertex, vid.0, snapshot) {
            Some(record) if !record.is_tombstone() => match &record.body {
                Some(RecordBody::Vertex(vertex)) => Ok(Some(vertex.clone())),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn get_edge_inner(&self, eid: EdgeId, ctx: &TxnContext) -> Result<Option<EdgeRecord>> {
        {
            let pending = self.pending.lock().unwrap();
            if let Some(txn) = pending.get(&ctx.txn_id) {
                match txn.get(1, eid.0) {
                    Some(PendingWrite::EdgePut { record, .. }) => return Ok(Some(record.clone())),
                    Some(PendingWrite::EdgeDelete { .. }) => return Ok(None),
                    _ => {}
                }
            }
        }
        let engine = self.engine.lock().unwrap();
        let snapshot = self.snapshot_ts(ctx, &engine);
        match engine.resolve(Namespace::Edge, eid.0, snapshot) {
            Some(record) if !record.is_tombstone() => match &record.body {
                Some(RecordBody::Edge(edge)) => Ok(Some(edge.clone())),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn edge_candidates(&self, vid: VertexId, outgoing: bool, ctx: &TxnContext) -> Vec<u64> {
        let mut candidates: Vec<u64> = Vec::new();
        {
            let engine = self.engine.lock().unwrap();
            let adjacency = if outgoing {
                engine.out_adj.get(&vid.0)
            } else {
                engine.in_adj.get(&vid.0)
            };
            if let Some(list) = adjacency {
                candidates.extend(list.iter().copied());
            }
        }
        {
            let pending = self.pending.lock().unwrap();
            if let Some(txn) = pending.get(&ctx.txn_id) {
                for write in &txn.writes {
                    match write {
                        PendingWrite::EdgePut { id, record } => {
                            let matches = if outgoing {
                                record.src == vid
                            } else {
                                record.dst == vid
                            };
                            if matches {
                                push_unique(&mut candidates, *id);
                            }
                        }
                        PendingWrite::EdgeDelete { id } => {
                            candidates.retain(|candidate| candidate != id);
                        }
                        _ => {}
                    }
                }
            }
        }
        candidates
    }

    /// Bulk import a `.graph` file, bypassing per-record WAL entries.
    pub fn bulk_load(
        schema: Arc<dyn GraphSchema>,
        base_dir: PathBuf,
        graph_path: &Path,
    ) -> Result<Self> {
        if !graph_path.exists() {
            return Err(Error::Storage(format!(
                "graph file does not exist: {}",
                graph_path.display()
            )));
        }
        std::fs::create_dir_all(&base_dir).map_err(io_error)?;
        let text = std::fs::read_to_string(graph_path).map_err(io_error)?;
        let mut lines = text.lines().filter(|line| !line.trim().is_empty());

        let meta = lines
            .next()
            .ok_or_else(|| Error::Storage("empty graph file".into()))?;
        let meta_parts: Vec<&str> = meta.split_whitespace().collect();
        if meta_parts.len() < 3 || meta_parts[0] != "t" {
            return Err(Error::Storage(
                "invalid meta line format, expected 't n m'".into(),
            ));
        }
        let vertex_count: u64 = meta_parts[1]
            .parse()
            .map_err(|_| Error::Storage("invalid vertex count".into()))?;
        let edge_count: u64 = meta_parts[2]
            .parse()
            .map_err(|_| Error::Storage("invalid edge count".into()))?;

        let mut records: Vec<PhysicalRecord> = Vec::with_capacity((vertex_count + edge_count) as usize);
        let mut max_vid = 0u64;
        for _ in 0..vertex_count {
            let line = lines
                .next()
                .ok_or_else(|| Error::Storage("insufficient vertex lines".into()))?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 || parts[0] != "v" {
                return Err(Error::Storage(format!("invalid vertex line: {line}")));
            }
            let external: u64 = parts[1]
                .parse()
                .map_err(|_| Error::Storage(format!("invalid vertex id: {line}")))?;
            let label: i64 = parts[2]
                .parse()
                .map_err(|_| Error::Storage(format!("invalid vertex label: {line}")))?;
            let community: i64 = parts
                .get(3)
                .map(|value| value.parse().unwrap_or(0))
                .unwrap_or(0);
            let id = external + 1;
            max_vid = max_vid.max(id);
            let mut properties = PropertyMap::new();
            properties.insert(PropertyKeyId(1), PropertyValue::Int(label));
            properties.insert(PropertyKeyId(2), PropertyValue::Int(community));
            records.push(PhysicalRecord {
                offset: records.len() as u64 + 1,
                kind: RecordKind::Vertex,
                logical_id: id,
                begin_ts: 1,
                prev_offset: 0,
                txn_id: 0,
                body: Some(RecordBody::Vertex(VertexRecord {
                    id: VertexId(id),
                    labels: vec![LabelId(0)],
                    properties,
                })),
            });
        }
        for edge_index in 0..edge_count {
            let line = lines
                .next()
                .ok_or_else(|| Error::Storage("insufficient edge lines".into()))?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 || parts[0] != "e" {
                return Err(Error::Storage(format!("invalid edge line: {line}")));
            }
            let src: u64 = parts[1]
                .parse()
                .map_err(|_| Error::Storage(format!("invalid edge source: {line}")))?;
            let dst: u64 = parts[2]
                .parse()
                .map_err(|_| Error::Storage(format!("invalid edge target: {line}")))?;
            let eid = edge_index + 1;
            records.push(PhysicalRecord {
                offset: records.len() as u64 + 1,
                kind: RecordKind::Edge,
                logical_id: eid,
                begin_ts: 1,
                prev_offset: 0,
                txn_id: 0,
                body: Some(RecordBody::Edge(EdgeRecord {
                    id: EdgeId(eid),
                    src: VertexId(src + 1),
                    dst: VertexId(dst + 1),
                    etype: EdgeTypeId(0),
                    properties: PropertyMap::new(),
                })),
            });
        }

        let segment_path = base_dir.join(SEGMENT_FILE);
        let _ = std::fs::remove_file(&segment_path);
        append_segment_to(&segment_path, &records)?;

        let wal_path = base_dir.join(WAL_FILE);
        let _ = std::fs::remove_file(&wal_path);
        let mut wal = Wal::open(&wal_path)?;
        wal.append(&WalEntry::Checkpoint {
            commit_ts: 1,
            next_vid: max_vid + 1,
            next_eid: edge_count + 1,
            segment_len: records.len() as u64,
        })?;

        let mut engine = Engine::new();
        for record in &records {
            engine.store(record.clone());
        }
        for record in &records {
            engine.link(record);
        }
        engine.commit_ts = 1;
        engine.next_vid = max_vid + 1;
        engine.next_eid = edge_count + 1;

        let file_len = std::fs::metadata(&segment_path).map(|m| m.len()).unwrap_or(0);
        let segment = SegmentWriter::open(&segment_path, file_len, file_len)?;
        Ok(Self {
            schema,
            base_dir,
            engine: Mutex::new(engine),
            pending: Mutex::new(HashMap::new()),
            wal: Mutex::new(wal),
            segment: Mutex::new(segment),
        })
    }

    /// Offset currently indexed for a logical vertex (candidate diagnostics).
    pub fn redirection_offset(&self, logical: VertexId) -> Option<u64> {
        self.engine.lock().unwrap().vertex_head.get(&logical.0).copied()
    }

    /// Offset currently indexed for a logical edge (candidate diagnostics).
    pub fn edge_redirection_offset(&self, logical: EdgeId) -> Option<u64> {
        self.engine.lock().unwrap().edge_head.get(&logical.0).copied()
    }

    /// Number of immutable physical records currently materialised.
    pub fn physical_record_count(&self) -> usize {
        self.engine.lock().unwrap().records.len()
    }

    /// Number of committed transactions durably recorded.
    pub fn commit_timestamp(&self) -> u64 {
        self.engine.lock().unwrap().commit_ts
    }

    /// The schema handle this storage was opened with.
    pub fn schema(&self) -> &Arc<dyn GraphSchema> {
        &self.schema
    }

    /// Directory holding the record segment and the WAL.
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

}

/// Parse the record segment; stops at a torn trailing record.
/// Parse the record segment.
///
/// Returns the decoded records, the byte offset where decoding stopped, and
/// whether everything past that offset is zero.  Zero means an untouched
/// reservation; anything else is a torn trailing write that must be cleared
/// before the next append.
fn read_segment(path: &Path) -> Result<(Vec<PhysicalRecord>, u64, bool)> {
    if !path.exists() {
        return Ok((Vec::new(), 0, true));
    }
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(io_error)?
        .read_to_end(&mut bytes)
        .map_err(io_error)?;
    let mut records = Vec::new();
    let mut cursor = 0usize;
    let mut offset = 1u64;
    while cursor < bytes.len() {
        let Some(record) = PhysicalRecord::decode(&bytes[cursor..], offset) else {
            break;
        };
        cursor += record.encoded_len();
        records.push(record);
        offset += 1;
    }
    let clean_tail = bytes[cursor..].iter().all(|byte| *byte == 0);
    Ok((records, cursor as u64, clean_tail))
}

/// Overwrite a byte range with zeros (used to clear a torn trailing write).
fn zero_range(file: &mut File, from: u64, to: u64) -> Result<()> {
    let zeros = vec![0u8; (1 << 20).min((to.saturating_sub(from)) as usize)];
    if zeros.is_empty() {
        return Ok(());
    }
    let mut offset = from;
    while offset < to {
        let len = zeros.len().min((to - offset) as usize);
        file.write_all_at(&zeros[..len], offset).map_err(io_error)?;
        offset += len as u64;
    }
    file.sync_data().map_err(io_error)
}

/// Rebuild committed state from the segment plus the WAL.
///
/// The two are handled separately on purpose:
///
/// * every record physically present in the segment is *stored*, so the record
///   numbering always follows the segment and a later append can never reuse a
///   position that is already occupied — not even a position holding a record
///   from a transaction whose WAL commit marker never became durable;
/// * only *committed* records are *linked* into the redirection index, so a
///   transaction that never committed stays invisible while still occupying its
///   slot in the segment.
///
/// A committed record whose segment write was lost is re-appended and linked at
/// its new position.
fn replay(
    engine: &mut Engine,
    parsed: Vec<PhysicalRecord>,
    entries: Vec<WalEntry>,
    segment: &mut SegmentWriter,
) -> Result<()> {
    let mut checkpoint_ts: u64 = 0;
    let mut next_vid: u64 = 1;
    let mut next_eid: u64 = 1;
    let mut committed: Vec<PhysicalRecord> = Vec::new();

    for entry in &entries {
        match entry {
            WalEntry::Checkpoint {
                commit_ts,
                next_vid: vid,
                next_eid: eid,
                ..
            } => {
                checkpoint_ts = checkpoint_ts.max(*commit_ts);
                next_vid = next_vid.max(*vid);
                next_eid = next_eid.max(*eid);
            }
            WalEntry::Commit {
                commit_ts,
                next_vid: vid,
                next_eid: eid,
                records,
                ..
            } => {
                checkpoint_ts = checkpoint_ts.max(*commit_ts);
                next_vid = next_vid.max(*vid);
                next_eid = next_eid.max(*eid);
                committed.extend(records.iter().cloned());
            }
            WalEntry::Abort { .. } => {}
        }
    }

    // Bulk-import records (txn_id 0) are committed by the checkpoint entry; they
    // were fsynced with the segment and are never carried in the WAL.
    committed.extend(parsed.iter().filter(|record| record.txn_id == 0).cloned());

    // Identity of a physical record across a reopen, including its namespace:
    // `VertexId(1)` and `EdgeId(1)` are distinct logical elements that may share
    // a commit timestamp.  Uncommitted records always carry `begin_ts == 0`, so
    // they can never collide with a committed record's key.
    let mut materialised: HashMap<(u8, u64, u64, u64), u64> = HashMap::new();
    for record in &parsed {
        materialised.insert(replay_key(record), record.offset);
    }

    let parsed_len = parsed.len() as u64;
    committed.sort_by_key(|record| (record.begin_ts, record.offset));

    // Assign final positions.  Records the segment already holds keep theirs;
    // records whose segment write was lost are re-appended after the segment.
    let mut next_offset = parsed_len + 1;
    let mut relocated: HashMap<u64, u64> = HashMap::new();
    for record in committed.iter_mut() {
        match materialised.get(&replay_key(record)) {
            Some(offset) => record.offset = *offset,
            None => {
                relocated.insert(record.offset, next_offset);
                record.offset = next_offset;
                next_offset += 1;
            }
        }
    }
    // A relocated version's predecessor may itself have been relocated.
    for record in committed.iter_mut() {
        if record.prev_offset != 0 {
            if let Some(mapped) = relocated.get(&record.prev_offset) {
                record.prev_offset = *mapped;
            }
        }
    }

    let repaired: Vec<PhysicalRecord> = committed
        .iter()
        .filter(|record| record.offset > parsed_len)
        .cloned()
        .collect();
    if !repaired.is_empty() {
        // Re-materialising a committed record whose segment write was lost: make
        // it durable immediately, since this is a recovery path.
        segment.append(&repaired, true)?;
    }

    // Every physical record takes its segment position, committed or not.
    let mut physical = parsed;
    physical.extend(repaired);
    physical.sort_by_key(|record| record.offset);
    for record in physical {
        engine.store(record);
    }

    // Only committed versions become visible.
    for record in &committed {
        engine.link(record);
    }

    engine.commit_ts = checkpoint_ts.max(engine.commit_ts);
    engine.next_vid = next_vid;
    engine.next_eid = next_eid;
    for id in engine.vertex_head.keys() {
        engine.next_vid = engine.next_vid.max(id + 1);
    }
    for id in engine.edge_head.keys() {
        engine.next_eid = engine.next_eid.max(id + 1);
    }
    Ok(())
}

/// Identity of a physical record across a reopen, including its namespace.
fn replay_key(record: &PhysicalRecord) -> (u8, u64, u64, u64) {
    let namespace = match record.kind.namespace() {
        Namespace::Vertex => 0,
        Namespace::Edge => 1,
    };
    (
        namespace,
        record.logical_id,
        record.txn_id,
        record.begin_ts,
    )
}

fn append_segment_to(path: &Path, records: &[PhysicalRecord]) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(io_error)?;
    let mut buffer = Vec::new();
    for record in records {
        buffer.extend_from_slice(&record.encode());
    }
    file.write_all(&buffer).map_err(io_error)?;
    file.flush().map_err(io_error)?;
    file.sync_data().map_err(io_error)?;
    Ok(())
}

fn io_error(error: std::io::Error) -> Error {
    Error::Storage(error.to_string())
}

impl StorageEngine for PersistentStorage {
    fn get_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<Option<VertexRecord>> {
        self.get_vertex_inner(vid, ctx)
    }

    fn scan_vertices(&self, filter: VertexFilter, ctx: &TxnContext) -> Result<VertexIter> {
        let mut ids: Vec<u64> = Vec::new();
        let mut seen: HashSet<u64> = HashSet::new();
        {
            let engine = self.engine.lock().unwrap();
            let snapshot = self.snapshot_ts(ctx, &engine);
            for id in &engine.vertex_order {
                if engine.resolve(Namespace::Vertex, *id, snapshot).is_some() && seen.insert(*id) {
                    ids.push(*id);
                }
            }
        }
        {
            let pending = self.pending.lock().unwrap();
            if let Some(txn) = pending.get(&ctx.txn_id) {
                for write in &txn.writes {
                    match write {
                        PendingWrite::VertexPut { id, .. } => {
                            if seen.insert(*id) {
                                ids.push(*id);
                            }
                        }
                        PendingWrite::VertexDelete { id } => {
                            ids.retain(|candidate| candidate != id);
                        }
                        _ => {}
                    }
                }
            }
        }

        let mut out = Vec::new();
        for id in ids {
            if let Some(record) = self.get_vertex(VertexId(id), ctx)? {
                if let Some(labels) = &filter.labels {
                    if !record.labels.iter().any(|label| labels.contains(label)) {
                        continue;
                    }
                }
                out.push(record);
            }
        }
        Ok(Box::new(out.into_iter()))
    }

    fn get_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<Option<EdgeRecord>> {
        self.get_edge_inner(eid, ctx)
    }

    fn out_edges(&self, vid: VertexId, filter: EdgeFilter, ctx: &TxnContext) -> Result<EdgeIter> {
        let candidates = self.edge_candidates(vid, true, ctx);
        let mut out = Vec::new();
        for eid in candidates {
            if let Some(edge) = self.get_edge(EdgeId(eid), ctx)? {
                if let Some(types) = &filter.types {
                    if !types.contains(&edge.etype) {
                        continue;
                    }
                }
                out.push(edge);
            }
        }
        Ok(Box::new(out.into_iter()))
    }

    fn in_edges(&self, vid: VertexId, filter: EdgeFilter, ctx: &TxnContext) -> Result<EdgeIter> {
        let candidates = self.edge_candidates(vid, false, ctx);
        let mut out = Vec::new();
        for eid in candidates {
            if let Some(edge) = self.get_edge(EdgeId(eid), ctx)? {
                if let Some(types) = &filter.types {
                    if !types.contains(&edge.etype) {
                        continue;
                    }
                }
                out.push(edge);
            }
        }
        Ok(Box::new(out.into_iter()))
    }

    fn index_lookup(
        &self,
        _idx: IndexHandle,
        _key: &IndexKey,
        _ctx: &TxnContext,
    ) -> Result<VertexIter> {
        Err(Error::Unsupported(
            "no secondary property index is registered in this design".into(),
        ))
    }

    fn insert_vertex(&self, mut record: VertexRecord, ctx: &TxnContext) -> Result<VertexId> {
        self.ensure_writable(ctx)?;
        let id = {
            let mut engine = self.engine.lock().unwrap();
            if record.id.0 == 0 {
                let id = engine.next_vid;
                engine.next_vid += 1;
                id
            } else {
                engine.next_vid = engine.next_vid.max(record.id.0 + 1);
                record.id.0
            }
        };
        record.id = VertexId(id);
        self.with_pending(ctx.txn_id, |txn| {
            txn.push(PendingWrite::VertexPut { id, record });
        })?;
        Ok(VertexId(id))
    }

    fn insert_edge(&self, mut record: EdgeRecord, ctx: &TxnContext) -> Result<EdgeId> {
        self.ensure_writable(ctx)?;
        if self.get_vertex(record.src, ctx)?.is_none() {
            return Err(Error::Storage(format!(
                "edge source vertex {} does not exist",
                record.src.0
            )));
        }
        if self.get_vertex(record.dst, ctx)?.is_none() {
            return Err(Error::Storage(format!(
                "edge destination vertex {} does not exist",
                record.dst.0
            )));
        }
        let id = {
            let mut engine = self.engine.lock().unwrap();
            if record.id.0 == 0 {
                let id = engine.next_eid;
                engine.next_eid += 1;
                id
            } else {
                engine.next_eid = engine.next_eid.max(record.id.0 + 1);
                record.id.0
            }
        };
        record.id = EdgeId(id);
        self.with_pending(ctx.txn_id, |txn| {
            txn.push(PendingWrite::EdgePut { id, record });
        })?;
        Ok(EdgeId(id))
    }

    fn update_vertex(&self, record: VertexRecord, ctx: &TxnContext) -> Result<()> {
        self.ensure_writable(ctx)?;
        if self.get_vertex(record.id, ctx)?.is_none() {
            return Err(Error::NotFound(format!(
                "vertex {} does not exist",
                record.id.0
            )));
        }
        let id = record.id.0;
        self.with_pending(ctx.txn_id, |txn| {
            txn.push(PendingWrite::VertexPut { id, record });
        })
    }

    fn delete_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<()> {
        self.ensure_writable(ctx)?;
        if self.get_vertex(vid, ctx)?.is_none() {
            return Err(Error::NotFound(format!("vertex {} does not exist", vid.0)));
        }
        if self
            .out_edges(vid, EdgeFilter::default(), ctx)?
            .next()
            .is_some()
            || self
                .in_edges(vid, EdgeFilter::default(), ctx)?
                .next()
                .is_some()
        {
            return Err(Error::Storage(format!(
                "vertex {} still has incident edges",
                vid.0
            )));
        }
        self.with_pending(ctx.txn_id, |txn| {
            txn.push(PendingWrite::VertexDelete { id: vid.0 });
        })
    }

    fn update_edge(&self, record: EdgeRecord, ctx: &TxnContext) -> Result<()> {
        self.ensure_writable(ctx)?;
        let current = self
            .get_edge(record.id, ctx)?
            .ok_or_else(|| Error::NotFound(format!("edge {} does not exist", record.id.0)))?;
        if current.src != record.src || current.dst != record.dst || current.etype != record.etype {
            return Err(Error::Storage(format!(
                "edge {} endpoints and type are immutable",
                record.id.0
            )));
        }
        let id = record.id.0;
        self.with_pending(ctx.txn_id, |txn| {
            txn.push(PendingWrite::EdgePut { id, record });
        })
    }

    fn delete_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<()> {
        self.ensure_writable(ctx)?;
        if self.get_edge(eid, ctx)?.is_none() {
            return Err(Error::NotFound(format!("edge {} does not exist", eid.0)));
        }
        self.with_pending(ctx.txn_id, |txn| {
            txn.push(PendingWrite::EdgeDelete { id: eid.0 });
        })
    }
}
