/**
 * @brief: WAL (Write-Ahead Logging) module, providing write-ahead logging and crash recovery capabilities.
 *
 * Design: Redo Log + Sync truncation.
 * - Before each data modification, the operation record is first appended to the WAL file and fsynced
 * - After each sync(), the WAL is truncated (all data has been safely flushed to disk)
 * - On startup, if the WAL is non-empty, entries are replayed one by one to restore unflushed modifications
 */

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use metagdb_core::common::ids::{EdgeId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};

/// WAL file magic: "WAL1"
const WAL_MAGIC: u32 = 0x5741_4C31;

/// WAL operation type tags
const OP_INSERT_VERTEX: u8 = 0;
const OP_UPDATE_VERTEX: u8 = 1;
const OP_DELETE_VERTEX: u8 = 2;
const OP_INSERT_EDGE: u8 = 3;
const OP_UPDATE_EDGE: u8 = 4;
const OP_DELETE_EDGE: u8 = 5;

/**
 * @brief: WAL log entry, describing a single data modification operation.
 */
#[derive(Debug, Clone)]
pub enum WalOp {
    InsertVertex(VertexRecord),
    UpdateVertex(VertexRecord),
    DeleteVertex(VertexId),
    InsertEdge(EdgeRecord),
    UpdateEdge(EdgeRecord),
    DeleteEdge(EdgeId),
}

// ============================================================
// Serialization
// ============================================================

/// Compute a simple checksum (XOR + accumulate)
fn checksum(data: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    for &b in data {
        sum = sum.wrapping_mul(31).wrapping_add(b as u32);
    }
    sum
}

fn serialize_property_map(map: &PropertyMap, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&(map.len() as u32).to_le_bytes());
    for (k, v) in map.iter() {
        buf.extend_from_slice(&k.0.to_le_bytes());
        serialize_property_value(v, buf);
    }
}

fn serialize_property_value(v: &PropertyValue, buf: &mut Vec<u8>) {
    match v {
        PropertyValue::Null => buf.push(0),
        PropertyValue::Bool(b) => { buf.push(1); buf.push(*b as u8); }
        PropertyValue::Int(i) => { buf.push(2); buf.extend_from_slice(&i.to_le_bytes()); }
        PropertyValue::Float(f) => { buf.push(3); buf.extend_from_slice(&f.to_le_bytes()); }
        PropertyValue::Text(s) => {
            buf.push(4);
            buf.extend_from_slice(&(s.len() as u32).to_le_bytes());
            buf.extend_from_slice(s.as_bytes());
        }
    }
}

fn deserialize_property_map(data: &[u8], pos: &mut usize) -> io::Result<PropertyMap> {
    let len = read_u32(data, pos)? as usize;
    let mut map = PropertyMap::new();
    for _ in 0..len {
        let key_id = metagdb_core::common::ids::PropertyKeyId(read_u32(data, pos)?);
        let val = deserialize_property_value(data, pos)?;
        map.insert(key_id, val);
    }
    Ok(map)
}

