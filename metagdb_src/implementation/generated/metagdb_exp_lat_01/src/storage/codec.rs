//! Binary codec for persistent graph records, checkpoints, and the write-ahead
//! log. Everything is little-endian and length-prefixed so a torn tail can be
//! detected and truncated safely by the WAL reader.

use std::collections::BTreeMap;

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
use metagdb_core::Result;

pub const SNAPSHOT_MAGIC: &[u8; 8] = b"MGSNAP01";
pub const WAL_MAGIC: &[u8; 8] = b"MGWAL001";

// ---------------------------------------------------------------------------
// Primitive writers
// ---------------------------------------------------------------------------

pub fn put_u8(out: &mut Vec<u8>, v: u8) {
    out.push(v);
}

pub fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn put_str(out: &mut Vec<u8>, v: &str) {
    put_u32(out, v.len() as u32);
    out.extend_from_slice(v.as_bytes());
}

// ---------------------------------------------------------------------------
// Primitive readers
// ---------------------------------------------------------------------------

pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn u8(&mut self) -> Result<u8> {
        if self.remaining() < 1 {
            return Err(short());
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }

    pub fn u32(&mut self) -> Result<u32> {
        if self.remaining() < 4 {
            return Err(short());
        }
        let mut b = [0u8; 4];
        b.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(u32::from_le_bytes(b))
    }

    pub fn u64(&mut self) -> Result<u64> {
        if self.remaining() < 8 {
            return Err(short());
        }
        let mut b = [0u8; 8];
        b.copy_from_slice(&self.data[self.pos..self.pos + 8]);
        self.pos += 8;
        Ok(u64::from_le_bytes(b))
    }

    pub fn string(&mut self) -> Result<String> {
        let len = self.u32()? as usize;
        if self.remaining() < len {
            return Err(short());
        }
        let s = String::from_utf8_lossy(&self.data[self.pos..self.pos + len]).into_owned();
        self.pos += len;
        Ok(s)
    }
}

fn short() -> metagdb_core::Error {
    metagdb_core::Error::Storage("truncated record".into())
}

// ---------------------------------------------------------------------------
// Property encoding
// ---------------------------------------------------------------------------

pub fn encode_property(out: &mut Vec<u8>, value: &PropertyValue) {
    match value {
        PropertyValue::Null => put_u8(out, 0),
        PropertyValue::Bool(b) => {
            put_u8(out, 1);
            put_u8(out, u8::from(*b));
        }
        PropertyValue::Int(i) => {
            put_u8(out, 2);
            out.extend_from_slice(&i.to_le_bytes());
        }
        PropertyValue::Float(f) => {
            put_u8(out, 3);
            out.extend_from_slice(&f.to_le_bytes());
        }
        PropertyValue::Text(t) => {
            put_u8(out, 4);
            put_str(out, t);
        }
    }
}

pub fn decode_property(reader: &mut Reader<'_>) -> Result<PropertyValue> {
    Ok(match reader.u8()? {
        0 => PropertyValue::Null,
        1 => PropertyValue::Bool(reader.u8()? != 0),
        2 => {
            let mut b = [0u8; 8];
            for slot in &mut b {
                *slot = reader.u8()?;
            }
            PropertyValue::Int(i64::from_le_bytes(b))
        }
        3 => {
            let mut b = [0u8; 8];
            for slot in &mut b {
                *slot = reader.u8()?;
            }
            PropertyValue::Float(f64::from_le_bytes(b))
        }
        4 => PropertyValue::Text(reader.string()?),
        other => {
            return Err(metagdb_core::Error::Storage(format!(
                "unknown property tag {other}"
            )))
        }
    })
}

pub fn encode_properties(out: &mut Vec<u8>, props: &PropertyMap) {
    put_u32(out, props.len() as u32);
    for (key, value) in props {
        put_u32(out, key.0);
        encode_property(out, value);
    }
}

pub fn decode_properties(reader: &mut Reader<'_>) -> Result<PropertyMap> {
    let count = reader.u32()? as usize;
    let mut map = BTreeMap::new();
    for _ in 0..count {
        let key = PropertyKeyId(reader.u32()?);
        let value = decode_property(reader)?;
        map.insert(key, value);
    }
    Ok(map)
}

// ---------------------------------------------------------------------------
// Record encoding
// ---------------------------------------------------------------------------

pub fn encode_vertex(out: &mut Vec<u8>, record: &VertexRecord) {
    put_u64(out, record.id.0);
    put_u32(out, record.labels.len() as u32);
    for label in &record.labels {
        put_u32(out, label.0);
    }
    encode_properties(out, &record.properties);
}

