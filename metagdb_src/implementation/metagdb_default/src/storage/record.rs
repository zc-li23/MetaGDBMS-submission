/**
 * @brief: Record encoding/decoding, responsible for bidirectional conversion between VertexRecord / EdgeRecord and byte sequences.
 *
 * Each record contains a fixed-length record header (36 bytes) before the user data:
 *   created_ts:    u64  (8B)  Creation commit timestamp
 *   deleted_ts:    u64  (8B)  Deletion timestamp (0 = not deleted)
 *   creator_txn:   u64  (8B)  Creating transaction ID
 *   flags:         u8   (1B)  Flag bits (bit 0 = committed)
 *   prev_location: 11B        Previous version disk location (file_tag=0xFF means none)
 */

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};

use super::index::RecordLocation;

/// PropertyValue type tags.
const PV_NULL: u8 = 0;
const PV_BOOL: u8 = 1;
const PV_INT: u8 = 2;
const PV_FLOAT: u8 = 3;
const PV_TEXT: u8 = 4;

/// MVCC version header fixed size (bytes).
pub const MVCC_HEADER_SIZE: usize = 25;

/// prev_location field size (bytes): consistent with RecordLocation encoding.
pub const PREV_LOCATION_SIZE: usize = 11;

/// Total record header size: MVCC header + prev_location.
pub const RECORD_HEADER_SIZE: usize = MVCC_HEADER_SIZE + PREV_LOCATION_SIZE; // 36

/// MVCC flag bit: committed.
pub const MVCC_FLAG_COMMITTED: u8 = 0x01;

/// MVCC header field offsets (relative to start of record data).
pub const MVCC_OFFSET_CREATED_TS: usize = 0;
pub const MVCC_OFFSET_DELETED_TS: usize = 8;
pub const MVCC_OFFSET_CREATOR_TXN: usize = 16;
pub const MVCC_OFFSET_FLAGS: usize = 24;
/// Sentinel value for prev_location null (file_tag = 0xFF), indicating no previous version.
const PREV_NULL_SENTINEL: u8 = 0xFF;

/**
 * @brief: MVCC version header, embedded in the first 25 bytes of each vertex/edge record.
 */
#[derive(Debug, Clone, Copy)]
pub struct MvccHeader {
    pub created_ts: u64,
    pub deleted_ts: u64,
    pub creator_txn: u64,
    pub flags: u8,
}

impl MvccHeader {
    /// Construct a new MVCC header.
    pub fn new(created_ts: u64, deleted_ts: u64, creator_txn: u64, flags: u8) -> Self {
        Self {
            created_ts,
            deleted_ts,
            creator_txn,
            flags,
        }
    }

    /// Whether the record is committed.
    pub fn is_committed(&self) -> bool {
        self.flags & MVCC_FLAG_COMMITTED != 0
    }
}

impl Default for MvccHeader {
    fn default() -> Self {
        Self {
            created_ts: 0,
            deleted_ts: 0,
            creator_txn: 0,
            flags: 0,
        }
    }
}

/**
 * @brief: Encode MVCC header to a fixed-length byte array.
 */
pub fn encode_mvcc_header(header: &MvccHeader) -> [u8; MVCC_HEADER_SIZE] {
    let mut buf = [0u8; MVCC_HEADER_SIZE];
    buf[0..8].copy_from_slice(&header.created_ts.to_le_bytes());
    buf[8..16].copy_from_slice(&header.deleted_ts.to_le_bytes());
    buf[16..24].copy_from_slice(&header.creator_txn.to_le_bytes());
    buf[24] = header.flags;
    buf
}

/**
 * @brief: Decode MVCC header from a byte slice.
 */
pub fn decode_mvcc_header(data: &[u8]) -> Option<MvccHeader> {
    if data.len() < MVCC_HEADER_SIZE {
        return None;
    }
    let created_ts = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);
    let deleted_ts = u64::from_le_bytes([
        data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
    ]);
    let creator_txn = u64::from_le_bytes([
        data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23],
    ]);
    let flags = data[24];
    Some(MvccHeader {
        created_ts,
        deleted_ts,
        creator_txn,
        flags,
    })
}

/**
 * @brief: Encode prev_location to 11 bytes. None is represented by file_tag=0xFF sentinel.
 */
pub fn encode_prev_location(prev: Option<RecordLocation>) -> [u8; PREV_LOCATION_SIZE] {
    match prev {
        Some(loc) => loc.to_bytes(),
        None => {
            let mut buf = [0u8; PREV_LOCATION_SIZE];
            buf[0] = PREV_NULL_SENTINEL;
            buf
        }
    }
}

/**
 * @brief: Decode prev_location. file_tag=0xFF means no previous version.
 * @return: Some(loc) for the previous version location, None means no previous version.
 */
pub fn decode_prev_location(data: &[u8]) -> Option<RecordLocation> {
    if data.len() < PREV_LOCATION_SIZE {
        return None;
    }
    if data[0] == PREV_NULL_SENTINEL {
        None
    } else {
        RecordLocation::from_bytes(data)
    }
}

