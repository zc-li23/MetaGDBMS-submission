//! Physical record layout for the append-only record segment.
//!
//! Every committed mutation materialises one immutable physical record.  A
//! logical vertex or edge is represented by a *chain* of physical records: the
//! logical-ID redirection index points at the newest record and each record
//! stores the offset of the previous version.  Readers walk the chain backwards
//! until they find the newest record whose `begin_ts` is visible to their
//! snapshot, which is what makes snapshot isolation work without mutating older
//! records in place.

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};

/// Fixed on-disk header length in bytes.
pub const HEADER_LEN: usize = 48;
const MAGIC: u32 = 0x4D47_4452; // "MGDR"

/// Discriminates the physical record families stored in one segment.
///
/// Deletion tombstones carry the namespace they terminate so recovery can
/// reattach them to the correct redirection index without extra side tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordKind {
    Vertex,
    Edge,
    VertexTombstone,
    EdgeTombstone,
}

impl RecordKind {
    fn tag(self) -> u8 {
        match self {
            RecordKind::Vertex => 1,
            RecordKind::Edge => 2,
            RecordKind::VertexTombstone => 3,
            RecordKind::EdgeTombstone => 4,
        }
    }

    fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            1 => Some(RecordKind::Vertex),
            2 => Some(RecordKind::Edge),
            3 => Some(RecordKind::VertexTombstone),
            4 => Some(RecordKind::EdgeTombstone),
            _ => None,
        }
    }

    /// Logical namespace this record belongs to.
    pub fn namespace(self) -> Namespace {
        match self {
            RecordKind::Vertex | RecordKind::VertexTombstone => Namespace::Vertex,
            RecordKind::Edge | RecordKind::EdgeTombstone => Namespace::Edge,
        }
    }

    /// Whether this record terminates the element's visibility.
    pub fn is_tombstone(self) -> bool {
        matches!(self, RecordKind::VertexTombstone | RecordKind::EdgeTombstone)
    }
}

/// Logical namespace a redirection entry belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    Vertex,
    Edge,
}

/// Payload of one physical record.
#[derive(Debug, Clone)]
pub enum RecordBody {
    Vertex(VertexRecord),
    Edge(EdgeRecord),
}

/// One immutable physical record in the append-only segment.
#[derive(Debug, Clone)]
pub struct PhysicalRecord {
    /// Byte offset of this record inside the segment (1-based, unique).
    pub offset: u64,
    pub kind: RecordKind,
    /// Logical id in the vertex or edge namespace (tombstones keep it too).
    pub logical_id: u64,
    /// Commit timestamp that made this version visible; 0 while uncommitted.
    pub begin_ts: u64,
    /// Offset of the previous version of the same logical element, 0 if none.
    pub prev_offset: u64,
    /// Transaction that produced the record (0 for bulk import).
    pub txn_id: u64,
    pub body: Option<RecordBody>,
}

impl PhysicalRecord {
    pub fn is_tombstone(&self) -> bool {
        self.kind.is_tombstone()
    }

    /// Serialise header plus payload into a fresh buffer.
    pub fn encode(&self) -> Vec<u8> {
        let payload = encode_payload(self);
        let mut out = Vec::with_capacity(HEADER_LEN + payload.len());
        out.extend_from_slice(&MAGIC.to_le_bytes());
        out.push(self.kind.tag());
        out.push(0);
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&self.logical_id.to_le_bytes());
        out.extend_from_slice(&self.begin_ts.to_le_bytes());
        out.extend_from_slice(&self.prev_offset.to_le_bytes());
        out.extend_from_slice(&self.txn_id.to_le_bytes());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&crc32(&payload).to_le_bytes());
        out.extend_from_slice(&payload);
        out
    }

    /// Number of bytes `encode` will produce for this record.
    pub fn encoded_len(&self) -> usize {
        HEADER_LEN + encode_payload(self).len()
    }

    /// Decode one record from `bytes` starting at `offset`.
    ///
    /// Returns `None` when the buffer does not yet contain a complete,
    /// checksum-valid record, which is how a torn trailing write is detected.
    pub fn decode(bytes: &[u8], offset: u64) -> Option<PhysicalRecord> {
        if bytes.len() < HEADER_LEN {
            return None;
        }
        if u32::from_le_bytes(bytes[0..4].try_into().ok()?) != MAGIC {
            return None;
        }
        let kind = RecordKind::from_tag(bytes[4])?;
        let logical_id = read_u64(bytes, 8)?;
        let begin_ts = read_u64(bytes, 16)?;
        let prev_offset = read_u64(bytes, 24)?;
        let txn_id = read_u64(bytes, 32)?;
        let payload_len = read_u32(bytes, 40)? as usize;
        let checksum = read_u32(bytes, 44)?;
        let payload = bytes.get(HEADER_LEN..HEADER_LEN + payload_len)?;
        if crc32(payload) != checksum {
            return None;
        }
        let body = decode_payload(kind, payload, logical_id)?;
        Some(PhysicalRecord {
            offset,
            kind,
            logical_id,
            begin_ts,
            prev_offset,
            txn_id,
            body,
        })
    }
}

