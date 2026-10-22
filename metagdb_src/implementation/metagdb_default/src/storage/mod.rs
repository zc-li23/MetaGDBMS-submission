/**
 * @brief: Disk-based DefaultStorage, implementing the StorageEngine interface.
 *
 * This module persists vertex/edge/adjacency data to disk files, providing efficient access through buffer pools and indexes.
 * Keeps the DefaultStorage interface stable externally.
 *
 * Supports MVCC-based transaction semantics:
 * - insert writes uncommitted records and registers write set
 * - update appends a new version (prev points to old head), updates index to point to new version
 * - delete writes deleted_ts at commit time
 * - commit_txn updates MVCC header in-place (commit_ts + committed flag / deleted_ts)
 * - abort_txn rolls back index and cleans up write set
 * - read path traverses version chain to obtain visible version
 * - write locks ensure the same logical ID is not written concurrently
 */

mod buffer_pool;
mod index;
mod layout;
mod page;
mod page_manager;
mod record;
pub mod wal;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l1_model::GraphSchema;
use metagdb_core::l2_storage::{
    EdgeFilter, EdgeIter, EdgeRecord, IndexHandle, IndexKey, StorageEngine, VertexFilter,
    VertexIter, VertexRecord,
};
use metagdb_core::l3_txn::{IsolationLevel, TxnContext};
use metagdb_core::{Error, Result};

use self::buffer_pool::BufferPool;
use self::index::{Index, RecordLocation};
use self::layout::PAGE_HEADER_SIZE;
use self::page::PageId;
use self::page_manager::PageManager;
use self::record::{
    decode_adjacency, decode_edge, decode_vertex, encode_adjacency,
    encode_edge, encode_vertex, MvccHeader, MVCC_FLAG_COMMITTED,
    MVCC_OFFSET_CREATED_TS, MVCC_OFFSET_CREATOR_TXN, MVCC_OFFSET_DELETED_TS, MVCC_OFFSET_FLAGS,
};

// META_SIZE has been moved to layout.rs, use layout::META_SIZE

/// Write operation type, used to distinguish handling logic during commit/abort.
#[derive(Debug, Clone, Copy)]
enum WriteOp {
    /// INSERT: new record at location.
    Insert,
    /// UPDATE: new version at location, old version at old_location.
    Update { old_location: RecordLocation },
    /// DELETE: record marked as deleted at location, old_location retains the previous version pointer.
    Delete { old_location: Option<RecordLocation> },
}

/// Write set entry, recording the disk location and operation type of records written by a transaction.
#[derive(Debug, Clone, Copy)]
struct WriteSetEntry {
    location: RecordLocation,
    op: WriteOp,
}

/// Write set type: txn_id -> list of written records.
type WriteSet = HashMap<u64, Vec<WriteSetEntry>>;

/**
 * @brief: Disk-persistent default storage engine.
 */
pub struct DefaultStorage {
    /// In-memory index: ID -> disk location.
    index: RwLock<Index>,
    /// Page buffer pool.
    buffer_pool: BufferPool,
    /// Next allocatable vertex ID.
    next_vid: AtomicU64,
    /// Next allocatable edge ID.
    next_eid: AtomicU64,
    /// Next transaction timestamp (persisted to meta.dat, monotonically increasing across restarts).
    next_ts: AtomicU64,
    /// Next transaction ID (persisted to meta.dat, monotonically increasing across restarts).
    next_txn_id: AtomicU64,
    /// Current vertex page number being written to (reduces space check frequency).
    current_v_page: AtomicU64,
    /// Current edge page number being written to.
    current_e_page: AtomicU64,
    /// Data file base directory.
    base_dir: PathBuf,
    /// Associated metadata Schema.
    #[allow(dead_code)]
    schema: Arc<dyn GraphSchema>,
    /// Transaction write set: tracks records written by each transaction.
    write_set: Mutex<WriteSet>,
    /// Vertex write lock table: vertex_id -> txn_id, only one writer allowed per ID.
    write_locks_v: Mutex<HashMap<u64, u64>>,
    /// Edge write lock table: edge_id -> txn_id, only one writer allowed per ID.
    write_locks_e: Mutex<HashMap<u64, u64>>,
    /// WAL writer (None means recovery mode, WAL writes are skipped).
    wal: Mutex<Option<wal::WalWriter>>,
}

impl DefaultStorage {
    /**
     * @brief: Open an existing database or create a new one (default data/ directory).
     * @param schema: Associated metadata Schema.
     */
    pub fn new(schema: Arc<dyn GraphSchema>) -> Self {
        Self::with_base_dir(schema, Path::new(layout::DATA_DIR).to_path_buf())
    }

    /**
     * @brief: Open an existing database or create a new one (specified base directory).
     * @param schema: Associated metadata Schema.
     * @param base_dir: Data file storage directory.
     */
    pub fn with_base_dir(schema: Arc<dyn GraphSchema>, base_dir: PathBuf) -> Self {
        // Crash recovery: if WAL is non-empty, replay with a temporary WAL-free instance
        let wal_entries = wal::read_wal_entries(&base_dir).unwrap_or_default();
        if !wal_entries.is_empty() {
            let recovery_storage = Self::new_recovery_instance(schema.clone(), base_dir.clone());
            for op in &wal_entries {
                match op {
                    wal::WalOp::InsertVertex(rec) => {
                        let ctx = TxnContext { txn_id: 0, isolation: IsolationLevel::SnapshotIsolation, read_only: false, start_ts: 1 };
                        let _ = recovery_storage.insert_vertex(rec.clone(), &ctx);
                    }
                    wal::WalOp::UpdateVertex(rec) => {
                        let ctx = TxnContext { txn_id: 0, isolation: IsolationLevel::SnapshotIsolation, read_only: false, start_ts: 1 };
                        let _ = recovery_storage.update_vertex(rec.clone(), &ctx);
                    }
                    wal::WalOp::DeleteVertex(vid) => {
                        let ctx = TxnContext { txn_id: 0, isolation: IsolationLevel::SnapshotIsolation, read_only: false, start_ts: 1 };
                        let _ = recovery_storage.delete_vertex(*vid, &ctx);
                    }
                    wal::WalOp::InsertEdge(rec) => {
                        let ctx = TxnContext { txn_id: 0, isolation: IsolationLevel::SnapshotIsolation, read_only: false, start_ts: 1 };
                        let _ = recovery_storage.insert_edge(rec.clone(), &ctx);
                    }
                    wal::WalOp::UpdateEdge(rec) => {
                        let ctx = TxnContext { txn_id: 0, isolation: IsolationLevel::SnapshotIsolation, read_only: false, start_ts: 1 };
                        let _ = recovery_storage.update_edge(rec.clone(), &ctx);
                    }
                    wal::WalOp::DeleteEdge(eid) => {
                        let ctx = TxnContext { txn_id: 0, isolation: IsolationLevel::SnapshotIsolation, read_only: false, start_ts: 1 };
                        let _ = recovery_storage.delete_edge(*eid, &ctx);
                    }
                }
            }
            let _ = recovery_storage.sync();
            wal::remove_wal(&base_dir).ok();
        }

        let (next_vid, next_eid, next_ts, next_txn_id) = match read_meta(&base_dir) {
            Ok(v) => v,
            Err(_) => {
                let _ = write_meta(&base_dir, 0, 0, 0, 0);
                (0, 0, 0, 0)
            }
        };

        // If upgrading from old format, ensure counters are at least 1
        let next_ts = if next_ts == 0 { 1 } else { next_ts };
        let next_txn_id = if next_txn_id == 0 { 1 } else { next_txn_id };

        let index = Index::load_from_disk(&base_dir).unwrap_or_default();

        let page_manager =
            PageManager::open_or_create(&base_dir).expect("Unable to open or create data files");
        let buffer_pool = BufferPool::new(page_manager);

        // Initialize WAL writer
        let wal_writer = wal::WalWriter::open(&base_dir).ok();

        Self {
            index: RwLock::new(index),
            buffer_pool,
            next_vid: AtomicU64::new(next_vid),
            next_eid: AtomicU64::new(next_eid),
            next_ts: AtomicU64::new(next_ts),
            next_txn_id: AtomicU64::new(next_txn_id),
            current_v_page: AtomicU64::new(0),
            current_e_page: AtomicU64::new(0),
            base_dir,
            schema,
            write_set: Mutex::new(WriteSet::new()),
            write_locks_v: Mutex::new(HashMap::new()),
            write_locks_e: Mutex::new(HashMap::new()),
            wal: Mutex::new(wal_writer),
        }
    }

    /// Create a recovery instance that does not write WAL
    fn new_recovery_instance(schema: Arc<dyn GraphSchema>, base_dir: PathBuf) -> Self {
        let index = Index::load_from_disk(&base_dir).unwrap_or_default();
        let page_manager = PageManager::open_or_create(&base_dir).expect("Recovery: unable to open data files");
        let buffer_pool = BufferPool::new(page_manager);
        Self {
            index: RwLock::new(index),
            buffer_pool,
            next_vid: AtomicU64::new(0),
            next_eid: AtomicU64::new(0),
            next_ts: AtomicU64::new(1),
            next_txn_id: AtomicU64::new(1),
            current_v_page: AtomicU64::new(0),
            current_e_page: AtomicU64::new(0),
            base_dir,
            schema,
            write_set: Mutex::new(WriteSet::new()),
            write_locks_v: Mutex::new(HashMap::new()),
            write_locks_e: Mutex::new(HashMap::new()),
            wal: Mutex::new(None),  // recovery: do not write WAL
        }
    }

