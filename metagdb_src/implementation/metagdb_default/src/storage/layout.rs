/**
 * @brief: File layout and metadata constant definitions.
 *
 * Defines compile-time constants such as page size, file paths, magic numbers, and in-page offsets for disk storage.
 */

use std::path::{Path, PathBuf};

/// Page size (4KB).
pub const PAGE_SIZE: usize = 4096;

/// Page header size (bytes), located at the start of each page.
pub const PAGE_HEADER_SIZE: usize = 16;

/// Maximum bytes per page available for storing record data (including 2-byte record length prefix).
#[allow(dead_code)]
pub const MAX_RECORD_DATA: usize = PAGE_SIZE - PAGE_HEADER_SIZE - 2;

/// meta.dat file magic: "MGDB"
pub const META_MAGIC: u32 = 0x4D47_4442;

/// Current storage format version number.
pub const STORAGE_VERSION: u32 = 2;

/// Data file tags, used for file identification in PageId.
pub const FILE_VERTICES: u8 = 0;
pub const FILE_EDGES: u8 = 1;
pub const FILE_ADJ: u8 = 2;

/// Data file names.
pub const META_FILE: &str = "meta.dat";
pub const VERTICES_FILE: &str = "vertices.dat";
pub const EDGES_FILE: &str = "edges.dat";
pub const ADJ_FILE: &str = "adj.dat";
pub const INDEX_FILE: &str = "index.dat";
/// WAL (Write-Ahead Log) file name
#[allow(dead_code)]
pub const WAL_FILE: &str = "wal.dat";

/// meta.dat file size (bytes): magic(4) + version(4) + next_vid(8) + next_eid(8) + next_ts(8) + next_txn_id(8) = 40
pub const META_SIZE: usize = 40;

/// meta.dat field offsets (for documentation purposes)
#[allow(dead_code)]
pub const META_OFFSET_MAGIC: usize = 0;
#[allow(dead_code)]
pub const META_OFFSET_VERSION: usize = 4;
#[allow(dead_code)]
pub const META_OFFSET_NEXT_VID: usize = 8;
#[allow(dead_code)]
pub const META_OFFSET_NEXT_EID: usize = 16;
#[allow(dead_code)]
pub const META_OFFSET_NEXT_TS: usize = 24;
/// Byte offset of the next_txn_id field in meta.dat
pub const META_OFFSET_NEXT_TXN_ID: usize = 32;

/// Old version meta.dat size (before version 1, no next_ts/next_txn_id).
#[allow(dead_code)]
const META_SIZE_V1: usize = 24;

/// Base directory for data file storage.
pub const DATA_DIR: &str = "data/gdb";

/// Page header field offsets.
pub const HEADER_OFFSET_RECORD_COUNT: usize = 0;
pub const HEADER_OFFSET_FREE_OFFSET: usize = 2;
pub const HEADER_OFFSET_FLAGS: usize = 4;

/// Get the full path of a data file.
pub fn data_file_path(base_dir: &Path, filename: &str) -> PathBuf {
    base_dir.join(filename)
}
