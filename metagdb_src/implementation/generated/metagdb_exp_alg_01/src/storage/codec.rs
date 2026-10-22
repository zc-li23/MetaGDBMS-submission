//! Minimal binary codec for the write-ahead log and the compaction snapshot.
//!
//! The design vector asks for a local-disk write-ahead log whose replay
//! restores committed transactions and discards incomplete ones.  Records are
//! therefore self-describing and length-prefixed so a torn tail record can be
//! detected and dropped: every record is `u32 payload_len | u32 crc32 | payload`
//! and replay stops at the first record that is short or fails its checksum.
//!
//! Encoding is written against a [`Sink`] rather than a `Vec` so a large
//! transaction (a million-vertex graph import, for example) streams to the log
//! while its running checksum is computed, instead of materializing the whole
//! record in memory first.

use std::fs::File;
use std::io::Write;

use metagdb_core::common::ids::{EdgeId, EdgeTypeId, LabelId, PropertyKeyId, VertexId};
use metagdb_core::common::property::{PropertyMap, PropertyValue};
use metagdb_core::l2_storage::{EdgeRecord, VertexRecord};
use metagdb_core::{Error, Result};

/// Destination for encoded bytes.
pub trait Sink {
    fn write(&mut self, bytes: &[u8]) -> Result<()>;
}

/// In-memory sink used by the snapshot writer and by tests.
#[derive(Default)]
pub struct VecSink {
    buffer: Vec<u8>,
}

impl VecSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl Sink for VecSink {
    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }
}

/// Buffered file sink that maintains the running CRC and length of what it wrote.
pub struct FileSink<'a> {
    file: &'a mut File,
    buffer: Vec<u8>,
    crc: u32,
    length: u64,
}

impl<'a> FileSink<'a> {
    const FLUSH_THRESHOLD: usize = 64 * 1024;

    pub fn new(file: &'a mut File) -> Self {
        Self {
            file,
            buffer: Vec::with_capacity(Self::FLUSH_THRESHOLD * 2),
            crc: crc32_state_init(),
            length: 0,
        }
    }

    /// Flush buffered bytes and report `(crc32, payload length)`.
    pub fn finish(mut self) -> Result<(u32, u64)> {
        self.file.write_all(&self.buffer).map_err(io_error)?;
        self.buffer.clear();
        Ok((crc32_state_finish(self.crc), self.length))
    }
}

impl Sink for FileSink<'_> {
    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.crc = crc32_state_update(self.crc, bytes);
        self.length += bytes.len() as u64;
        self.buffer.extend_from_slice(bytes);
        if self.buffer.len() >= Self::FLUSH_THRESHOLD {
            self.file.write_all(&self.buffer).map_err(io_error)?;
            self.buffer.clear();
        }
        Ok(())
    }
}

fn io_error(error: std::io::Error) -> Error {
    Error::Storage(error.to_string())
}

/// Typed encoder over any sink.
pub struct Writer<S: Sink> {
    sink: S,
}

impl<S: Sink> Writer<S> {
    pub fn new(sink: S) -> Self {
        Self { sink }
    }

    pub fn u8(&mut self, value: u8) -> Result<&mut Self> {
        self.sink.write(&[value])?;
        Ok(self)
    }

    pub fn u32(&mut self, value: u32) -> Result<&mut Self> {
        self.sink.write(&value.to_le_bytes())?;
        Ok(self)
    }

    pub fn u64(&mut self, value: u64) -> Result<&mut Self> {
        self.sink.write(&value.to_le_bytes())?;
        Ok(self)
    }

    pub fn i64(&mut self, value: i64) -> Result<&mut Self> {
        self.sink.write(&value.to_le_bytes())?;
        Ok(self)
    }

    pub fn f64(&mut self, value: f64) -> Result<&mut Self> {
        self.sink.write(&value.to_le_bytes())?;
        Ok(self)
    }