    /**
     * @brief: Bulk load data from a .graph file, creating a new database.
     *
     * Bypasses per-record WAL writes, MVCC version chain, and write lock overhead — all records
     * are written directly to pages in committed state (created_ts=1, flags=committed),
     * with a single sync flush after loading completes.
     *
     * @param schema: Associated metadata Schema.
     * @param base_dir: Data file storage directory (if it already exists, remove and recreate).
     * @param graph_path: .graph file path.
     * @return: On success, returns a DefaultStorage instance populated with data.
     */
    pub fn bulk_load_from_graph(
        schema: Arc<dyn GraphSchema>,
        base_dir: PathBuf,
        graph_path: &Path,
    ) -> Result<Self> {
        use std::fs;
        use std::io::{BufRead, BufReader};

        // 1. If the target directory already exists, remove and recreate it
        if base_dir.exists() {
            fs::remove_dir_all(&base_dir)
                .map_err(|e| Error::Storage(format!("Failed to remove old directory: {}", e)))?;
        }
        fs::create_dir_all(&base_dir)
            .map_err(|e| Error::Storage(format!("Failed to create data directory: {}", e)))?;

        // 2. Create PageManager + BufferPool (empty data files)
        let page_manager =
            PageManager::open_or_create(&base_dir).map_err(|e| Error::Storage(format!("Failed to create page manager: {}", e)))?;
        let buffer_pool = BufferPool::new(page_manager);

        // 3. Create empty index
        let mut index = Index::new();

        // 4. Open .graph file for streaming read
        let file = File::open(graph_path)
            .map_err(|e| Error::Storage(format!("Unable to open .graph file {}: {}", graph_path.display(), e)))?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // 5. Parse meta line "t m n"
        let meta_line = lines
            .next()
            .ok_or_else(|| Error::Storage(".graph file is empty".into()))?
            .map_err(|e| Error::Storage(format!("Failed to read meta line: {}", e)))?;
        let meta_parts: Vec<&str> = meta_line.split_whitespace().collect();
        if meta_parts.len() != 3 || meta_parts[0] != "t" {
            return Err(Error::Storage(format!(
                "Invalid meta line format, expected 't n m' (n=vertex count, m=edge count), actual: '{}'",
                meta_line
            )));
        }
        let vertex_count: u64 = meta_parts[1].parse().map_err(|e| {
            Error::Storage(format!("Failed to parse vertex count from meta line: {}", e))
        })?;
        let edge_count: u64 = meta_parts[2].parse().map_err(|e| {
            Error::Storage(format!("Failed to parse edge count from meta line: {}", e))
        })?;

        println!(
            "Loading .graph file: {} vertices, {} edges",
            vertex_count, edge_count
        );

        // 6. Bulk load vertices (lines 2..n+1)
        //    format: "v v_id v_label v_com"
        let committed_mvcc = MvccHeader::new(1, 0, 0, MVCC_FLAG_COMMITTED);
        let mut line_num: u64 = 1;

        for _ in 0..vertex_count {
            line_num += 1;
            let vline = lines
                .next()
                .ok_or_else(|| {
                    Error::Storage(format!(
                        "Insufficient vertex lines: expected {} lines, ended early at line {}",
                        vertex_count, line_num
                    ))
                })?
                .map_err(|e| Error::Storage(format!("Failed to read line {}: {}", line_num, e)))?;
            let parts: Vec<&str> = vline.split_whitespace().collect();
            if parts.len() < 3 || parts[0] != "v" {
                return Err(Error::Storage(format!(
                    "Invalid vertex line format (line {}), expected 'v v_id v_label ...', actual: '{}'",
                    line_num, vline
                )));
            }
            let v_id: u64 = parts[1].parse().map_err(|e| {
                Error::Storage(format!("Failed to parse v_id at line {}: {}", line_num, e))
            })?;
            let v_label: i64 = parts[2].parse().map_err(|e| {
                Error::Storage(format!("Failed to parse v_label at line {}: {}", line_num, e))
            })?;
            let v_com: i64 = if parts.len() >= 4 {
                parts[3].parse().map_err(|e| {
                    Error::Storage(format!("Failed to parse v_com at line {}: {}", line_num, e))
                })?
            } else {
                0
            };

            let mut props = PropertyMap::new();
            props.insert(PropertyKeyId(1), PropertyValue::Int(v_label));
            props.insert(PropertyKeyId(2), PropertyValue::Int(v_com));

            let record = VertexRecord {
                id: VertexId(v_id + 1), // file 0-based → storage 1-based
                labels: vec![LabelId(0)],
                properties: props,
            };

            let encoded = encode_vertex(&record, &committed_mvcc, None);
            let loc = {
                let needed = 2 + encoded.len();
                let mut page_num = 0u64;
                loop {
                    let page_id = PageId::new(layout::FILE_VERTICES, page_num);
                    let has_space = buffer_pool
                        .has_space(page_id, needed)
                        .unwrap_or(false);
                    if has_space {
                        let slot = buffer_pool
                            .write_record(page_id, &encoded)
                            .map_err(|e| {
                                Error::Storage(format!("Failed to write vertex record: {}", e))
                            })?;
                        break RecordLocation::new(layout::FILE_VERTICES, page_num, slot);
                    }
                    page_num += 1;
                    let next_id = PageId::new(layout::FILE_VERTICES, page_num);
                    if buffer_pool.has_space(next_id, needed).is_err() {
                        let new_id = buffer_pool
                            .allocate_page(layout::FILE_VERTICES)
                            .map_err(|e| {
                                Error::Storage(format!("Failed to allocate vertex page: {}", e))
                            })?;
                        let slot = buffer_pool
                            .write_record(new_id, &encoded)
                            .map_err(|e| {
                                Error::Storage(format!("Failed to write vertex record: {}", e))
                            })?;
                        break RecordLocation::new(
                            layout::FILE_VERTICES,
                            new_id.page_num,
                            slot,
                        );
                    }
                }
            };
            index.insert_vertex(record.id, loc);

            // Progress printing
            let vid = v_id + 1;
            if vid % 10000 == 0 {
                println!("  Loaded {} / {} vertices", vid, vertex_count);
            }
        }

        // 7. Bulk load edges (last m lines)
        //    format: "e src dst"
        let mut out_adj: HashMap<VertexId, Vec<EdgeId>> = HashMap::new();
        let mut in_adj: HashMap<VertexId, Vec<EdgeId>> = HashMap::new();

        for eid_val in 1..=edge_count {
            line_num += 1;
            let eline = lines
                .next()
                .ok_or_else(|| {
                    Error::Storage(format!(
                        "Insufficient edge lines: expected {} lines, ended early at line {}",
                        edge_count, line_num
                    ))
                })?
                .map_err(|e| Error::Storage(format!("Failed to read line {}: {}", line_num, e)))?;
            let parts: Vec<&str> = eline.split_whitespace().collect();
            if parts.len() < 3 || parts[0] != "e" {
                return Err(Error::Storage(format!(
                    "Invalid edge line format (line {}), expected 'e src dst', actual: '{}'",
                    line_num, eline
                )));
            }
            let src: u64 = parts[1].parse().map_err(|e| {
                Error::Storage(format!("Failed to parse src at line {}: {}", line_num, e))
            })?;
            let dst: u64 = parts[2].parse().map_err(|e| {
                Error::Storage(format!("Failed to parse dst at line {}: {}", line_num, e))
            })?;

            let eid = EdgeId(eid_val);
            let src_vid = VertexId(src + 1);
            let dst_vid = VertexId(dst + 1);

            let record = EdgeRecord {
                id: eid,
                src: src_vid,
                dst: dst_vid,
                etype: EdgeTypeId(0),
                properties: PropertyMap::new(),
            };

            let encoded = encode_edge(&record, &committed_mvcc, None);
            let loc = {
                let needed = 2 + encoded.len();
                let mut page_num = 0u64;
                loop {
                    let page_id = PageId::new(layout::FILE_EDGES, page_num);
                    let has_space = buffer_pool
                        .has_space(page_id, needed)
                        .unwrap_or(false);
                    if has_space {
                        let slot = buffer_pool
                            .write_record(page_id, &encoded)
                            .map_err(|e| {
                                Error::Storage(format!("Failed to write edge record: {}", e))
                            })?;
                        break RecordLocation::new(layout::FILE_EDGES, page_num, slot);
                    }
                    page_num += 1;
                    let next_id = PageId::new(layout::FILE_EDGES, page_num);
                    if buffer_pool.has_space(next_id, needed).is_err() {
                        let new_id = buffer_pool
                            .allocate_page(layout::FILE_EDGES)
                            .map_err(|e| {
                                Error::Storage(format!("Failed to allocate edge page: {}", e))
                            })?;
                        let slot = buffer_pool
                            .write_record(new_id, &encoded)
                            .map_err(|e| {
                                Error::Storage(format!("Failed to write edge record: {}", e))
                            })?;
                        break RecordLocation::new(
                            layout::FILE_EDGES,
                            new_id.page_num,
                            slot,
                        );
                    }
                }
            };
            index.insert_edge(eid, loc);

            // Accumulate adjacency lists
            out_adj.entry(src_vid).or_default().push(eid);
            in_adj.entry(dst_vid).or_default().push(eid);

            if eid_val % 10000 == 0 {
                println!("  Loaded {} / {} edges", eid_val, edge_count);
            }
        }

        // 8. Write adjacency pages for all vertices
        //    First collect vertex IDs (the index will later be moved into storage)
        let vids: Vec<VertexId> = index.vertex_index.keys().copied().collect();
        for vid in &vids {
            let out = out_adj.get(vid).cloned().unwrap_or_default();
            let inp = in_adj.get(vid).cloned().unwrap_or_default();
            let adj_data = encode_adjacency(&out, &inp);
            buffer_pool
                .ensure_page_exists(layout::FILE_ADJ, vid.0)
                .map_err(|e| {
                    Error::Storage(format!("Ensure adjacency page vid={:?}: {}", vid, e))
                })?;
            let page_id = PageId::new(layout::FILE_ADJ, vid.0);
            buffer_pool
                .write_bytes(page_id, PAGE_HEADER_SIZE, &adj_data)
                .map_err(|e| {
                    Error::Storage(format!("Write adjacency page vid={:?}: {}", vid, e))
                })?;
        }

        // 9. Construct DefaultStorage and write meta.dat + index.dat
        let storage = Self {
            index: RwLock::new(index),
            buffer_pool,
            next_vid: AtomicU64::new(vertex_count + 1),
            next_eid: AtomicU64::new(edge_count + 1),
            next_ts: AtomicU64::new(1),
            next_txn_id: AtomicU64::new(1),
            current_v_page: AtomicU64::new(0),
            current_e_page: AtomicU64::new(0),
            base_dir,
            schema,
            write_set: Mutex::new(WriteSet::new()),
            write_locks_v: Mutex::new(HashMap::new()),
            write_locks_e: Mutex::new(HashMap::new()),
            wal: Mutex::new(None), // Bulk load does not need WAL
        };

        // 10. Persist metadata and index
        storage.sync()?;

        println!(
            "Bulk load complete: {} vertices, {} edges",
            vertex_count, edge_count
        );

        Ok(storage)
    }