fn encode_payload(record: &PhysicalRecord) -> Vec<u8> {
    let mut out = Vec::new();
    match &record.body {
        None => {}
        Some(RecordBody::Vertex(vertex)) => {
            put_u32(&mut out, vertex.labels.len() as u32);
            for label in &vertex.labels {
                put_u32(&mut out, label.0);
            }
            encode_properties(&mut out, &vertex.properties);
        }
        Some(RecordBody::Edge(edge)) => {
            put_u64(&mut out, edge.src.0);
            put_u64(&mut out, edge.dst.0);
            put_u32(&mut out, edge.etype.0);
            encode_properties(&mut out, &edge.properties);
        }
    }
    out
}

fn decode_payload(kind: RecordKind, payload: &[u8], logical_id: u64) -> Option<Option<RecordBody>> {
    match kind {
        RecordKind::VertexTombstone | RecordKind::EdgeTombstone => Some(None),
        RecordKind::Vertex => {
            let mut cursor = Cursor::new(payload);
            let count = cursor.u32()? as usize;
            let mut labels = Vec::with_capacity(count.min(1024));
            for _ in 0..count {
                labels.push(LabelId(cursor.u32()?));
            }
            let properties = cursor.properties()?;
            Some(Some(RecordBody::Vertex(VertexRecord {
                id: VertexId(logical_id),
                labels,
                properties,
            })))
        }
        RecordKind::Edge => {
            let mut cursor = Cursor::new(payload);
            let src = cursor.u64()?;
            let dst = cursor.u64()?;
            let etype = cursor.u32()?;
            let properties = cursor.properties()?;
            Some(Some(RecordBody::Edge(EdgeRecord {
                id: EdgeId(logical_id),
                src: VertexId(src),
                dst: VertexId(dst),
                etype: EdgeTypeId(etype),
                properties,
            })))
        }
    }
}

fn encode_properties(out: &mut Vec<u8>, properties: &PropertyMap) {
    put_u32(out, properties.len() as u32);
    for (key, value) in properties {
        put_u32(out, key.0);
        encode_value(out, value);
    }
}

fn encode_value(out: &mut Vec<u8>, value: &PropertyValue) {
    match value {
        PropertyValue::Null => out.push(0),
        PropertyValue::Bool(v) => {
            out.push(1);
            out.push(u8::from(*v));
        }
        PropertyValue::Int(v) => {
            out.push(2);
            out.extend_from_slice(&v.to_le_bytes());
        }
        PropertyValue::Float(v) => {
            out.push(3);
            out.extend_from_slice(&v.to_le_bytes());
        }
        PropertyValue::Text(v) => {
            out.push(4);
            put_u32(out, v.len() as u32);
            out.extend_from_slice(v.as_bytes());
        }
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        let slice = self.bytes.get(self.position..self.position + len)?;
        self.position += len;
        Some(slice)
    }

    fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|v| v[0])
    }

    fn u32(&mut self) -> Option<u32> {
        self.take(4).map(|v| u32::from_le_bytes(v.try_into().unwrap()))
    }

    fn u64(&mut self) -> Option<u64> {
        self.take(8).map(|v| u64::from_le_bytes(v.try_into().unwrap()))
    }

    fn value(&mut self) -> Option<PropertyValue> {
        match self.u8()? {
            0 => Some(PropertyValue::Null),
            1 => Some(PropertyValue::Bool(self.u8()? != 0)),
            2 => Some(PropertyValue::Int(self.u64()? as i64)),
            3 => Some(PropertyValue::Float(f64::from_bits(self.u64()?))),
            4 => {
                let len = self.u32()? as usize;
                let raw = self.take(len)?;
                Some(PropertyValue::Text(String::from_utf8_lossy(raw).into_owned()))
            }
            _ => None,
        }
    }

    fn properties(&mut self) -> Option<PropertyMap> {
        let count = self.u32()? as usize;
        let mut map = PropertyMap::new();
        for _ in 0..count {
            let key = PropertyKeyId(self.u32()?);
            let value = self.value()?;
            map.insert(key, value);
        }
        Some(map)
    }
}

fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn read_u32(bytes: &[u8], at: usize) -> Option<u32> {
    bytes.get(at..at + 4).map(|v| u32::from_le_bytes(v.try_into().unwrap()))
}

fn read_u64(bytes: &[u8], at: usize) -> Option<u64> {
    bytes.get(at..at + 8).map(|v| u64::from_le_bytes(v.try_into().unwrap()))
}

/// CRC-32 (IEEE 802.3), used to reject torn or corrupt records.
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for byte in bytes {
        crc ^= *byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_vertex() -> PhysicalRecord {
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(1), PropertyValue::Text("A".into()));
        properties.insert(PropertyKeyId(2), PropertyValue::Int(30));
        PhysicalRecord {
            offset: 7,
            kind: RecordKind::Vertex,
            logical_id: 3,
            begin_ts: 12,
            prev_offset: 4,
            txn_id: 9,
            body: Some(RecordBody::Vertex(VertexRecord {
                id: VertexId(3),
                labels: vec![LabelId(1)],
                properties,
            })),
        }
    }

    #[test]
    fn vertex_record_round_trips() {
        let record = sample_vertex();
        let encoded = record.encode();
        assert_eq!(encoded.len(), record.encoded_len());
        let decoded = PhysicalRecord::decode(&encoded, 7).expect("decodes");
        assert_eq!(decoded.kind, RecordKind::Vertex);
        assert_eq!(decoded.logical_id, 3);
        assert_eq!(decoded.begin_ts, 12);
        assert_eq!(decoded.prev_offset, 4);
        match decoded.body {
            Some(RecordBody::Vertex(vertex)) => {
                assert_eq!(vertex.labels, vec![LabelId(1)]);
                assert_eq!(
                    vertex.properties.get(&PropertyKeyId(2)),
                    Some(&PropertyValue::Int(30))
                );
            }
            _ => panic!("expected vertex body"),
        }
    }

    #[test]
    fn edge_record_round_trips() {
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(7), PropertyValue::Float(0.5));
        let record = PhysicalRecord {
            offset: 1,
            kind: RecordKind::Edge,
            logical_id: 2,
            begin_ts: 1,
            prev_offset: 0,
            txn_id: 0,
            body: Some(RecordBody::Edge(EdgeRecord {
                id: EdgeId(2),
                src: VertexId(1),
                dst: VertexId(4),
                etype: EdgeTypeId(2),
                properties,
            })),
        };
        let decoded = PhysicalRecord::decode(&record.encode(), 1).expect("decodes");
        match decoded.body {
            Some(RecordBody::Edge(edge)) => {
                assert_eq!(edge.src, VertexId(1));
                assert_eq!(edge.dst, VertexId(4));
                assert_eq!(edge.etype, EdgeTypeId(2));
                assert_eq!(
                    edge.properties.get(&PropertyKeyId(7)),
                    Some(&PropertyValue::Float(0.5))
                );
            }
            _ => panic!("expected edge body"),
        }
    }

    #[test]
    fn tombstone_and_torn_records_are_handled() {
        let tombstone = PhysicalRecord {
            offset: 9,
            kind: RecordKind::EdgeTombstone,
            logical_id: 5,
            begin_ts: 4,
            prev_offset: 2,
            txn_id: 3,
            body: None,
        };
        let decoded = PhysicalRecord::decode(&tombstone.encode(), 9).expect("decodes");
        assert!(decoded.is_tombstone());

        let mut encoded = sample_vertex().encode();
        encoded.truncate(encoded.len() - 1);
        assert!(PhysicalRecord::decode(&encoded, 7).is_none());

        let mut corrupt = sample_vertex().encode();
        let last = corrupt.len() - 1;
        corrupt[last] ^= 0xFF;
        assert!(PhysicalRecord::decode(&corrupt, 7).is_none());
    }
}