    pub fn bytes(&mut self, value: &[u8]) -> Result<&mut Self> {
        self.sink.write(&(value.len() as u32).to_le_bytes())?;
        self.sink.write(value)?;
        Ok(self)
    }

    /// Write bytes verbatim, without the length prefix `bytes` adds.
    pub fn raw(&mut self, value: &[u8]) -> Result<&mut Self> {
        self.sink.write(value)?;
        Ok(self)
    }

    pub fn text(&mut self, value: &str) -> Result<&mut Self> {
        self.bytes(value.as_bytes())
    }

    pub fn value(&mut self, value: &PropertyValue) -> Result<&mut Self> {
        match value {
            PropertyValue::Null => {
                self.u8(0)?;
            }
            PropertyValue::Bool(v) => {
                self.u8(1)?.u8(u8::from(*v))?;
            }
            PropertyValue::Int(v) => {
                self.u8(2)?.i64(*v)?;
            }
            PropertyValue::Float(v) => {
                self.u8(3)?.f64(*v)?;
            }
            PropertyValue::Text(v) => {
                self.u8(4)?.text(v)?;
            }
        }
        Ok(self)
    }

    pub fn properties(&mut self, value: &PropertyMap) -> Result<&mut Self> {
        self.u32(value.len() as u32)?;
        for (key, item) in value {
            self.u32(key.0)?.value(item)?;
        }
        Ok(self)
    }

    pub fn labels(&mut self, value: &[LabelId]) -> Result<&mut Self> {
        self.u32(value.len() as u32)?;
        for label in value {
            self.u32(label.0)?;
        }
        Ok(self)
    }

    pub fn vertex(&mut self, value: &VertexRecord) -> Result<&mut Self> {
        self.u64(value.id.0)?;
        self.labels(&value.labels)?;
        self.properties(&value.properties)?;
        Ok(self)
    }

    pub fn edge(&mut self, value: &EdgeRecord) -> Result<&mut Self> {
        self.u64(value.id.0)?;
        self.u64(value.src.0)?;
        self.u64(value.dst.0)?;
        self.u32(value.etype.0)?;
        self.properties(&value.properties)?;
        Ok(self)
    }

    pub fn into_sink(self) -> S {
        self.sink
    }
}

impl Writer<VecSink> {
    pub fn into_bytes(self) -> Vec<u8> {
        self.sink.into_bytes()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.sink.as_bytes()
    }
}

/// Typed decoder over a byte slice.
pub struct Reader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.position)
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn done(&self) -> bool {
        self.remaining() == 0
    }

    pub fn seek(&mut self, position: usize) {
        self.position = position.min(self.data.len());
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8]> {
        if self.remaining() < count {
            return Err(Error::Storage("truncated record".into()));
        }
        let slice = &self.data[self.position..self.position + count];
        self.position += count;
        Ok(slice)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    pub fn u32(&mut self) -> Result<u32> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn u64(&mut self) -> Result<u64> {
        let bytes = self.take(8)?;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(u64::from_le_bytes(array))
    }

    pub fn i64(&mut self) -> Result<i64> {
        Ok(self.u64()? as i64)
    }

    pub fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.u64()?))
    }

    pub fn bytes(&mut self) -> Result<&'a [u8]> {
        let length = self.u32()? as usize;
        self.take(length)
    }

    pub fn text(&mut self) -> Result<String> {
        let raw = self.bytes()?;
        String::from_utf8(raw.to_vec()).map_err(|_| Error::Storage("invalid utf-8".into()))
    }

    pub fn value(&mut self) -> Result<PropertyValue> {
        Ok(match self.u8()? {
            0 => PropertyValue::Null,
            1 => PropertyValue::Bool(self.u8()? != 0),
            2 => PropertyValue::Int(self.i64()?),
            3 => PropertyValue::Float(self.f64()?),
            4 => PropertyValue::Text(self.text()?),
            other => {
                return Err(Error::Storage(format!(
                    "unknown property value tag {other}"
                )))
            }
        })
    }

    pub fn properties(&mut self) -> Result<PropertyMap> {
        let count = self.u32()? as usize;
        let mut map = PropertyMap::new();
        for _ in 0..count {
            let key = PropertyKeyId(self.u32()?);
            let value = self.value()?;
            map.insert(key, value);
        }
        Ok(map)
    }

    pub fn labels(&mut self) -> Result<Vec<LabelId>> {
        let count = self.u32()? as usize;
        let mut labels = Vec::with_capacity(count);
        for _ in 0..count {
            labels.push(LabelId(self.u32()?));
        }
        Ok(labels)
    }

    pub fn vertex(&mut self) -> Result<VertexRecord> {
        Ok(VertexRecord {
            id: VertexId(self.u64()?),
            labels: self.labels()?,
            properties: self.properties()?,
        })
    }

    pub fn edge(&mut self) -> Result<EdgeRecord> {
        Ok(EdgeRecord {
            id: EdgeId(self.u64()?),
            src: VertexId(self.u64()?),
            dst: VertexId(self.u64()?),
            etype: EdgeTypeId(self.u32()?),
            properties: self.properties()?,
        })
    }
}