    /**
     * @brief: Persist metadata and index, and sync all files.
     */
    pub fn sync(&self) -> Result<()> {
        let index = self.index.read().map_err(|_| lock_poisoned())?;
        index
            .save_to_disk(&self.base_dir)
            .map_err(|e| Error::Storage(format!("Failed to save index: {}", e)))?;
        drop(index);

        write_meta(
            &self.base_dir,
            self.next_vid.load(Ordering::SeqCst),
            self.next_eid.load(Ordering::SeqCst),
            self.next_ts.load(Ordering::SeqCst),
            self.next_txn_id.load(Ordering::SeqCst),
        )
        .map_err(|e| Error::Storage(format!("Failed to save metadata: {}", e)))?;

        self.buffer_pool
            .sync_all()
            .map_err(|e| Error::Storage(format!("Failed to sync buffer pool: {}", e)))?;

        // WAL truncation: all data has been safely flushed to disk
        if let Ok(mut guard) = self.wal.lock() {
            if let Some(ref mut w) = *guard {
                let _ = w.truncate();
            }
        }

        Ok(())
    }

    /**
     * @brief: Allocate next transaction timestamp and persist.
     * @return: Newly allocated timestamp value.
     */
    pub fn alloc_ts(&self) -> u64 {
        let ts = self.next_ts.fetch_add(1, Ordering::SeqCst);
        // Best-effort persist; if failed does not block (will be rewritten on next sync)
        let _ = write_meta(
            &self.base_dir,
            self.next_vid.load(Ordering::SeqCst),
            self.next_eid.load(Ordering::SeqCst),
            self.next_ts.load(Ordering::SeqCst),
            self.next_txn_id.load(Ordering::SeqCst),
        );
        ts
    }

    /**
     * @brief: Allocate next transaction ID and persist.
     * @return: Newly allocated transaction ID.
     */
    pub fn alloc_txn_id(&self) -> u64 {
        let id = self.next_txn_id.fetch_add(1, Ordering::SeqCst);
        let _ = write_meta(
            &self.base_dir,
            self.next_vid.load(Ordering::SeqCst),
            self.next_eid.load(Ordering::SeqCst),
            self.next_ts.load(Ordering::SeqCst),
            self.next_txn_id.load(Ordering::SeqCst),
        );
        id
    }

    /**
     * @brief: Get next transaction timestamp (without allocating).
     * @return: Current next_ts value.
     */
    pub fn current_ts(&self) -> u64 {
        self.next_ts.load(Ordering::SeqCst)
    }

    /**
     * @brief: Get next transaction ID (without allocating).
     * @return: Current next_txn_id value.
     */
    pub fn current_txn_id(&self) -> u64 {
        self.next_txn_id.load(Ordering::SeqCst)
    }

    /**
     * @brief: Commit transaction: write MVCC header fields for records in write set, release write locks, then flush.
     * @param txn_id: Transaction ID initiating the commit.
     * @param commit_ts: Commit timestamp (i.e., the final value of created_ts).
     */
    pub fn commit_txn(&self, txn_id: u64, commit_ts: u64) -> Result<()> {
        let entries = {
            let mut ws = self.write_set.lock().map_err(|_| lock_poisoned())?;
            ws.remove(&txn_id).unwrap_or_default()
        };

        for entry in &entries {
            let page_id = PageId::new(entry.location.file_tag, entry.location.page_num);

            match entry.op {
                WriteOp::Insert | WriteOp::Update { .. } => {
                    // Update created_ts
                    self.buffer_pool
                        .write_record_bytes(
                            page_id,
                            entry.location.slot,
                            MVCC_OFFSET_CREATED_TS,
                            &commit_ts.to_le_bytes(),
                        )
                        .map_err(|e| Error::Storage(format!("commit Update created_ts: {}", e)))?;

                    // Set committed flag
                    self.buffer_pool
                        .write_record_bytes(
                            page_id,
                            entry.location.slot,
                            MVCC_OFFSET_FLAGS,
                            &[MVCC_FLAG_COMMITTED],
                        )
                        .map_err(|e| Error::Storage(format!("commit update flags: {}", e)))?;
                }
                WriteOp::Delete { .. } => {
                    // DELETE: write deleted_ts on the old version
                    self.buffer_pool
                        .write_record_bytes(
                            page_id,
                            entry.location.slot,
                            MVCC_OFFSET_DELETED_TS,
                            &commit_ts.to_le_bytes(),
                        )
                        .map_err(|e| Error::Storage(format!("commit update deleted_ts: {}", e)))?;

                    // Ensure committed flag is set (if this record was previously INSERTed, it may already be set)
                    self.buffer_pool
                        .write_record_bytes(
                            page_id,
                            entry.location.slot,
                            MVCC_OFFSET_FLAGS,
                            &[MVCC_FLAG_COMMITTED],
                        )
                        .map_err(|e| Error::Storage(format!("commit update flags: {}", e)))?;
                }
            }
        }

        // Release all write locks held by this transaction
        self.release_write_locks(txn_id);

        // Sync index and metadata after commit
        self.sync()
    }

    /**
     * @brief: Abort transaction: rollback index, clean up MVCC header, release write locks.
     * @param txn_id: Transaction ID to be aborted.
     */
    pub fn abort_txn(&self, txn_id: u64) -> Result<()> {
        let entries = {
            let mut ws = self.write_set.lock().map_err(|_| lock_poisoned())?;
            ws.remove(&txn_id).unwrap_or_default()
        };

        let mut index = self.index.write().map_err(|_| lock_poisoned())?;

        for entry in &entries {
            let page_id = PageId::new(entry.location.file_tag, entry.location.page_num);

            match entry.op {
                WriteOp::Insert => {
                    // INSERT rollback: remove from index and clean up MVCC header
                    // Obtain ID by scanning record data, then remove from index
                    let data = self
                        .buffer_pool
                        .read_record(page_id, entry.location.slot)
                        .map_err(|e| Error::Storage(format!("abort read record: {}", e)))?;
                    // Extract ID from encoding (both vertex and edge IDs are in the first 8 bytes of payload)
                    if entry.location.file_tag == layout::FILE_VERTICES {
                        if let Some((_, _, record)) = decode_vertex(&data) {
                            index.remove_vertex(record.id);
                        }
                    } else if entry.location.file_tag == layout::FILE_EDGES {
                        if let Some((_, _, record)) = decode_edge(&data) {
                            index.remove_edge(record.id);
                        }
                    }

                    // Clear creator_txn and flags
                    self.clear_mvcc_header(page_id, entry.location.slot)?;
                }
                WriteOp::Update { old_location } => {
                    // UPDATE rollback: index reverts to old version, clean up new version MVCC header
                    let data = self
                        .buffer_pool
                        .read_record(page_id, entry.location.slot)
                        .map_err(|e| Error::Storage(format!("abort read record: {}", e)))?;
                    if entry.location.file_tag == layout::FILE_VERTICES {
                        if let Some((_, _, record)) = decode_vertex(&data) {
                            index.insert_vertex(record.id, old_location);
                        }
                    } else if entry.location.file_tag == layout::FILE_EDGES {
                        if let Some((_, _, record)) = decode_edge(&data) {
                            index.insert_edge(record.id, old_location);
                        }
                    }

                    // Clear new version MVCC header
                    self.clear_mvcc_header(page_id, entry.location.slot)?;
                }
                WriteOp::Delete { old_location } => {
                    // DELETE rollback: restore index if old location is recorded; deleted_ts is not written on disk, no cleanup needed
                    let data = self
                        .buffer_pool
                        .read_record(page_id, entry.location.slot)
                        .map_err(|e| Error::Storage(format!("abort read record: {}", e)))?;
                    if let Some(old_loc) = old_location {
                        if entry.location.file_tag == layout::FILE_VERTICES {
                            if let Some((_, _, record)) = decode_vertex(&data) {
                                index.insert_vertex(record.id, old_loc);
                            }
                        } else if entry.location.file_tag == layout::FILE_EDGES {
                            if let Some((_, _, record)) = decode_edge(&data) {
                                index.insert_edge(record.id, old_loc);
                            }
                        }
                    }
                }
            }
        }

        drop(index);

        // Release all write locks held by this transaction
        self.release_write_locks(txn_id);

        Ok(())
    }

