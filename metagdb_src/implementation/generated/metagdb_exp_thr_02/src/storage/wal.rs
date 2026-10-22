//! Write-ahead log used for crash recovery.
//!
//! The WAL is *distinct* from the append-only record segment: the segment holds
//! the immutable physical records that queries read, while the WAL holds the
//! commit markers that decide which of those records are durable.  A commit
//! appends its physical records to the segment, flushes it, then appends a WAL
//! commit entry and `fsync`s the WAL.  Because the entry carries the encoded
//! records, recovery can re-materialise a segment whose trailing writes were
//! lost, in addition to discarding transactions that never committed.

use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::unix::fs::FileExt;
use std::path::Path;

use metagdb_core::{Error, Result};

use super::record::{crc32, PhysicalRecord};

const ENTRY_CHECKPOINT: u8 = 1;
const ENTRY_COMMIT: u8 = 2;
const ENTRY_ABORT: u8 = 3;

/// First reservation made when the log starts to fill.  Kept small so a tiny
/// database does not pay for a large reservation in disk size.
/// Smallest headroom kept above the required size.
const MIN_HEADROOM: u64 = 64 * 1024;
/// Chunk used to zero-fill a reservation without a large temporary buffer.
const ZERO_CHUNK: usize = 1024 * 1024;

/// One decoded WAL entry.
#[derive(Debug, Clone)]
pub enum WalEntry {
    /// Baseline established by bulk import: every segment record written before
    /// `segment_len` is committed at `commit_ts`.
    Checkpoint {
        commit_ts: u64,
        next_vid: u64,
        next_eid: u64,
        segment_len: u64,
    },
    Commit {
        txn_id: u64,
        commit_ts: u64,
        next_vid: u64,
        next_eid: u64,
        records: Vec<PhysicalRecord>,
    },
    Abort {
        txn_id: u64,
    },
}

/// Append-only WAL file handle.
///
/// The log keeps a *reservation*: a region of the file whose blocks are already
/// allocated, so a commit changes neither the file size nor its block map.
/// Each commit writes its frame into the reservation with `pwrite` and then
/// `fdatasync`s, which leaves the sync with no metadata transaction to commit.
/// A reservation is taken lazily, grown geometrically, and its unused tail is
/// never truncated.
pub struct Wal {
    file: File,
    /// Offset where the next entry is written: the end of the last valid entry,
    /// and the only part of the file that is meaningful.
    logical_len: u64,
    /// Bytes of the file whose blocks are already allocated.
    reserved_len: u64,
}

impl Wal {
    /// Open (creating if needed) the WAL at `path`.
    pub fn open(path: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)
            .map_err(io_error)?;
        let file_len = file.metadata().map_err(io_error)?.len();
        let (_, logical_len) = scan(path)?;
        Ok(Self {
            file,
            logical_len,
            reserved_len: file_len,
        })
    }

    /// Read every complete entry from a log file on disk.
    pub fn read_all(path: &Path) -> Result<Vec<WalEntry>> {
        Ok(scan(path)?.0)
    }

    /// Append a commit entry, encoding the records directly from the slice so
    /// the caller does not have to clone them.
    pub fn append_commit(
        &mut self,
        txn_id: u64,
        commit_ts: u64,
        next_vid: u64,
        next_eid: u64,
        records: &[PhysicalRecord],
    ) -> Result<()> {
        let payload = encode_commit(txn_id, commit_ts, next_vid, next_eid, records);
        self.write_framed(&payload)
    }

    /// Append one entry and make it durable.
    pub fn append(&mut self, entry: &WalEntry) -> Result<()> {
        let payload = encode_entry(entry);
        self.write_framed(&payload)
    }

    fn write_framed(&mut self, payload: &[u8]) -> Result<()> {
        let mut framed = Vec::with_capacity(payload.len() + 8);
        framed.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        framed.extend_from_slice(&crc32(payload).to_le_bytes());
        framed.extend_from_slice(payload);
        self.reserve(framed.len() as u64)?;
        self.file
            .write_all_at(&framed, self.logical_len)
            .map_err(io_error)?;
        self.logical_len += framed.len() as u64;
        self.file.sync_data().map_err(io_error)?;
        Ok(())
    }

    /// Make sure `need` more bytes fit inside the allocated reservation.
    fn reserve(&mut self, need: u64) -> Result<()> {
        let required = self.logical_len + need;
        if required <= self.reserved_len {
            return Ok(());
        }
        let target = self.growth_target(required);
        // Zero-filling both extends the file and allocates its blocks, so the
        // per-commit path never allocates.  The reservation is synced once here
        // and amortised across every commit it covers.
        let zeros = vec![0u8; ZERO_CHUNK.min((target - self.reserved_len) as usize)];
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

    /// Pick the next reservation size for `required` bytes.
    ///
    /// A reservation covers `required` plus a headroom of 6.25% (at least
    /// [`MIN_HEADROOM`]), so the next commits find room instead of reserving
    /// again while the on-disk footprint stays within a few percent of the bytes
    /// actually written.
    fn growth_target(&self, required: u64) -> u64 {
        required + (required / 16).max(MIN_HEADROOM)
    }

    /// Meaningful bytes in the log, excluding the unused reservation tail.
    pub fn len(&self) -> Result<u64> {
        Ok(self.logical_len)
    }

    /// Bytes of the file whose blocks are already allocated.
    pub fn reserved_len(&self) -> u64 {
        self.reserved_len
    }
}