// ---------------------------------------------------------------------------
// CRC-32 (IEEE 802.3), incremental so the log can be checksummed while it is
// written rather than after it is fully buffered.
// ---------------------------------------------------------------------------

pub fn crc32_state_init() -> u32 {
    0xFFFF_FFFF
}

pub fn crc32_state_update(mut crc: u32, data: &[u8]) -> u32 {
    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    crc
}

pub fn crc32_state_finish(state: u32) -> u32 {
    !state
}

pub fn crc32(data: &[u8]) -> u32 {
    crc32_state_finish(crc32_state_update(crc32_state_init(), data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_round_trip() {
        let mut writer = Writer::new(VecSink::new());
        let mut properties = PropertyMap::new();
        properties.insert(PropertyKeyId(1), PropertyValue::Int(-7));
        properties.insert(PropertyKeyId(2), PropertyValue::Text("hé".into()));
        properties.insert(PropertyKeyId(3), PropertyValue::Float(1.5));
        properties.insert(PropertyKeyId(4), PropertyValue::Bool(true));
        properties.insert(PropertyKeyId(5), PropertyValue::Null);
        let vertex = VertexRecord {
            id: VertexId(9),
            labels: vec![LabelId(1), LabelId(2)],
            properties: properties.clone(),
        };
        let edge = EdgeRecord {
            id: EdgeId(4),
            src: VertexId(1),
            dst: VertexId(2),
            etype: EdgeTypeId(3),
            properties,
        };
        writer.vertex(&vertex).unwrap();
        writer.edge(&edge).unwrap();
        let bytes = writer.into_bytes();

        let mut reader = Reader::new(&bytes);
        assert_eq!(reader.vertex().unwrap().id, VertexId(9));
        assert_eq!(reader.edge().unwrap().dst, VertexId(2));
        assert!(reader.done());
    }

    #[test]
    fn truncated_records_are_reported() {
        let mut writer = Writer::new(VecSink::new());
        writer.u64(7).unwrap();
        let bytes = writer.into_bytes();
        let mut reader = Reader::new(&bytes[..3]);
        assert!(reader.u64().is_err());
    }

    #[test]
    fn checksum_matches_the_reference_check_value() {
        // The standard CRC-32 (IEEE 802.3) check value for "123456789".
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_ne!(crc32(b"committed"), crc32(b"committeD"));
    }

    #[test]
    fn incremental_checksum_equals_the_one_shot_value() {
        let data: Vec<u8> = (0..=255u8).collect();
        let mut state = crc32_state_init();
        for chunk in data.chunks(7) {
            state = crc32_state_update(state, chunk);
        }
        assert_eq!(crc32_state_finish(state), crc32(&data));
    }
}