    /// Clear the MVCC header of a record (creator_txn and flags set to zero).
    fn clear_mvcc_header(&self, page_id: PageId, slot: u16) -> Result<()> {
        self.buffer_pool
            .write_record_bytes(page_id, slot, MVCC_OFFSET_CREATOR_TXN, &0u64.to_le_bytes())
            .map_err(|e| Error::Storage(format!("clear creator_txn: {}", e)))?;
        self.buffer_pool
            .write_record_bytes(page_id, slot, MVCC_OFFSET_FLAGS, &[0u8])
            .map_err(|e| Error::Storage(format!("clear flags: {}", e)))?;
        Ok(())
    }

    /// Allocate vertex ID.
    fn alloc_vid(&self, id: VertexId) -> VertexId {
        if id.0 == 0 {
            self.next_vid.fetch_add(1, Ordering::SeqCst);
            VertexId(self.next_vid.load(Ordering::SeqCst) - 1)
        } else {
            let mut current = self.next_vid.load(Ordering::SeqCst);
            while current <= id.0 {
                let _ = self.next_vid.compare_exchange(
                    current,
                    id.0 + 1,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
                current = self.next_vid.load(Ordering::SeqCst);
            }
            id
        }
    }

    /// Allocate edge ID.
    fn alloc_eid(&self, id: EdgeId) -> EdgeId {
        if id.0 == 0 {
            self.next_eid.fetch_add(1, Ordering::SeqCst);
            EdgeId(self.next_eid.load(Ordering::SeqCst) - 1)
        } else {
            let mut current = self.next_eid.load(Ordering::SeqCst);
            while current <= id.0 {
                let _ = self.next_eid.compare_exchange(
                    current,
                    id.0 + 1,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
                current = self.next_eid.load(Ordering::SeqCst);
            }
            id
        }
    }

    /// Find or allocate a page with enough space in the vertex file, append the record and return location.
    fn append_vertex_record(&self, data: &[u8]) -> Result<RecordLocation> {
        let needed = 2 + data.len();
        let mut page_num = self.current_v_page.load(Ordering::SeqCst);

        loop {
            let page_id = PageId::new(layout::FILE_VERTICES, page_num);
            let has_space = self
                .buffer_pool
                .has_space(page_id, needed)
                .unwrap_or(false);
            if has_space {
                let slot = self
                    .buffer_pool
                    .write_record(page_id, data)
                    .map_err(|e| Error::Storage(format!("Failed to write vertex record: {}", e)))?;
                return Ok(RecordLocation::new(layout::FILE_VERTICES, page_num, slot));
            }
            page_num += 1;
            let next_id = PageId::new(layout::FILE_VERTICES, page_num);
            if self.buffer_pool.has_space(next_id, needed).is_err() {
                let new_id = self
                    .buffer_pool
                    .allocate_page(layout::FILE_VERTICES)
                    .map_err(|e| Error::Storage(format!("Failed to allocate vertex page: {}", e)))?;
                self.current_v_page
                    .store(new_id.page_num, Ordering::SeqCst);
                let slot = self
                    .buffer_pool
                    .write_record(new_id, data)
                    .map_err(|e| Error::Storage(format!("Failed to write vertex record: {}", e)))?;
                return Ok(RecordLocation::new(
                    layout::FILE_VERTICES,
                    new_id.page_num,
                    slot,
                ));
            }
        }
    }

    /// Find or allocate a page with enough space in the edge file, append the record and return location.
    fn append_edge_record(&self, data: &[u8]) -> Result<RecordLocation> {
        let needed = 2 + data.len();
        let mut page_num = self.current_e_page.load(Ordering::SeqCst);

        loop {
            let page_id = PageId::new(layout::FILE_EDGES, page_num);
            let has_space = self
                .buffer_pool
                .has_space(page_id, needed)
                .unwrap_or(false);
            if has_space {
                let slot = self
                    .buffer_pool
                    .write_record(page_id, data)
                    .map_err(|e| Error::Storage(format!("Failed to write edge record: {}", e)))?;
                return Ok(RecordLocation::new(layout::FILE_EDGES, page_num, slot));
            }
            page_num += 1;
            let next_id = PageId::new(layout::FILE_EDGES, page_num);
            if self.buffer_pool.has_space(next_id, needed).is_err() {
                let new_id = self
                    .buffer_pool
                    .allocate_page(layout::FILE_EDGES)
                    .map_err(|e| Error::Storage(format!("Failed to allocate edge page: {}", e)))?;
                self.current_e_page
                    .store(new_id.page_num, Ordering::SeqCst);
                let slot = self
                    .buffer_pool
                    .write_record(new_id, data)
                    .map_err(|e| Error::Storage(format!("Failed to write edge record: {}", e)))?;
                return Ok(RecordLocation::new(
                    layout::FILE_EDGES,
                    new_id.page_num,
                    slot,
                ));
            }
        }
    }

    /// Read the adjacency list of a vertex (from the corresponding page in adj.dat).
    fn read_adjacency(&self, vid: VertexId) -> Result<(Vec<EdgeId>, Vec<EdgeId>)> {
        let page_id = PageId::new(layout::FILE_ADJ, vid.0);
        let data = self
            .buffer_pool
            .read_bytes(
                page_id,
                PAGE_HEADER_SIZE,
                layout::PAGE_SIZE - PAGE_HEADER_SIZE,
            )
            .map_err(|e| Error::Storage(format!("Read adjacency page: {}", e)))?;
        decode_adjacency(&data)
            .ok_or_else(|| Error::Storage(format!("Failed to decode adjacency data vid={:?}", vid)))
    }

    /// Write the adjacency list of a vertex.
    fn write_adjacency(
        &self,
        vid: VertexId,
        out_edges: &[EdgeId],
        in_edges: &[EdgeId],
    ) -> Result<()> {
        let page_id = PageId::new(layout::FILE_ADJ, vid.0);
        self.buffer_pool
            .ensure_page_exists(layout::FILE_ADJ, vid.0)
            .map_err(|e| Error::Storage(format!("Ensure adjacency page: {}", e)))?;
        let data = encode_adjacency(out_edges, in_edges);
        self.buffer_pool
            .write_bytes(page_id, PAGE_HEADER_SIZE, &data)
            .map_err(|e| Error::Storage(format!("Write adjacency page: {}", e)))?;
        Ok(())
    }

    /// Register a record in the write set.
    fn track_write(&self, txn_id: u64, location: RecordLocation, op: WriteOp) -> Result<()> {
        if txn_id == 0 {
            return Ok(()); // Auto-commit transactions do not need tracking
        }
        let mut ws = self.write_set.lock().map_err(|_| lock_poisoned())?;
        ws.entry(txn_id).or_default().push(WriteSetEntry { location, op });
        Ok(())
    }

    /// Acquire vertex write lock. Returns conflict error if held by another transaction.
    fn acquire_write_lock_v(&self, vid: VertexId, txn_id: u64) -> Result<()> {
        let mut locks = self.write_locks_v.lock().map_err(|_| lock_poisoned())?;
        if let Some(&existing_txn) = locks.get(&vid.0) {
            if existing_txn != txn_id {
                return Err(Error::Transaction(format!(
                    "write conflict on vertex {:?}: held by txn {}",
                    vid, existing_txn
                )));
            }
            return Ok(());
        }
        locks.insert(vid.0, txn_id);
        Ok(())
    }

    /// Acquire edge write lock. Returns conflict error if held by another transaction.
    fn acquire_write_lock_e(&self, eid: EdgeId, txn_id: u64) -> Result<()> {
        let mut locks = self.write_locks_e.lock().map_err(|_| lock_poisoned())?;
        if let Some(&existing_txn) = locks.get(&eid.0) {
            if existing_txn != txn_id {
                return Err(Error::Transaction(format!(
                    "write conflict on edge {:?}: held by txn {}",
                    eid, existing_txn
                )));
            }
            return Ok(());
        }
        locks.insert(eid.0, txn_id);
        Ok(())
    }

    /// Release all write locks held by the specified transaction.
    fn release_write_locks(&self, txn_id: u64) {
        if let Ok(mut locks) = self.write_locks_v.lock() {
            locks.retain(|_, &mut tid| tid != txn_id);
        }
        if let Ok(mut locks) = self.write_locks_e.lock() {
            locks.retain(|_, &mut tid| tid != txn_id);
        }
    }

    /**
     * @brief: Traverse vertex records along the version chain, returning the first visible version.
     * @param head_loc: Version chain head location (latest version pointed to by the index).
     * @param ctx: Transaction context.
     * @return: Visible vertex record, or None (all versions invisible).
     */
    fn traverse_vertex_chain(
        &self,
        head_loc: RecordLocation,
        ctx: &TxnContext,
    ) -> Result<Option<VertexRecord>> {
        let mut current = head_loc;
        loop {
            let page_id = PageId::new(current.file_tag, current.page_num);
            let data = self
                .buffer_pool
                .read_record(page_id, current.slot)
                .map_err(|e| Error::Storage(format!("Version chain read vertex: {}", e)))?;
            let (mvcc, prev_location, record) = decode_vertex(&data)
                .ok_or_else(|| Error::Storage("Version chain decode vertex failed".into()))?;
            if is_delete_visible(&mvcc, ctx) {
                return Ok(None);
            }
            if is_visible(&mvcc, ctx) {
                return Ok(Some(record));
            }
            match prev_location {
                Some(loc) => current = loc,
                None => return Ok(None),
            }
        }
    }

    /**
     * @brief: Traverse edge records along the version chain, returning the first visible version.
     * @param head_loc: Version chain head location.
     * @param ctx: Transaction context.
     * @return: Visible edge record, or None.
     */
    fn traverse_edge_chain(
        &self,
        head_loc: RecordLocation,
        ctx: &TxnContext,
    ) -> Result<Option<EdgeRecord>> {
        let mut current = head_loc;
        loop {
            let page_id = PageId::new(current.file_tag, current.page_num);
            let data = self
                .buffer_pool
                .read_record(page_id, current.slot)
                .map_err(|e| Error::Storage(format!("Version chain read edge: {}", e)))?;
            let (mvcc, prev_location, record) = decode_edge(&data)
                .ok_or_else(|| Error::Storage("Version chain decode edge failed".into()))?;
            if is_delete_visible(&mvcc, ctx) {
                return Ok(None);
            }
            if is_visible(&mvcc, ctx) {
                return Ok(Some(record));
            }
            match prev_location {
                Some(loc) => current = loc,
                None => return Ok(None),
            }
        }
    }
}

impl Drop for DefaultStorage {
    fn drop(&mut self) {
        let _ = self.sync();
    }
}

fn lock_poisoned() -> Error {
    Error::Storage("Internal lock poisoned".into())
}

fn vertex_passes(v: &VertexRecord, filter: &VertexFilter) -> bool {
    match &filter.labels {
        None => true,
        Some(labels) => v.labels.iter().any(|l| labels.contains(l)),
    }
}

fn edge_passes(e: &EdgeRecord, filter: &EdgeFilter) -> bool {
    match &filter.types {
        None => true,
        Some(types) => types.contains(&e.etype),
    }
}

/**
 * @brief: MVCC visibility determination (Snapshot Isolation).
 *
 * Rules:
 *   1. Records written by this transaction are always visible (read your own writes).
 *   2. Must be committed.
 *   3. created_ts <= ctx.start_ts (snapshot isolation).
 *   4. Not deleted, or delete timestamp > ctx.start_ts.
 */
fn is_visible(mvcc: &MvccHeader, ctx: &TxnContext) -> bool {
    // Read your own writes: uncommitted writes by this transaction are visible to itself
    if mvcc.creator_txn != 0 && mvcc.creator_txn == ctx.txn_id {
        return true;
    }
    // Must be committed
    if !mvcc.is_committed() {
        return false;
    }
    // Snapshot Isolation: created_ts must be before the snapshot time point
    if mvcc.created_ts > ctx.start_ts {
        return false;
    }
    // Delete check: not deleted, or delete time is later than snapshot time
    if mvcc.deleted_ts != 0 && mvcc.deleted_ts <= ctx.start_ts {
        return false;
    }
    true
}

/// WAL log writing: in non-recovery mode (wal is not None), append log entry and fsync
fn log_wal(storage: &DefaultStorage, op: &wal::WalOp) -> Result<()> {
    if let Ok(mut guard) = storage.wal.lock() {
        if let Some(ref mut w) = *guard {
            w.append(op).map_err(|e| Error::Storage(format!("WAL: {}", e)))?;
        }
    }
    Ok(())
}

/// Whether the delete is visible to the current snapshot: once the delete is visible, the version chain is no longer traversed.
fn is_delete_visible(mvcc: &MvccHeader, ctx: &TxnContext) -> bool {
    mvcc.is_committed() && mvcc.deleted_ts != 0 && mvcc.deleted_ts <= ctx.start_ts
}

/// Read (next_vid, next_eid, next_ts, next_txn_id) from meta.dat.
/// Compatible with old format (24 bytes): in this case next_ts and next_txn_id default to 0.
fn read_meta(base_dir: &Path) -> io::Result<(u64, u64, u64, u64)> {
    let path = layout::data_file_path(base_dir, layout::META_FILE);
    let file = File::open(&path)?;
    let metadata = file.metadata()?;
    let file_len = metadata.len() as usize;

    let mut buf = [0u8; layout::META_SIZE];
    let read_len = file_len.min(layout::META_SIZE);
    file.read_at(&mut buf[..read_len], 0)?;

    let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    if magic != layout::META_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "meta.dat magic number mismatch",
        ));
    }

    let next_vid = u64::from_le_bytes([
        buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15],
    ]);
    let next_eid = u64::from_le_bytes([
        buf[16], buf[17], buf[18], buf[19], buf[20], buf[21], buf[22], buf[23],
    ]);

    // New format (>= 24 bytes) includes next_ts and next_txn_id
    let (next_ts, next_txn_id) = if file_len >= layout::META_OFFSET_NEXT_TXN_ID + 8 {
        let ts = u64::from_le_bytes([
            buf[24], buf[25], buf[26], buf[27], buf[28], buf[29], buf[30], buf[31],
        ]);
        let txn = u64::from_le_bytes([
            buf[32], buf[33], buf[34], buf[35], buf[36], buf[37], buf[38], buf[39],
        ]);
        (ts, txn)
    } else {
        (0, 0)
    };

    Ok((next_vid, next_eid, next_ts, next_txn_id))
}