pub fn decode_vertex(reader: &mut Reader<'_>) -> Result<VertexRecord> {
    let id = VertexId(reader.u64()?);
    let label_count = reader.u32()? as usize;
    let mut labels = Vec::with_capacity(label_count);
    for _ in 0..label_count {
        labels.push(LabelId(reader.u32()?));
    }
    let properties = decode_properties(reader)?;
    Ok(VertexRecord {
        id,
        labels,
        properties,
    })
}

pub fn encode_edge(out: &mut Vec<u8>, record: &EdgeRecord) {
    put_u64(out, record.id.0);
    put_u64(out, record.src.0);
    put_u64(out, record.dst.0);
    put_u32(out, record.etype.0);
    encode_properties(out, &record.properties);
}

pub fn decode_edge(reader: &mut Reader<'_>) -> Result<EdgeRecord> {
    let id = EdgeId(reader.u64()?);
    let src = VertexId(reader.u64()?);
    let dst = VertexId(reader.u64()?);
    let etype = EdgeTypeId(reader.u32()?);
    let properties = decode_properties(reader)?;
    Ok(EdgeRecord {
        id,
        src,
        dst,
        etype,
        properties,
    })
}

// ---------------------------------------------------------------------------
// Snapshot container
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct Snapshot {
    pub next_vid: u64,
    pub next_eid: u64,
    pub next_ts: u64,
    pub next_txn: u64,
    pub labels: Vec<(u32, String)>,
    pub edge_types: Vec<(u32, String)>,
    pub property_keys: Vec<(u32, String)>,
    pub vertices: Vec<VertexRecord>,
    pub edges: Vec<EdgeRecord>,
}

pub fn encode_snapshot(snapshot: &Snapshot) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(SNAPSHOT_MAGIC);
    put_u64(&mut out, snapshot.next_vid);
    put_u64(&mut out, snapshot.next_eid);
    put_u64(&mut out, snapshot.next_ts);
    put_u64(&mut out, snapshot.next_txn);
    put_table(&mut out, &snapshot.labels);
    put_table(&mut out, &snapshot.edge_types);
    put_table(&mut out, &snapshot.property_keys);
    put_u64(&mut out, snapshot.vertices.len() as u64);
    for record in &snapshot.vertices {
        encode_vertex(&mut out, record);
    }
    put_u64(&mut out, snapshot.edges.len() as u64);
    for record in &snapshot.edges {
        encode_edge(&mut out, record);
    }
    out
}

fn put_table(out: &mut Vec<u8>, table: &[(u32, String)]) {
    put_u32(out, table.len() as u32);
    for (id, name) in table {
        put_u32(out, *id);
        put_str(out, name);
    }
}

fn read_table(reader: &mut Reader<'_>) -> Result<Vec<(u32, String)>> {
    let count = reader.u32()? as usize;
    let mut table = Vec::with_capacity(count);
    for _ in 0..count {
        let id = reader.u32()?;
        let name = reader.string()?;
        table.push((id, name));
    }
    Ok(table)
}

pub fn decode_snapshot(data: &[u8]) -> Result<Snapshot> {
    if data.len() < 8 || &data[..8] != SNAPSHOT_MAGIC {
        return Err(metagdb_core::Error::Storage("invalid snapshot magic".into()));
    }
    let mut reader = Reader::new(&data[8..]);
    let mut snapshot = Snapshot {
        next_vid: reader.u64()?,
        next_eid: reader.u64()?,
        next_ts: reader.u64()?,
        next_txn: reader.u64()?,
        labels: read_table(&mut reader)?,
        edge_types: read_table(&mut reader)?,
        property_keys: read_table(&mut reader)?,
        vertices: Vec::new(),
        edges: Vec::new(),
    };
    let vertex_count = reader.u64()? as usize;
    for _ in 0..vertex_count {
        snapshot.vertices.push(decode_vertex(&mut reader)?);
    }
    let edge_count = reader.u64()? as usize;
    for _ in 0..edge_count {
        snapshot.edges.push(decode_edge(&mut reader)?);
    }
    Ok(snapshot)
}

// ---------------------------------------------------------------------------
// Write-ahead log
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum WalOp {
    InsertVertex(VertexRecord, u64),
    UpdateVertex(VertexRecord, u64),
    DeleteVertex(VertexId, u64),
    InsertEdge(EdgeRecord, u64),
    UpdateEdge(EdgeRecord, u64),
    DeleteEdge(EdgeId, u64),
}