/**
 * @brief: Append encoded PropertyValue to a byte buffer.
 */
fn encode_property_value(value: &PropertyValue, buf: &mut Vec<u8>) {
    match value {
        PropertyValue::Null => {
            buf.push(PV_NULL);
        }
        PropertyValue::Bool(v) => {
            buf.push(PV_BOOL);
            buf.push(*v as u8);
        }
        PropertyValue::Int(v) => {
            buf.push(PV_INT);
            buf.extend_from_slice(&v.to_le_bytes());
        }
        PropertyValue::Float(v) => {
            buf.push(PV_FLOAT);
            buf.extend_from_slice(&v.to_le_bytes());
        }
        PropertyValue::Text(s) => {
            buf.push(PV_TEXT);
            let len = s.len() as u32;
            buf.extend_from_slice(&len.to_le_bytes());
            buf.extend_from_slice(s.as_bytes());
        }
    }
}

/**
 * @brief: Decode PropertyValue from a byte slice, returning (value, bytes_consumed).
 */
fn decode_property_value(data: &[u8]) -> Option<(PropertyValue, usize)> {
    if data.is_empty() {
        return None;
    }
    match data[0] {
        PV_NULL => Some((PropertyValue::Null, 1)),
        PV_BOOL => {
            if data.len() < 2 {
                return None;
            }
            Some((PropertyValue::Bool(data[1] != 0), 2))
        }
        PV_INT => {
            if data.len() < 9 {
                return None;
            }
            let v = i64::from_le_bytes([
                data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
            ]);
            Some((PropertyValue::Int(v), 9))
        }
        PV_FLOAT => {
            if data.len() < 9 {
                return None;
            }
            let v = f64::from_le_bytes([
                data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
            ]);
            Some((PropertyValue::Float(v), 9))
        }
        PV_TEXT => {
            if data.len() < 5 {
                return None;
            }
            let len = u32::from_le_bytes([data[1], data[2], data[3], data[4]]) as usize;
            if data.len() < 5 + len {
                return None;
            }
            let s = String::from_utf8(data[5..5 + len].to_vec()).ok()?;
            Some((PropertyValue::Text(s), 5 + len))
        }
        _ => None,
    }
}

/**
 * @brief: Encode PropertyMap to a byte buffer.
 */
fn encode_property_map(props: &PropertyMap, buf: &mut Vec<u8>) {
    let count = props.len() as u16;
    buf.extend_from_slice(&count.to_le_bytes());
    for (key, value) in props.iter() {
        buf.extend_from_slice(&key.0.to_le_bytes());
        encode_property_value(value, buf);
    }
}

/**
 * @brief: Decode PropertyMap from a byte slice, returning (map, bytes_consumed).
 */
fn decode_property_map(data: &[u8]) -> Option<(PropertyMap, usize)> {
    if data.len() < 2 {
        return None;
    }
    let count = u16::from_le_bytes([data[0], data[1]]) as usize;
    let mut offset = 2;
    let mut map = PropertyMap::new();
    for _ in 0..count {
        if data.len() < offset + 4 {
            return None;
        }
        let key = PropertyKeyId(u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]));
        offset += 4;
        let (value, consumed) = decode_property_value(&data[offset..])?;
        offset += consumed;
        map.insert(key, value);
    }
    Some((map, offset))
}

/**
 * @brief: Encode VertexRecord together with MVCC header and prev_location into a byte sequence.
 * @param record: Vertex record to encode.
 * @param mvcc: MVCC version header.
 * @param prev_location: Previous version disk location, None means this is the first version.
 * @return: Encoded byte vector.
 */
pub fn encode_vertex(
    record: &VertexRecord,
    mvcc: &MvccHeader,
    prev_location: Option<RecordLocation>,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&encode_mvcc_header(mvcc));
    buf.extend_from_slice(&encode_prev_location(prev_location));
    buf.extend_from_slice(&record.id.0.to_le_bytes());
    let label_count = record.labels.len() as u16;
    buf.extend_from_slice(&label_count.to_le_bytes());
    for label in &record.labels {
        buf.extend_from_slice(&label.0.to_le_bytes());
    }
    encode_property_map(&record.properties, &mut buf);
    buf
}

/**
 * @brief: Decode a vertex record from a byte slice, also returning MVCC header and prev_location.
 * @param data: Encoded byte slice (starting from the record header).
 * @return: On success returns (MvccHeader, prev_location, VertexRecord).
 */