/// Write (next_vid, next_eid, next_ts, next_txn_id) to meta.dat.
fn write_meta(
    base_dir: &Path,
    next_vid: u64,
    next_eid: u64,
    next_ts: u64,
    next_txn_id: u64,
) -> io::Result<()> {
    let path = layout::data_file_path(base_dir, layout::META_FILE);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    let mut buf = [0u8; layout::META_SIZE];
    buf[0..4].copy_from_slice(&layout::META_MAGIC.to_le_bytes());
    buf[4..8].copy_from_slice(&layout::STORAGE_VERSION.to_le_bytes());
    buf[8..16].copy_from_slice(&next_vid.to_le_bytes());
    buf[16..24].copy_from_slice(&next_eid.to_le_bytes());
    buf[24..32].copy_from_slice(&next_ts.to_le_bytes());
    buf[32..40].copy_from_slice(&next_txn_id.to_le_bytes());
    file.write_all_at(&buf, 0)?;
    file.sync_all()?;
    Ok(())
}

impl StorageEngine for DefaultStorage {
    fn get_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<Option<VertexRecord>> {
        let index = self.index.read().map_err(|_| lock_poisoned())?;
        let loc = match index.get_vertex(vid) {
            Some(l) => l,
            None => return Ok(None),
        };
        drop(index);
        self.traverse_vertex_chain(loc, ctx)
    }

    fn scan_vertices(&self, filter: VertexFilter, ctx: &TxnContext) -> Result<VertexIter> {
        // First collect all head locations, then traverse version chain (to avoid cross-lock issues)
        let heads: Vec<RecordLocation> = {
            let index = self.index.read().map_err(|_| lock_poisoned())?;
            index
                .vertex_ids()
                .filter_map(|vid| index.get_vertex(vid))
                .collect()
        };
        let mut records = Vec::new();
        for loc in heads {
            if let Ok(Some(record)) = self.traverse_vertex_chain(loc, ctx) {
                if vertex_passes(&record, &filter) {
                    records.push(record);
                }
            }
        }
        Ok(Box::new(records.into_iter()))
    }

    fn get_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<Option<EdgeRecord>> {
        let index = self.index.read().map_err(|_| lock_poisoned())?;
        let loc = match index.get_edge(eid) {
            Some(l) => l,
            None => return Ok(None),
        };
        drop(index);
        self.traverse_edge_chain(loc, ctx)
    }

    fn out_edges(
        &self,
        vid: VertexId,
        filter: EdgeFilter,
        ctx: &TxnContext,
    ) -> Result<EdgeIter> {
        let (out_eids, _) = self.read_adjacency(vid)?;
        // First collect all head locations, then traverse the version chain
        let heads: Vec<RecordLocation> = {
            let index = self.index.read().map_err(|_| lock_poisoned())?;
            out_eids
                .iter()
                .filter_map(|eid| index.get_edge(*eid))
                .collect()
        };
        let mut records = Vec::new();
        for loc in heads {
            if let Ok(Some(record)) = self.traverse_edge_chain(loc, ctx) {
                if edge_passes(&record, &filter) {
                    records.push(record);
                }
            }
        }
        Ok(Box::new(records.into_iter()))
    }