pub fn encode_wal_op(op: &WalOp) -> Vec<u8> {
    let mut out = Vec::new();
    match op {
        WalOp::InsertVertex(record, ts) => {
            put_u8(&mut out, 1);
            put_u64(&mut out, *ts);
            encode_vertex(&mut out, record);
        }
        WalOp::UpdateVertex(record, ts) => {
            put_u8(&mut out, 2);
            put_u64(&mut out, *ts);
            encode_vertex(&mut out, record);
        }
        WalOp::DeleteVertex(id, ts) => {
            put_u8(&mut out, 3);
            put_u64(&mut out, *ts);
            put_u64(&mut out, id.0);
        }
        WalOp::InsertEdge(record, ts) => {
            put_u8(&mut out, 4);
            put_u64(&mut out, *ts);
            encode_edge(&mut out, record);
        }
        WalOp::UpdateEdge(record, ts) => {
            put_u8(&mut out, 5);
            put_u64(&mut out, *ts);
            encode_edge(&mut out, record);
        }
        WalOp::DeleteEdge(id, ts) => {
            put_u8(&mut out, 6);
            put_u64(&mut out, *ts);
            put_u64(&mut out, id.0);
        }
    }
    out
}

pub fn decode_wal_op(payload: &[u8]) -> Result<WalOp> {
    let mut reader = Reader::new(payload);
    let tag = reader.u8()?;
    let ts = reader.u64()?;
    Ok(match tag {
        1 => WalOp::InsertVertex(decode_vertex(&mut reader)?, ts),
        2 => WalOp::UpdateVertex(decode_vertex(&mut reader)?, ts),
        3 => WalOp::DeleteVertex(VertexId(reader.u64()?), ts),
        4 => WalOp::InsertEdge(decode_edge(&mut reader)?, ts),
        5 => WalOp::UpdateEdge(decode_edge(&mut reader)?, ts),
        6 => WalOp::DeleteEdge(EdgeId(reader.u64()?), ts),
        other => {
            return Err(metagdb_core::Error::Storage(format!(
                "unknown WAL op tag {other}"
            )))
        }
    })
}

pub fn checksum(payload: &[u8]) -> u32 {
    payload.iter().fold(2166136261u32, |hash, byte| {
        (hash ^ u32::from(*byte)).wrapping_mul(16777619)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_vertex() -> VertexRecord {
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(1), PropertyValue::Text("A".into()));
        properties.insert(PropertyKeyId(2), PropertyValue::Int(30));
        VertexRecord {
            id: VertexId(7),
            labels: vec![LabelId(1), LabelId(3)],
            properties,
        }
    }

    #[test]
    fn vertex_round_trip() {
        let record = sample_vertex();
        let mut bytes = Vec::new();
        encode_vertex(&mut bytes, &record);
        let decoded = decode_vertex(&mut Reader::new(&bytes)).unwrap();
        assert_eq!(decoded.id, record.id);
        assert_eq!(decoded.labels, record.labels);
        assert_eq!(decoded.properties, record.properties);
    }

    #[test]
    fn snapshot_round_trip() {
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(1), PropertyValue::Float(0.5));
        let snapshot = Snapshot {
            next_vid: 10,
            next_eid: 4,
            next_ts: 8,
            next_txn: 3,
            labels: vec![(1, "Person".into())],
            edge_types: vec![(1, "KNOWS".into())],
            property_keys: vec![(1, "weight".into())],
            vertices: vec![sample_vertex()],
            edges: vec![EdgeRecord {
                id: EdgeId(1),
                src: VertexId(7),
                dst: VertexId(8),
                etype: EdgeTypeId(1),
                properties,
            }],
        };
        let bytes = encode_snapshot(&snapshot);
        let decoded = decode_snapshot(&bytes).unwrap();
        assert_eq!(decoded.next_vid, 10);
        assert_eq!(decoded.vertices.len(), 1);
        assert_eq!(decoded.edges.len(), 1);
        assert_eq!(decoded.labels, snapshot.labels);
    }

    #[test]
    fn wal_op_round_trip() {
        let op = WalOp::InsertVertex(sample_vertex(), 5);
        let payload = encode_wal_op(&op);
        match decode_wal_op(&payload).unwrap() {
            WalOp::InsertVertex(record, ts) => {
                assert_eq!(ts, 5);
                assert_eq!(record.id, VertexId(7));
            }
            other => panic!("unexpected op {other:?}"),
        }
    }
}