pub fn decode_vertex(data: &[u8]) -> Option<(MvccHeader, Option<RecordLocation>, VertexRecord)> {
    if data.len() < RECORD_HEADER_SIZE + 10 {
        return None;
    }
    let mvcc = decode_mvcc_header(data)?;
    let prev_location = decode_prev_location(&data[MVCC_HEADER_SIZE..]);
    let payload = &data[RECORD_HEADER_SIZE..];
    let id = VertexId(u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3],
        payload[4], payload[5], payload[6], payload[7],
    ]));
    let label_count = u16::from_le_bytes([payload[8], payload[9]]) as usize;
    let mut offset = 10;
    let mut labels = Vec::with_capacity(label_count);
    for _ in 0..label_count {
        if payload.len() < offset + 4 {
            return None;
        }
        labels.push(LabelId(u32::from_le_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
        ])));
        offset += 4;
    }
    let (properties, _) = decode_property_map(&payload[offset..])?;
    Some((
        mvcc,
        prev_location,
        VertexRecord {
            id,
            labels,
            properties,
        },
    ))
}

/**
 * @brief: Encode EdgeRecord together with MVCC header and prev_location into a byte sequence.
 * @param record: Edge record to encode.
 * @param mvcc: MVCC version header.
 * @param prev_location: Previous version disk location, None means this is the first version.
 * @return: Encoded byte vector.
 */
pub fn encode_edge(
    record: &EdgeRecord,
    mvcc: &MvccHeader,
    prev_location: Option<RecordLocation>,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&encode_mvcc_header(mvcc));
    buf.extend_from_slice(&encode_prev_location(prev_location));
    buf.extend_from_slice(&record.id.0.to_le_bytes());
    buf.extend_from_slice(&record.src.0.to_le_bytes());
    buf.extend_from_slice(&record.dst.0.to_le_bytes());
    buf.extend_from_slice(&record.etype.0.to_le_bytes());
    encode_property_map(&record.properties, &mut buf);
    buf
}

/**
 * @brief: Decode an edge record from a byte slice, also returning MVCC header and prev_location.
 * @param data: Encoded byte slice (starting from the record header).
 * @return: On success returns (MvccHeader, prev_location, EdgeRecord).
 */
pub fn decode_edge(data: &[u8]) -> Option<(MvccHeader, Option<RecordLocation>, EdgeRecord)> {
    if data.len() < RECORD_HEADER_SIZE + 28 {
        return None;
    }
    let mvcc = decode_mvcc_header(data)?;
    let prev_location = decode_prev_location(&data[MVCC_HEADER_SIZE..]);
    let payload = &data[RECORD_HEADER_SIZE..];
    let id = EdgeId(u64::from_le_bytes([
        payload[0], payload[1], payload[2], payload[3],
        payload[4], payload[5], payload[6], payload[7],
    ]));
    let src = VertexId(u64::from_le_bytes([
        payload[8], payload[9], payload[10], payload[11],
        payload[12], payload[13], payload[14], payload[15],
    ]));
    let dst = VertexId(u64::from_le_bytes([
        payload[16], payload[17], payload[18], payload[19],
        payload[20], payload[21], payload[22], payload[23],
    ]));
    let etype = EdgeTypeId(u32::from_le_bytes([
        payload[24], payload[25], payload[26], payload[27],
    ]));
    let (properties, _) = decode_property_map(&payload[28..])?;
    Some((
        mvcc,
        prev_location,
        EdgeRecord {
            id,
            src,
            dst,
            etype,
            properties,
        },
    ))
}

/**
 * @brief: Encode adjacency list to a byte sequence.
 * @param out_edges: List of outgoing edge IDs.
 * @param in_edges: List of incoming edge IDs.
 * @return: Encoded byte vector.
 */
pub fn encode_adjacency(out_edges: &[EdgeId], in_edges: &[EdgeId]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&(out_edges.len() as u16).to_le_bytes());
    for eid in out_edges {
        buf.extend_from_slice(&eid.0.to_le_bytes());
    }
    buf.extend_from_slice(&(in_edges.len() as u16).to_le_bytes());
    for eid in in_edges {
        buf.extend_from_slice(&eid.0.to_le_bytes());
    }
    buf
}

/**
 * @brief: Decode adjacency list from a byte slice.
 * @param data: Encoded byte slice.
 * @return: On success returns (out_edges, in_edges) pair.
 */
pub fn decode_adjacency(data: &[u8]) -> Option<(Vec<EdgeId>, Vec<EdgeId>)> {
    if data.len() < 2 {
        return None;
    }
    let out_count = u16::from_le_bytes([data[0], data[1]]) as usize;
    let mut offset = 2;
    let mut out_edges = Vec::with_capacity(out_count);
    for _ in 0..out_count {
        if data.len() < offset + 8 {
            return None;
        }
        out_edges.push(EdgeId(u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ])));
        offset += 8;
    }
    if data.len() < offset + 2 {
        return None;
    }
    let in_count = u16::from_le_bytes([data[offset], data[offset + 1]]) as usize;
    offset += 2;
    let mut in_edges = Vec::with_capacity(in_count);
    for _ in 0..in_count {
        if data.len() < offset + 8 {
            return None;
        }
        in_edges.push(EdgeId(u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ])));
        offset += 8;
    }
    Some((out_edges, in_edges))
}