    fn in_edges(
        &self,
        vid: VertexId,
        filter: EdgeFilter,
        ctx: &TxnContext,
    ) -> Result<EdgeIter> {
        let (_, in_eids) = self.read_adjacency(vid)?;
        let heads: Vec<RecordLocation> = {
            let index = self.index.read().map_err(|_| lock_poisoned())?;
            in_eids
                .iter()
                .filter_map(|eid| index.get_edge(*eid))
                .collect()
        };
        let mut records = Vec::new();
        for loc in heads {
            if let Ok(Some(record)) = self.traverse_edge_chain(loc, ctx) {
                if edge_passes(&record, &filter) {
                    records.push(record);
                }
            }
        }
        Ok(Box::new(records.into_iter()))
    }

    fn index_lookup(
        &self,
        _idx: IndexHandle,
        _key: &IndexKey,
        _ctx: &TxnContext,
    ) -> Result<VertexIter> {
        Err(Error::Unsupported(
            "DefaultStorage has not yet implemented secondary indexes".into(),
        ))
    }

    fn insert_vertex(&self, mut record: VertexRecord, ctx: &TxnContext) -> Result<VertexId> {
        if ctx.read_only {
            return Err(Error::Transaction(
                "read-only transaction cannot insert vertex".into(),
            ));
        }
        // WAL: write log first
        log_wal(self, &wal::WalOp::InsertVertex(record.clone()))?;

        let vid = self.alloc_vid(record.id);
        record.id = vid;

        // Acquire write lock
        self.acquire_write_lock_v(vid, ctx.txn_id)?;

        // Construct MVCC header: uncommitted state, no previous version
        let mvcc = MvccHeader::new(0, 0, ctx.txn_id, 0);
        let data = encode_vertex(&record, &mvcc, None);
        let loc = self.append_vertex_record(&data)?;

        {
            let mut index = self.index.write().map_err(|_| lock_poisoned())?;
            index.insert_vertex(vid, loc);
        }

        let empty_adj: Vec<EdgeId> = Vec::new();
        self.write_adjacency(vid, &empty_adj, &empty_adj)?;

        self.track_write(ctx.txn_id, loc, WriteOp::Insert)?;

        Ok(vid)
    }

    fn insert_edge(&self, mut record: EdgeRecord, ctx: &TxnContext) -> Result<EdgeId> {
        if ctx.read_only {
            return Err(Error::Transaction(
                "read-only transaction cannot insert edge".into(),
            ));
        }
        // WAL: write log first
        log_wal(self, &wal::WalOp::InsertEdge(record.clone()))?;
        {
            let index = self.index.read().map_err(|_| lock_poisoned())?;
            if !index.vertex_index.contains_key(&record.src) {
                return Err(Error::Storage(format!(
                    "insert_edge: src vertex {:?} does not exist",
                    record.src
                )));
            }
            if !index.vertex_index.contains_key(&record.dst) {
                return Err(Error::Storage(format!(
                    "insert_edge: dst vertex {:?} does not exist",
                    record.dst
                )));
            }
        }

        let eid = self.alloc_eid(record.id);
        record.id = eid;

        // Acquire write lock
        self.acquire_write_lock_e(eid, ctx.txn_id)?;

        // Construct MVCC header: uncommitted state, no previous version
        let mvcc = MvccHeader::new(0, 0, ctx.txn_id, 0);
        let data = encode_edge(&record, &mvcc, None);
        let loc = self.append_edge_record(&data)?;

        {
            let mut index = self.index.write().map_err(|_| lock_poisoned())?;
            index.insert_edge(eid, loc);
        }

        let (mut out_edges, in_edges_src) = self.read_adjacency(record.src)?;
        out_edges.push(eid);
        self.write_adjacency(record.src, &out_edges, &in_edges_src)?;

        let (out_edges_dst, mut in_edges) = self.read_adjacency(record.dst)?;
        in_edges.push(eid);
        self.write_adjacency(record.dst, &out_edges_dst, &in_edges)?;

        self.track_write(ctx.txn_id, loc, WriteOp::Insert)?;

        Ok(eid)
    }

    fn update_vertex(&self, record: VertexRecord, ctx: &TxnContext) -> Result<()> {
        if ctx.read_only {
            return Err(Error::Transaction(
                "read-only transaction cannot update vertex".into(),
            ));
        }
        log_wal(self, &wal::WalOp::UpdateVertex(record.clone()))?;

        // Acquire write lock
        self.acquire_write_lock_v(record.id, ctx.txn_id)?;

        // Read current head location
        let old_location = {
            let index = self.index.read().map_err(|_| lock_poisoned())?;
            index
                .get_vertex(record.id)
                .ok_or_else(|| Error::NotFound(format!("update_vertex: {:?} does not exist", record.id)))?
        };

        // Construct new version: prev points to old head
        let mvcc = MvccHeader::new(0, 0, ctx.txn_id, 0);
        let data = encode_vertex(&record, &mvcc, Some(old_location));
        let new_location = self.append_vertex_record(&data)?;

        // Update index to point to new version
        {
            let mut index = self.index.write().map_err(|_| lock_poisoned())?;
            index.insert_vertex(record.id, new_location);
        }

        self.track_write(
            ctx.txn_id,
            new_location,
            WriteOp::Update {
                old_location,
            },
        )?;

        Ok(())
    }

    fn delete_vertex(&self, vid: VertexId, ctx: &TxnContext) -> Result<()> {
        if ctx.read_only {
            return Err(Error::Transaction(
                "read-only transaction cannot delete vertex".into(),
            ));
        }
        log_wal(self, &wal::WalOp::DeleteVertex(vid))?;

        // Acquire write lock
        self.acquire_write_lock_v(vid, ctx.txn_id)?;

        // Read current head location and prev_location, used for abort rollback
        let (head_location, prev_location) = {
            let index = self.index.read().map_err(|_| lock_poisoned())?;
            let loc = index
                .get_vertex(vid)
                .ok_or_else(|| Error::NotFound(format!("delete_vertex: {:?} does not exist", vid)))?;
            let page_id = PageId::new(loc.file_tag, loc.page_num);
            let data = self
                .buffer_pool
                .read_record(page_id, loc.slot)
                .map_err(|e| Error::Storage(format!("delete_vertex read: {}", e)))?;
            let (_, prev_loc, _) = decode_vertex(&data)
                .ok_or_else(|| Error::Storage("delete_vertex decode failed".into()))?;
            (loc, prev_loc)
        };

        // DELETE does not modify the index yet, writes deleted_ts at commit time
        self.track_write(
            ctx.txn_id,
            head_location,
            WriteOp::Delete {
                old_location: prev_location,
            },
        )?;

        Ok(())
    }

    fn update_edge(&self, record: EdgeRecord, ctx: &TxnContext) -> Result<()> {
        if ctx.read_only {
            return Err(Error::Transaction(
                "read-only transaction cannot update edge".into(),
            ));
        }
        log_wal(self, &wal::WalOp::UpdateEdge(record.clone()))?;

        // Acquire write lock
        self.acquire_write_lock_e(record.id, ctx.txn_id)?;

        // Read current head, verify src/dst/etype are immutable
        let old_location = {
            let index = self.index.read().map_err(|_| lock_poisoned())?;
            let loc = index
                .get_edge(record.id)
                .ok_or_else(|| Error::NotFound(format!("update_edge: {:?} does not exist", record.id)))?;
            let page_id = PageId::new(loc.file_tag, loc.page_num);
            let data = self
                .buffer_pool
                .read_record(page_id, loc.slot)
                .map_err(|e| Error::Storage(format!("update_edge read: {}", e)))?;
            let (_, _, existing) = decode_edge(&data)
                .ok_or_else(|| Error::Storage("update_edge decode failed".into()))?;
            if existing.src != record.src
                || existing.dst != record.dst
                || existing.etype != record.etype
            {
                return Err(Error::Storage(
                    "update_edge: Modification of src/dst/etype is not allowed".into(),
                ));
            }
            loc
        };

        // Construct new version: prev points to old head
        let mvcc = MvccHeader::new(0, 0, ctx.txn_id, 0);
        let data = encode_edge(&record, &mvcc, Some(old_location));
        let new_location = self.append_edge_record(&data)?;

        // Update index to point to new version
        {
            let mut index = self.index.write().map_err(|_| lock_poisoned())?;
            index.insert_edge(record.id, new_location);
        }

        self.track_write(
            ctx.txn_id,
            new_location,
            WriteOp::Update {
                old_location,
            },
        )?;

        Ok(())
    }

