/**
 * @brief: Persistent index, maintaining a mapping from vertex/edge IDs to disk locations.
 *
 * The index is maintained in memory as a HashMap, with persistence achieved via the index.dat file.
 * On each sync, the index is serialized to disk; on startup, it is recovered by loading from disk.
 */

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::FileExt;
use std::path::Path;

use metagdb_core::common::ids::{EdgeId, VertexId};

use crate::storage::layout;

/// index.dat file magic: "IDNX"
const INDEX_MAGIC: u32 = 0x4944_4E58;
const INDEX_VERSION: u32 = 1;

/**
 * @brief: Record disk location information.
 */
#[derive(Debug, Clone, Copy)]
pub struct RecordLocation {
    pub file_tag: u8,
    pub page_num: u64,
    pub slot: u16,
}

impl RecordLocation {
    pub fn new(file_tag: u8, page_num: u64, slot: u16) -> Self {
        Self {
            file_tag,
            page_num,
            slot,
        }
    }

    /// Encode to byte representation: [file_tag: u8][page_num: u64][slot: u16] = 11 bytes
    pub fn to_bytes(&self) -> [u8; 11] {
        let mut buf = [0u8; 11];
        buf[0] = self.file_tag;
        buf[1..9].copy_from_slice(&self.page_num.to_le_bytes());
        buf[9..11].copy_from_slice(&self.slot.to_le_bytes());
        buf
    }

    /// Decode from bytes.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 11 {
            return None;
        }
        let file_tag = data[0];
        let page_num = u64::from_le_bytes([
            data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
        ]);
        let slot = u16::from_le_bytes([data[9], data[10]]);
        Some(Self {
            file_tag,
            page_num,
            slot,
        })
    }
}

/**
 * @brief: In-memory index, maintaining fast ID-to-disk-location mapping.
 */
pub struct Index {
    /// Vertex index: vid -> RecordLocation.
    pub vertex_index: HashMap<VertexId, RecordLocation>,
    /// Edge index: eid -> RecordLocation.
    pub edge_index: HashMap<EdgeId, RecordLocation>,
}

impl Index {
    /**
     * @brief: Create an empty index.
     */
    pub fn new() -> Self {
        Self {
            vertex_index: HashMap::new(),
            edge_index: HashMap::new(),
        }
    }

    /**
     * @brief: Register a vertex location.
     */
    pub fn insert_vertex(&mut self, vid: VertexId, loc: RecordLocation) {
        self.vertex_index.insert(vid, loc);
    }

    /**
     * @brief: Look up a vertex location.
     */
    pub fn get_vertex(&self, vid: VertexId) -> Option<RecordLocation> {
        self.vertex_index.get(&vid).copied()
    }

    /**
     * @brief: Register an edge location.
     */
    pub fn insert_edge(&mut self, eid: EdgeId, loc: RecordLocation) {
        self.edge_index.insert(eid, loc);
    }

    /**
     * @brief: Look up an edge location.
     */
    pub fn get_edge(&self, eid: EdgeId) -> Option<RecordLocation> {
        self.edge_index.get(&eid).copied()
    }

    /**
     * @brief: Remove a vertex index entry, returning the removed location.
     */
    pub fn remove_vertex(&mut self, vid: VertexId) -> Option<RecordLocation> {
        self.vertex_index.remove(&vid)
    }

    /**
     * @brief: Remove an edge index entry, returning the removed location.
     */
    pub fn remove_edge(&mut self, eid: EdgeId) -> Option<RecordLocation> {
        self.edge_index.remove(&eid)
    }

    /**
     * @brief: Iterate over all vertex IDs.
     */
    pub fn vertex_ids(&self) -> impl Iterator<Item = VertexId> + '_ {
        self.vertex_index.keys().copied()
    }

    /**
     * @brief: Iterate over all edge IDs.
     */
    #[allow(dead_code)]
    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.edge_index.keys().copied()
    }

    /**
     * @brief: Check whether the vertex index is empty.
     */
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.vertex_index.is_empty()
    }

    /**
     * @brief: Export the index as a byte sequence for persistence.
     */
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&INDEX_MAGIC.to_le_bytes());
        buf.extend_from_slice(&INDEX_VERSION.to_le_bytes());
        buf.extend_from_slice(&(self.vertex_index.len() as u32).to_le_bytes());
        for (vid, loc) in &self.vertex_index {
            buf.extend_from_slice(&vid.0.to_le_bytes());
            buf.extend_from_slice(&loc.to_bytes());
        }
        buf.extend_from_slice(&(self.edge_index.len() as u32).to_le_bytes());
        for (eid, loc) in &self.edge_index {
            buf.extend_from_slice(&eid.0.to_le_bytes());
            buf.extend_from_slice(&loc.to_bytes());
        }
        buf
    }

    /**
     * @brief: Restore index from a byte sequence.
     */
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 16 {
            return None;
        }
        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != INDEX_MAGIC {
            return None;
        }
        let _version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let vcount = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
        let mut offset = 12;
        let mut vertex_index = HashMap::new();
        for _ in 0..vcount {
            if data.len() < offset + 8 + 11 {
                return None;
            }
            let vid = VertexId(u64::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]));
            offset += 8;
            let loc = RecordLocation::from_bytes(&data[offset..offset + 11])?;
            offset += 11;
            vertex_index.insert(vid, loc);
        }
        if data.len() < offset + 4 {
            return None;
        }
        let ecount = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;
        let mut edge_index = HashMap::new();
        for _ in 0..ecount {
            if data.len() < offset + 8 + 11 {
                return None;
            }
            let eid = EdgeId(u64::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]));
            offset += 8;
            let loc = RecordLocation::from_bytes(&data[offset..offset + 11])?;
            offset += 11;
            edge_index.insert(eid, loc);
        }
        Some(Index {
            vertex_index,
            edge_index,
        })
    }

    /**
     * @brief: Write the index to a disk file.
     * @param base_dir: Data file base directory.
     */
    pub fn save_to_disk(&self, base_dir: &Path) -> io::Result<()> {
        let path = layout::data_file_path(base_dir, layout::INDEX_FILE);
        let data = self.to_bytes();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;
        file.write_all_at(&data, 0)?;
        file.sync_all()?;
        Ok(())
    }

    /**
     * @brief: Load the index from a disk file.
     * @param base_dir: Data file base directory.
     * @return: Returns Index on success, None if file does not exist or is corrupted.
     */
    pub fn load_from_disk(base_dir: &Path) -> Option<Self> {
        let path = layout::data_file_path(base_dir, layout::INDEX_FILE);
        let file = File::open(&path).ok()?;
        let len = file.metadata().ok()?.len() as usize;
        if len < 16 {
            return None;
        }
        let mut data = vec![0u8; len];
        file.read_at(&mut data, 0).ok()?;
        Self::from_bytes(&data)
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}