fn deserialize_property_value(data: &[u8], pos: &mut usize) -> io::Result<PropertyValue> {
    let tag = data[*pos]; *pos += 1;
    match tag {
        0 => Ok(PropertyValue::Null),
        1 => { let b = data[*pos] != 0; *pos += 1; Ok(PropertyValue::Bool(b)) }
        2 => { let i = read_i64(data, pos)?; Ok(PropertyValue::Int(i)) }
        3 => { let f = read_f64(data, pos)?; Ok(PropertyValue::Float(f)) }
        4 => {
            let len = read_u32(data, pos)? as usize;
            let s = String::from_utf8(data[*pos..*pos + len].to_vec())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            *pos += len;
            Ok(PropertyValue::Text(s))
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown property value type")),
    }
}

fn read_u32(data: &[u8], pos: &mut usize) -> io::Result<u32> {
    if *pos + 4 > data.len() { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF")); }
    let v = u32::from_le_bytes([data[*pos], data[*pos+1], data[*pos+2], data[*pos+3]]);
    *pos += 4;
    Ok(v)
}

fn read_i64(data: &[u8], pos: &mut usize) -> io::Result<i64> {
    if *pos + 8 > data.len() { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF")); }
    let v = i64::from_le_bytes([
        data[*pos], data[*pos+1], data[*pos+2], data[*pos+3],
        data[*pos+4], data[*pos+5], data[*pos+6], data[*pos+7],
    ]);
    *pos += 8;
    Ok(v)
}

fn read_f64(data: &[u8], pos: &mut usize) -> io::Result<f64> {
    if *pos + 8 > data.len() { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF")); }
    let v = f64::from_le_bytes([
        data[*pos], data[*pos+1], data[*pos+2], data[*pos+3],
        data[*pos+4], data[*pos+5], data[*pos+6], data[*pos+7],
    ]);
    *pos += 8;
    Ok(v)
}

fn read_u64(data: &[u8], pos: &mut usize) -> io::Result<u64> {
    if *pos + 8 > data.len() { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF")); }
    let v = u64::from_le_bytes([
        data[*pos], data[*pos+1], data[*pos+2], data[*pos+3],
        data[*pos+4], data[*pos+5], data[*pos+6], data[*pos+7],
    ]);
    *pos += 8;
    Ok(v)
}

fn write_u32(v: u32, buf: &mut Vec<u8>) { buf.extend_from_slice(&v.to_le_bytes()); }
fn write_u64(v: u64, buf: &mut Vec<u8>) { buf.extend_from_slice(&v.to_le_bytes()); }

fn serialize_vertex(rec: &VertexRecord, buf: &mut Vec<u8>) {
    write_u64(rec.id.0, buf);
    buf.extend_from_slice(&(rec.labels.len() as u32).to_le_bytes());
    for l in &rec.labels { write_u32(l.0, buf); }
    serialize_property_map(&rec.properties, buf);
}

fn deserialize_vertex(data: &[u8], pos: &mut usize) -> io::Result<VertexRecord> {
    let id = VertexId(read_u64(data, pos)?);
    let label_count = read_u32(data, pos)? as usize;
    let mut labels = Vec::with_capacity(label_count);
    for _ in 0..label_count {
        labels.push(metagdb_core::common::ids::LabelId(read_u32(data, pos)?));
    }
    let properties = deserialize_property_map(data, pos)?;
    Ok(VertexRecord { id, labels, properties })
}

fn serialize_edge(rec: &EdgeRecord, buf: &mut Vec<u8>) {
    write_u64(rec.id.0, buf);
    write_u64(rec.src.0, buf);
    write_u64(rec.dst.0, buf);
    write_u32(rec.etype.0, buf);
    serialize_property_map(&rec.properties, buf);
}

fn deserialize_edge(data: &[u8], pos: &mut usize) -> io::Result<EdgeRecord> {
    let id = EdgeId(read_u64(data, pos)?);
    let src = VertexId(read_u64(data, pos)?);
    let dst = VertexId(read_u64(data, pos)?);
    let etype = metagdb_core::common::ids::EdgeTypeId(read_u32(data, pos)?);
    let properties = deserialize_property_map(data, pos)?;
    Ok(EdgeRecord { id, src, dst, etype, properties })
}

fn serialize_op(op: &WalOp, buf: &mut Vec<u8>) {
    match op {
        WalOp::InsertVertex(r) => { buf.push(OP_INSERT_VERTEX); serialize_vertex(r, buf); }
        WalOp::UpdateVertex(r) => { buf.push(OP_UPDATE_VERTEX); serialize_vertex(r, buf); }
        WalOp::DeleteVertex(vid) => { buf.push(OP_DELETE_VERTEX); write_u64(vid.0, buf); }
        WalOp::InsertEdge(r) => { buf.push(OP_INSERT_EDGE); serialize_edge(r, buf); }
        WalOp::UpdateEdge(r) => { buf.push(OP_UPDATE_EDGE); serialize_edge(r, buf); }
        WalOp::DeleteEdge(eid) => { buf.push(OP_DELETE_EDGE); write_u64(eid.0, buf); }
    }
}

fn deserialize_op(data: &[u8], pos: &mut usize) -> io::Result<WalOp> {
    let op_type = data[*pos]; *pos += 1;
    match op_type {
        OP_INSERT_VERTEX => Ok(WalOp::InsertVertex(deserialize_vertex(data, pos)?)),
        OP_UPDATE_VERTEX => Ok(WalOp::UpdateVertex(deserialize_vertex(data, pos)?)),
        OP_DELETE_VERTEX => Ok(WalOp::DeleteVertex(VertexId(read_u64(data, pos)?))),
        OP_INSERT_EDGE => Ok(WalOp::InsertEdge(deserialize_edge(data, pos)?)),
        OP_UPDATE_EDGE => Ok(WalOp::UpdateEdge(deserialize_edge(data, pos)?)),
        OP_DELETE_EDGE => Ok(WalOp::DeleteEdge(EdgeId(read_u64(data, pos)?))),
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown WAL operation type")),
    }
}

// ============================================================
// WAL Writer
// ============================================================

/**
 * @brief: WAL writer, responsible for appending log entries to wal.dat in append mode.
 */
pub struct WalWriter {
    file: File,
}

impl WalWriter {
    /**
     * @brief: Create or open a WAL file (append mode).
     * @param base_dir: Data directory.
     * @return: WalWriter instance.
     */
    pub fn open(base_dir: &Path) -> io::Result<Self> {
        let path = base_dir.join(super::layout::WAL_FILE);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        // Write magic (new file only)
        let meta = file.metadata()?;
        if meta.len() == 0 {
            let mut f = OpenOptions::new().write(true).open(&path)?;
            f.write_all(&WAL_MAGIC.to_le_bytes())?;
            f.sync_all()?;
        }
        Ok(Self { file })
    }

    /**
     * @brief: Append a log entry and fsync.
     * @param op: Operation record.
     */
    pub fn append(&mut self, op: &WalOp) -> io::Result<()> {
        let mut buf = Vec::new();
        serialize_op(op, &mut buf);

        // Write format: [payload_len: u32][payload][checksum: u32]
        let payload_len = buf.len() as u32;
        let cs = checksum(&buf);

        self.file.write_all(&payload_len.to_le_bytes())?;
        self.file.write_all(&buf)?;
        self.file.write_all(&cs.to_le_bytes())?;
        self.file.sync_all()?;
        Ok(())
    }

    /**
     * @brief: Truncate WAL (called after sync).
     */
    pub fn truncate(&mut self) -> io::Result<()> {
        self.file.set_len(0)?;
        self.file.write_all(&WAL_MAGIC.to_le_bytes())?;
        self.file.sync_all()?;
        // Reopen to reset internal offset
        Ok(())
    }

    /**
     * @brief: Check if WAL is empty (only contains magic).
     */
    pub fn is_empty(&self) -> io::Result<bool> {
        let meta = self.file.metadata()?;
        Ok(meta.len() <= 4)
    }
}

// ============================================================
// WAL Read and Replay
// ============================================================

/**
 * @brief: Read all log entries from the WAL file.
 * @param base_dir: Data directory.
 * @return: List of log entries.
 */
pub fn read_wal_entries(base_dir: &Path) -> io::Result<Vec<WalOp>> {
    let path = base_dir.join(super::layout::WAL_FILE);
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(&path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data.len() < 4 {
        return Ok(Vec::new());
    }

    // Skip magic
    let mut pos: usize = 4;
    let mut entries = Vec::new();

    while pos + 4 <= data.len() {
        let payload_len = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]) as usize;
        pos += 4;

        if pos + payload_len + 4 > data.len() {
            break; // Incomplete entry
        }

        let payload = &data[pos..pos + payload_len];
        pos += payload_len;

        let stored_cs = u32::from_le_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]);
        pos += 4;

        let actual_cs = checksum(payload);
        if stored_cs != actual_cs {
            break; // Checksum failed, subsequent entries may also be incomplete
        }

        let mut p: usize = 0;
        match deserialize_op(payload, &mut p) {
            Ok(op) => entries.push(op),
            Err(_) => break,
        }
    }

    Ok(entries)
}

/**
 * @brief: Delete the WAL file (called after recovery completes).
 * @param base_dir: Data directory.
 */
pub fn remove_wal(base_dir: &Path) -> io::Result<()> {
    let path = base_dir.join(super::layout::WAL_FILE);
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wal_roundtrip() {
        let tmp = std::env::temp_dir().join("metagdb_wal_test");
        let _ = std::fs::create_dir_all(&tmp);

        let mut writer = WalWriter::open(&tmp).unwrap();

        let rec = VertexRecord {
            id: VertexId(42),
            labels: vec![metagdb_core::common::ids::LabelId(1)],
            properties: {
                let mut m = PropertyMap::new();
                m.insert(metagdb_core::common::ids::PropertyKeyId(1), PropertyValue::Text("test".into()));
                m
            },
        };
        writer.append(&WalOp::InsertVertex(rec.clone())).unwrap();
        writer.append(&WalOp::DeleteVertex(VertexId(99))).unwrap();

        let entries = read_wal_entries(&tmp).unwrap();
        assert_eq!(entries.len(), 2);
        match &entries[0] {
            WalOp::InsertVertex(r) => assert_eq!(r.id.0, 42),
            _ => panic!("expected InsertVertex"),
        }

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