    fn delete_edge(&self, eid: EdgeId, ctx: &TxnContext) -> Result<()> {
        if ctx.read_only {
            return Err(Error::Transaction(
                "read-only transaction cannot delete edge".into(),
            ));
        }
        log_wal(self, &wal::WalOp::DeleteEdge(eid))?;

        // Acquire write lock
        self.acquire_write_lock_e(eid, ctx.txn_id)?;

        // Read current head location and prev_location
        let (head_location, prev_location) = {
            let index = self.index.read().map_err(|_| lock_poisoned())?;
            let loc = index
                .get_edge(eid)
                .ok_or_else(|| Error::NotFound(format!("delete_edge: {:?} does not exist", eid)))?;
            let page_id = PageId::new(loc.file_tag, loc.page_num);
            let data = self
                .buffer_pool
                .read_record(page_id, loc.slot)
                .map_err(|e| Error::Storage(format!("delete_edge read: {}", e)))?;
            let (_, prev_loc, _) = decode_edge(&data)
                .ok_or_else(|| Error::Storage("delete_edge decode failed".into()))?;
            (loc, prev_loc)
        };

        self.track_write(
            ctx.txn_id,
            head_location,
            WriteOp::Delete {
                old_location: prev_location,
            },
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metagdb_core::common::ids::{EdgeTypeId, LabelId, PropertyKeyId};
    use metagdb_core::common::property::{PropertyMap, PropertyValue};
    use metagdb_core::Error;

    fn make_ctx(txn_id: u64, start_ts: u64) -> TxnContext {
        TxnContext {
            txn_id,
            isolation: metagdb_core::l3_txn::IsolationLevel::SnapshotIsolation,
            read_only: false,
            start_ts,
        }
    }

    fn test_schema() -> Arc<dyn GraphSchema> {
        use metagdb_core::l1_model::{EdgeTypeDescriptor, LabelDescriptor, PropertyKeyDescriptor};
        use std::sync::Arc;

        struct TestSchema {
            labels: Vec<LabelDescriptor>,
            edge_types: Vec<EdgeTypeDescriptor>,
            property_keys: Vec<PropertyKeyDescriptor>,
        }

        impl GraphSchema for TestSchema {
            fn schema_mode(&self) -> metagdb_core::l1_model::SchemaMode {
                metagdb_core::l1_model::SchemaMode::Schemaless
            }
            fn vertex_labels(&self) -> Vec<LabelDescriptor> {
                self.labels.clone()
            }
            fn edge_types(&self) -> Vec<EdgeTypeDescriptor> {
                self.edge_types.clone()
            }
            fn property_keys(&self) -> Vec<PropertyKeyDescriptor> {
                self.property_keys.clone()
            }
            fn label_by_name(&self, name: &str) -> Option<LabelId> {
                self.labels.iter().find(|l| l.name == name).map(|l| l.id)
            }
            fn edge_type_by_name(&self, name: &str) -> Option<EdgeTypeId> {
                self.edge_types
                    .iter()
                    .find(|e| e.name == name)
                    .map(|e| e.id)
            }
            fn property_key_by_name(&self, name: &str) -> Option<PropertyKeyId> {
                self.property_keys
                    .iter()
                    .find(|p| p.name == name)
                    .map(|p| p.id)
            }
            fn label_name(&self, id: LabelId) -> Option<&str> {
                self.labels.iter().find(|l| l.id == id).map(|l| l.name.as_str())
            }
            fn edge_type_name(&self, id: EdgeTypeId) -> Option<&str> {
                self.edge_types
                    .iter()
                    .find(|e| e.id == id)
                    .map(|e| e.name.as_str())
            }
            fn property_key_name(&self, id: PropertyKeyId) -> Option<&str> {
                self.property_keys
                    .iter()
                    .find(|p| p.id == id)
                    .map(|p| p.name.as_str())
            }
        }

        Arc::new(TestSchema {
            labels: vec![
                LabelDescriptor {
                    id: LabelId(1),
                    name: "Person".into(),
                },
                LabelDescriptor {
                    id: LabelId(2),
                    name: "City".into(),
                },
            ],
            edge_types: vec![
                EdgeTypeDescriptor {
                    id: EdgeTypeId(1),
                    name: "KNOWS".into(),
                },
                EdgeTypeDescriptor {
                    id: EdgeTypeId(2),
                    name: "LIVES_IN".into(),
                },
            ],
            property_keys: vec![
                PropertyKeyDescriptor {
                    id: PropertyKeyId(1),
                    name: "name".into(),
                },
                PropertyKeyDescriptor {
                    id: PropertyKeyId(2),
                    name: "age".into(),
                },
            ],
        })
    }

    /**
     * @brief: Persistence test — insert vertex/edge, commit, destroy storage, rebuild and verify data consistency.
     */
    #[test]
    fn test_persistence() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_persistence");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();

        let vid_alice;
        let vid_bob;
        let eid_knows;
        {
            let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

            let write_ctx = make_ctx(1, 1);
            let v_alice = VertexRecord {
                id: VertexId(0),
                labels: vec![LabelId(1)],
                properties: {
                    let mut m = PropertyMap::new();
                    m.insert(PropertyKeyId(1), PropertyValue::Text("Alice".into()));
                    m.insert(PropertyKeyId(2), PropertyValue::Int(30));
                    m
                },
            };
            let v_bob = VertexRecord {
                id: VertexId(0),
                labels: vec![LabelId(1)],
                properties: {
                    let mut m = PropertyMap::new();
                    m.insert(PropertyKeyId(1), PropertyValue::Text("Bob".into()));
                    m.insert(PropertyKeyId(2), PropertyValue::Int(28));
                    m
                },
            };
            vid_alice = storage.insert_vertex(v_alice, &write_ctx).unwrap();
            vid_bob = storage.insert_vertex(v_bob, &write_ctx).unwrap();

            let e = EdgeRecord {
                id: EdgeId(0),
                src: vid_alice,
                dst: vid_bob,
                etype: EdgeTypeId(1),
                properties: PropertyMap::new(),
            };
            eid_knows = storage.insert_edge(e, &write_ctx).unwrap();

            // Commit transaction
            storage.commit_txn(write_ctx.txn_id, 2).unwrap();
        } // storage is dropped

        // Phase 2: reopen storage and verify data
        {
            let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());
            let read_ctx = make_ctx(2, 3); // start_ts=3 > commit_ts=2

            // Verify vertex
            let alice = storage.get_vertex(vid_alice, &read_ctx).unwrap().unwrap();
            assert_eq!(alice.id, vid_alice);
            assert_eq!(
                alice.properties.get(&PropertyKeyId(1)),
                Some(&PropertyValue::Text("Alice".into()))
            );
            assert_eq!(
                alice.properties.get(&PropertyKeyId(2)),
                Some(&PropertyValue::Int(30))
            );

            // Verify out edge
            let out = storage
                .out_edges(vid_alice, EdgeFilter::default(), &read_ctx)
                .unwrap()
                .collect::<Vec<_>>();
            assert_eq!(out.len(), 1);
            assert_eq!(out[0].id, eid_knows);
            assert_eq!(out[0].src, vid_alice);
            assert_eq!(out[0].dst, vid_bob);
        }

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: Post-commit readability test.
     */
    #[test]
    fn test_txn_commit_visible() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_commit_visible");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        let write_ctx = make_ctx(100, 10);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };
        let vid = storage.insert_vertex(v, &write_ctx).unwrap();

        // Before commit: invisible to another transaction
        let other_ctx = make_ctx(200, 20);
        assert!(storage.get_vertex(vid, &other_ctx).unwrap().is_none());

        // Commit
        storage.commit_txn(write_ctx.txn_id, 15).unwrap();

        // After commit: visible to transactions with start_ts >= commit_ts
        let reader_ctx = make_ctx(300, 20);
        assert!(storage.get_vertex(vid, &reader_ctx).unwrap().is_some());

        // Transactions with start_ts < commit_ts cannot see it (snapshot isolation)
        let old_reader = make_ctx(400, 10);
        assert!(storage.get_vertex(vid, &old_reader).unwrap().is_none());

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: Post-abort invisibility test.
     */
    #[test]
    fn test_txn_abort_invisible() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_abort_invisible");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        let write_ctx = make_ctx(100, 10);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };
        let vid = storage.insert_vertex(v, &write_ctx).unwrap();

        // Rollback
        storage.abort_txn(write_ctx.txn_id).unwrap();

        // After rollback: invisible even to own transaction
        assert!(storage.get_vertex(vid, &write_ctx).unwrap().is_none());

        // After rollback: invisible to other transactions
        let reader_ctx = make_ctx(200, 20);
        assert!(storage.get_vertex(vid, &reader_ctx).unwrap().is_none());

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: Read-only transaction rejects writes test.
     */
    #[test]
    fn test_read_only_rejects_write() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_read_only_rejects_write");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        let read_only_ctx = TxnContext {
            txn_id: 1,
            isolation: metagdb_core::l3_txn::IsolationLevel::SnapshotIsolation,
            read_only: true,
            start_ts: 1,
        };

        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };

        let err = storage.insert_vertex(v, &read_only_ctx).unwrap_err();
        match err {
            Error::Transaction(_) => {}
            other => panic!("expected Transaction error, got: {:?}", other),
        }

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: Read your own writes — transaction can see its own uncommitted writes.
     */
    #[test]
    fn test_read_your_own_writes() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_read_own_writes");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        let ctx = make_ctx(100, 10);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };
        let vid = storage.insert_vertex(v, &ctx).unwrap();

        // Same transaction can read uncommitted writes
        assert!(storage.get_vertex(vid, &ctx).unwrap().is_some());

        // Still visible after commit
        storage.commit_txn(ctx.txn_id, 15).unwrap();
        assert!(storage.get_vertex(vid, &ctx).unwrap().is_some());

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: UPDATE visibility — after update, old version visible to old snapshot, invisible to new snapshot.
     */
    #[test]
    fn test_update_visibility() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_update_visibility");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        // Insert vertex and commit
        let insert_ctx = make_ctx(1, 1);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("v1".into()));
                m
            },
        };
        let vid = storage.insert_vertex(v, &insert_ctx).unwrap();
        storage.commit_txn(insert_ctx.txn_id, 2).unwrap();

        // Old snapshot (start_ts=3, before update)
        let old_snapshot = make_ctx(2, 3);

        // Update vertex
        let update_ctx = make_ctx(10, 10);
        let updated_v = VertexRecord {
            id: vid,
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("v2".into()));
                m
            },
        };
        storage.update_vertex(updated_v, &update_ctx).unwrap();

        // Updating transaction can see its own new version
        let self_read = storage.get_vertex(vid, &update_ctx).unwrap().unwrap();
        assert_eq!(
            self_read.properties.get(&PropertyKeyId(1)),
            Some(&PropertyValue::Text("v2".into()))
        );

        // Old snapshot can still see the old version
        let old_read = storage.get_vertex(vid, &old_snapshot).unwrap().unwrap();
        assert_eq!(
            old_read.properties.get(&PropertyKeyId(1)),
            Some(&PropertyValue::Text("v1".into()))
        );

        // Commit update
        storage.commit_txn(update_ctx.txn_id, 20).unwrap();

        // After commit, new snapshot sees new version
        let new_snapshot = make_ctx(20, 30);
        let new_read = storage.get_vertex(vid, &new_snapshot).unwrap().unwrap();
        assert_eq!(
            new_read.properties.get(&PropertyKeyId(1)),
            Some(&PropertyValue::Text("v2".into()))
        );

        // Old snapshot still sees old version
        let old_still = storage.get_vertex(vid, &old_snapshot).unwrap().unwrap();
        assert_eq!(
            old_still.properties.get(&PropertyKeyId(1)),
            Some(&PropertyValue::Text("v1".into()))
        );

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: DELETE visibility — after delete, invisible to new snapshot, still visible to old snapshot.
     */
    #[test]
    fn test_delete_visibility() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_delete_visibility");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        // Insert vertex and commit
        let insert_ctx = make_ctx(1, 1);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };
        let vid = storage.insert_vertex(v, &insert_ctx).unwrap();
        storage.commit_txn(insert_ctx.txn_id, 2).unwrap();

        // Old snapshot
        let old_snapshot = make_ctx(2, 3);

        // Delete vertex
        let delete_ctx = make_ctx(10, 10);
        storage.delete_vertex(vid, &delete_ctx).unwrap();

        // Commit delete
        storage.commit_txn(delete_ctx.txn_id, 20).unwrap();

        // New snapshot cannot see it
        let new_snapshot = make_ctx(20, 30);
        assert!(storage.get_vertex(vid, &new_snapshot).unwrap().is_none());

        // Old snapshot can still see it
        assert!(storage.get_vertex(vid, &old_snapshot).unwrap().is_some());

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: UPDATE -> DELETE -> invisible to new snapshot.
     */
    #[test]
    fn test_update_then_delete_invisible() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_update_then_delete");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        // Insert and commit
        let insert_ctx = make_ctx(1, 1);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("v1".into()));
                m
            },
        };
        let vid = storage.insert_vertex(v, &insert_ctx).unwrap();
        storage.commit_txn(insert_ctx.txn_id, 2).unwrap();

        // Update and commit
        let update_ctx = make_ctx(2, 3);
        let v2 = VertexRecord {
            id: vid,
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("v2".into()));
                m
            },
        };
        storage.update_vertex(v2, &update_ctx).unwrap();
        storage.commit_txn(update_ctx.txn_id, 4).unwrap();

        // Delete and commit
        let delete_ctx = make_ctx(3, 5);
        storage.delete_vertex(vid, &delete_ctx).unwrap();
        storage.commit_txn(delete_ctx.txn_id, 6).unwrap();

        // After delete commit, invisible to new snapshot
        let new_snapshot = make_ctx(4, 10);
        assert!(storage.get_vertex(vid, &new_snapshot).unwrap().is_none());

        // Snapshot before delete still visible
        let old_snapshot = make_ctx(5, 5);
        assert!(storage.get_vertex(vid, &old_snapshot).unwrap().is_some());

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: Conflict detection — when two transactions concurrently update the same vertex, the second should return a conflict error.
     */
    #[test]
    fn test_write_conflict() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_write_conflict");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        // Insert vertex and commit
        let insert_ctx = make_ctx(1, 1);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };
        let vid = storage.insert_vertex(v, &insert_ctx).unwrap();
        storage.commit_txn(insert_ctx.txn_id, 2).unwrap();

        // Transaction A updates vertex (uncommitted)
        let txn_a = make_ctx(10, 10);
        let v_a = VertexRecord {
            id: vid,
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("A".into()));
                m
            },
        };
        storage.update_vertex(v_a, &txn_a).unwrap();

        // Transaction B attempts to update the same vertex → should return conflict error
        let txn_b = make_ctx(20, 20);
        let v_b = VertexRecord {
            id: vid,
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("B".into()));
                m
            },
        };
        let err = storage.update_vertex(v_b, &txn_b).unwrap_err();
        match err {
            Error::Transaction(_) => {}
            other => panic!("expected Transaction error, got: {:?}", other),
        }

        // After transaction A commits, transaction B retry should succeed
        storage.commit_txn(txn_a.txn_id, 15).unwrap();
        let txn_b2 = make_ctx(30, 30);
        let v_b2 = VertexRecord {
            id: vid,
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("B".into()));
                m
            },
        };
        storage.update_vertex(v_b2, &txn_b2).unwrap();

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: Rollback consistency — after UPDATE/DELETE rollback, read path can fall back to old version.
     */
    #[test]
    fn test_update_rollback() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_update_rollback");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        // Insert and commit
        let insert_ctx = make_ctx(1, 1);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("original".into()));
                m
            },
        };
        let vid = storage.insert_vertex(v, &insert_ctx).unwrap();
        storage.commit_txn(insert_ctx.txn_id, 2).unwrap();

        // Rollback after update
        let update_ctx = make_ctx(10, 10);
        let updated_v = VertexRecord {
            id: vid,
            labels: vec![LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(PropertyKeyId(1), PropertyValue::Text("updated".into()));
                m
            },
        };
        storage.update_vertex(updated_v, &update_ctx).unwrap();
        storage.abort_txn(update_ctx.txn_id).unwrap();

        // After rollback, read path should fall back to original version
        let read_ctx = make_ctx(20, 20);
        let result = storage.get_vertex(vid, &read_ctx).unwrap().unwrap();
        assert_eq!(
            result.properties.get(&PropertyKeyId(1)),
            Some(&PropertyValue::Text("original".into()))
        );

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: DELETE rollback — record still visible after delete rollback.
     */
    #[test]
    fn test_delete_rollback() {
        let tmp_dir = std::env::temp_dir().join("metagdb_test_delete_rollback");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        // Insert and commit
        let insert_ctx = make_ctx(1, 1);
        let v = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };
        let vid = storage.insert_vertex(v, &insert_ctx).unwrap();
        storage.commit_txn(insert_ctx.txn_id, 2).unwrap();

        // Rollback after delete
        let delete_ctx = make_ctx(10, 10);
        storage.delete_vertex(vid, &delete_ctx).unwrap();
        storage.abort_txn(delete_ctx.txn_id).unwrap();

        // Record still visible after rollback
        let read_ctx = make_ctx(20, 20);
        assert!(storage.get_vertex(vid, &read_ctx).unwrap().is_some());

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    /**
     * @brief: update_edge Modification of src/dst/etype is not allowed.
     */
    #[test]
    fn test_update_edge_structural_validation() {
        let tmp_dir =
            std::env::temp_dir().join("metagdb_test_update_edge_structural");
        let _ = std::fs::remove_dir_all(&tmp_dir);

        let schema = test_schema();
        let storage = DefaultStorage::with_base_dir(schema.clone(), tmp_dir.clone());

        // Insert two vertices and an edge
        let ctx = make_ctx(1, 1);
        let v1 = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };
        let v2 = VertexRecord {
            id: VertexId(0),
            labels: vec![LabelId(1)],
            properties: PropertyMap::new(),
        };
        let vid1 = storage.insert_vertex(v1, &ctx).unwrap();
        let vid2 = storage.insert_vertex(v2, &ctx).unwrap();
        let e = EdgeRecord {
            id: EdgeId(0),
            src: vid1,
            dst: vid2,
            etype: EdgeTypeId(1),
            properties: PropertyMap::new(),
        };
        let eid = storage.insert_edge(e, &ctx).unwrap();
        storage.commit_txn(ctx.txn_id, 2).unwrap();

        // Attempt to modify src → should error
        let bad_ctx = make_ctx(10, 10);
        let bad_edge = EdgeRecord {
            id: eid,
            src: vid2, // Modify src
            dst: vid2,
            etype: EdgeTypeId(1),
            properties: PropertyMap::new(),
        };
        let err = storage.update_edge(bad_edge, &bad_ctx).unwrap_err();
        match err {
            Error::Storage(ref msg) if msg.contains("src/dst/etype") => {}
            other => panic!("expected Storage error about src/dst/etype, got: {:?}", other),
        }

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
}