/// Parse a log file, returning its entries and the offset where parsing stopped.
///
/// A zeroed reservation tail parses as a frame of length zero, which is how the
/// reader tells unused reserved space from a torn entry without an out-of-band
/// marker.
fn scan(path: &Path) -> Result<(Vec<WalEntry>, u64)> {
    if !path.exists() {
        return Ok((Vec::new(), 0));
    }
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(io_error)?
        .read_to_end(&mut bytes)
        .map_err(io_error)?;
    let mut entries = Vec::new();
    let mut cursor = 0usize;
    while cursor + 8 <= bytes.len() {
        let len = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        let checksum = u32::from_le_bytes(bytes[cursor + 4..cursor + 8].try_into().unwrap());
        if len == 0 {
            break;
        }
        let start = cursor + 8;
        let Some(payload) = bytes.get(start..start + len) else {
            break;
        };
        if crc32(payload) != checksum {
            break;
        }
        match decode_entry(payload) {
            Some(entry) => entries.push(entry),
            None => break,
        }
        cursor = start + len;
    }
    Ok((entries, cursor as u64))
}
fn encode_commit(
    txn_id: u64,
    commit_ts: u64,
    next_vid: u64,
    next_eid: u64,
    records: &[PhysicalRecord],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(ENTRY_COMMIT);
    out.extend_from_slice(&txn_id.to_le_bytes());
    out.extend_from_slice(&commit_ts.to_le_bytes());
    out.extend_from_slice(&next_vid.to_le_bytes());
    out.extend_from_slice(&next_eid.to_le_bytes());
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());
    for record in records {
        let encoded = record.encode();
        out.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
        out.extend_from_slice(&encoded);
    }
    out
}

fn encode_entry(entry: &WalEntry) -> Vec<u8> {
    let mut out = Vec::new();
    match entry {
        WalEntry::Checkpoint {
            commit_ts,
            next_vid,
            next_eid,
            segment_len,
        } => {
            out.push(ENTRY_CHECKPOINT);
            out.extend_from_slice(&commit_ts.to_le_bytes());
            out.extend_from_slice(&next_vid.to_le_bytes());
            out.extend_from_slice(&next_eid.to_le_bytes());
            out.extend_from_slice(&segment_len.to_le_bytes());
        }
        WalEntry::Commit {
            txn_id,
            commit_ts,
            next_vid,
            next_eid,
            records,
        } => {
            out.push(ENTRY_COMMIT);
            out.extend_from_slice(&txn_id.to_le_bytes());
            out.extend_from_slice(&commit_ts.to_le_bytes());
            out.extend_from_slice(&next_vid.to_le_bytes());
            out.extend_from_slice(&next_eid.to_le_bytes());
            out.extend_from_slice(&(records.len() as u32).to_le_bytes());
            for record in records {
                let encoded = record.encode();
                out.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
                out.extend_from_slice(&encoded);
            }
        }
        WalEntry::Abort { txn_id } => {
            out.push(ENTRY_ABORT);
            out.extend_from_slice(&txn_id.to_le_bytes());
        }
    }
    out
}

fn decode_entry(payload: &[u8]) -> Option<WalEntry> {
    let tag = *payload.first()?;
    let mut cursor = Cursor::new(payload, 1);
    match tag {
        ENTRY_CHECKPOINT => Some(WalEntry::Checkpoint {
            commit_ts: cursor.u64()?,
            next_vid: cursor.u64()?,
            next_eid: cursor.u64()?,
            segment_len: cursor.u64()?,
        }),
        ENTRY_COMMIT => {
            let txn_id = cursor.u64()?;
            let commit_ts = cursor.u64()?;
            let next_vid = cursor.u64()?;
            let next_eid = cursor.u64()?;
            let count = cursor.u32()? as usize;
            let mut records = Vec::with_capacity(count.min(4096));
            for _ in 0..count {
                let len = cursor.u32()? as usize;
                let bytes = cursor.take(len)?;
                records.push(PhysicalRecord::decode(bytes, 0)?);
            }
            Some(WalEntry::Commit {
                txn_id,
                commit_ts,
                next_vid,
                next_eid,
                records,
            })
        }
        ENTRY_ABORT => Some(WalEntry::Abort {
            txn_id: cursor.u64()?,
        }),
        _ => None,
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8], position: usize) -> Self {
        Self { bytes, position }
    }

    fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        let slice = self.bytes.get(self.position..self.position + len)?;
        self.position += len;
        Some(slice)
    }

    fn u32(&mut self) -> Option<u32> {
        self.take(4).map(|v| u32::from_le_bytes(v.try_into().unwrap()))
    }

    fn u64(&mut self) -> Option<u64> {
        self.take(8).map(|v| u64::from_le_bytes(v.try_into().unwrap()))
    }
}

fn io_error(error: std::io::Error) -> Error {
    Error::Storage(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::record::{RecordBody, RecordKind};
    use metagdb_core::common::ids::{LabelId, VertexId};
    use metagdb_core::l2_storage::VertexRecord;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("metagdb_thr02_wal_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn entries_round_trip_and_torn_tail_is_ignored() {
        let dir = temp_dir("roundtrip");
        let path = dir.join("wal.log");
        let record = PhysicalRecord {
            offset: 1,
            kind: RecordKind::Vertex,
            logical_id: 4,
            begin_ts: 2,
            prev_offset: 0,
            txn_id: 7,
            body: Some(RecordBody::Vertex(VertexRecord {
                id: VertexId(4),
                labels: vec![LabelId(1)],
                properties: Default::default(),
            })),
        };
        {
            let mut wal = Wal::open(&path).unwrap();
            wal.append(&WalEntry::Checkpoint {
                commit_ts: 1,
                next_vid: 5,
                next_eid: 1,
                segment_len: 0,
            })
            .unwrap();
            wal.append(&WalEntry::Commit {
                txn_id: 7,
                commit_ts: 2,
                next_vid: 6,
                next_eid: 1,
                records: vec![record],
            })
            .unwrap();
            wal.append(&WalEntry::Abort { txn_id: 8 }).unwrap();
        }
        let entries = Wal::read_all(&path).unwrap();
        assert_eq!(entries.len(), 3);

        // Truncate mid-entry: the torn tail must be dropped, not misread.  The
        // file ends with an unused reservation, so the cut has to land inside
        // the last entry rather than in the reserved tail.
        let end = {
            let wal = Wal::open(&path).unwrap();
            wal.len().unwrap() as usize
        };
        // The last frame is `[len u32][crc u32][tag u8][txn_id u64]`.
        let abort_frame = 8 + 1 + 8;
        let frame_start = end - abort_frame;
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(bytes[frame_start + 8], ENTRY_ABORT);
        let cut = frame_start + 2;
        std::fs::write(&path, &bytes[..cut]).unwrap();
        let entries = Wal::read_all(&path).unwrap();
        assert_eq!(entries.len(), 2);
    }
}
