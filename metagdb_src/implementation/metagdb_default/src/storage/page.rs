/**
 * @brief: In-memory page representation, containing a 4KB data buffer, page header fields, and a dirty flag.
 */

use crate::storage::layout;

/**
 * @brief: Global page identifier, composed of a file tag and page number.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageId {
    pub file_tag: u8,
    pub page_num: u64,
}

impl PageId {
    /**
     * @brief: Construct a new PageId.
     */
    pub fn new(file_tag: u8, page_num: u64) -> Self {
        Self { file_tag, page_num }
    }
}

/**
 * @brief: In-memory 4KB data page.
 */
pub struct Page {
    /// Raw page data buffer.
    pub data: [u8; layout::PAGE_SIZE],
    /// Global identifier for this page.
    pub page_id: PageId,
    /// Dirty flag, indicating in-memory page data differs from disk.
    pub dirty: bool,
    /// Pin count, cannot be evicted when greater than 0.
    pub pin_count: u32,
}

impl Page {
    /**
     * @brief: Create a new empty page, initializing the page header.
     * @param page_id: Page identifier.
     */
    pub fn new(page_id: PageId) -> Self {
        let mut page = Self {
            data: [0u8; layout::PAGE_SIZE],
            page_id,
            dirty: false,
            pin_count: 0,
        };
        page.init_header();
        page
    }

    /**
     * @brief: Construct a page object from bytes read from disk.
     *
     * If the page header is detected as uninitialized (free_offset less than PAGE_HEADER_SIZE),
     * the header is automatically padded to prevent writing records into the header area, which would cause data corruption.
     * @param page_id: Page identifier.
     * @param bytes: Raw 4KB data read from disk.
     */
    pub fn from_bytes(page_id: PageId, bytes: &[u8; layout::PAGE_SIZE]) -> Self {
        let mut page = Self {
            data: *bytes,
            page_id,
            dirty: false,
            pin_count: 0,
        };
        // If free_offset is 0 (all-zeros page, first creation), initialize page header
        if page.free_offset() < layout::PAGE_HEADER_SIZE as u16 {
            page.set_free_offset(layout::PAGE_HEADER_SIZE as u16);
            page.set_record_count(0);
        }
        page
    }

    /// Initialize page header to zero values.
    fn init_header(&mut self) {
        self.set_record_count(0);
        self.set_free_offset(layout::PAGE_HEADER_SIZE as u16);
        self.data[layout::HEADER_OFFSET_FLAGS] = 0;
    }

    /**
     * @brief: Read the number of records in the page.
     */
    pub fn record_count(&self) -> u16 {
        u16::from_le_bytes([
            self.data[layout::HEADER_OFFSET_RECORD_COUNT],
            self.data[layout::HEADER_OFFSET_RECORD_COUNT + 1],
        ])
    }

    /**
     * @brief: Set the number of records in the page.
     */
    pub fn set_record_count(&mut self, count: u16) {
        self.data[layout::HEADER_OFFSET_RECORD_COUNT
            ..layout::HEADER_OFFSET_RECORD_COUNT + 2]
            .copy_from_slice(&count.to_le_bytes());
    }

    /**
     * @brief: Read the current free area start offset.
     */
    pub fn free_offset(&self) -> u16 {
        u16::from_le_bytes([
            self.data[layout::HEADER_OFFSET_FREE_OFFSET],
            self.data[layout::HEADER_OFFSET_FREE_OFFSET + 1],
        ])
    }

    /**
     * @brief: Set the free area start offset.
     */
    pub fn set_free_offset(&mut self, offset: u16) {
        self.data[layout::HEADER_OFFSET_FREE_OFFSET
            ..layout::HEADER_OFFSET_FREE_OFFSET + 2]
            .copy_from_slice(&offset.to_le_bytes());
    }

    /**
     * @brief: Remaining bytes available in the current page for new records (including 2-byte length prefix).
     */
    pub fn free_space(&self) -> usize {
        layout::PAGE_SIZE.saturating_sub(self.free_offset() as usize)
    }

    /**
     * @brief: Append a record to the page.
     * @param data: Record bytes.
     * @return: Slot index of the record in the page.
     */
    pub fn write_record(&mut self, data: &[u8]) -> u16 {
        let slot = self.record_count();
        let offset = self.free_offset() as usize;
        let len = data.len() as u16;
        self.data[offset..offset + 2].copy_from_slice(&len.to_le_bytes());
        self.data[offset + 2..offset + 2 + data.len()].copy_from_slice(data);
        self.set_free_offset((offset + 2 + data.len()) as u16);
        self.set_record_count(slot + 1);
        self.dirty = true;
        slot
    }

    /**
     * @brief: Read record data by slot index.
     * @param slot: Record slot (0-based).
     * @return: Record byte slice if present.
     */
    pub fn read_record(&self, slot: u16) -> Option<&[u8]> {
        if slot >= self.record_count() {
            return None;
        }
        let mut offset = layout::PAGE_HEADER_SIZE;
        for i in 0..=slot {
            let len = u16::from_le_bytes([self.data[offset], self.data[offset + 1]]) as usize;
            if i == slot {
                return Some(&self.data[offset + 2..offset + 2 + len]);
            }
            offset += 2 + len;
        }
        None
    }

    /**
     * @brief: Find the start offset of record data by slot (skipping the 2-byte length prefix).
     *
     * The returned offset points to the first byte of the record payload (i.e., the start of the MVCC header),
     * and can be used for in-place MVCC header field updates.
     * @param slot: Record slot (0-based).
     * @return: Byte offset within the page if present.
     */
    pub fn record_data_offset(&self, slot: u16) -> Option<usize> {
        if slot >= self.record_count() {
            return None;
        }
        let mut offset = layout::PAGE_HEADER_SIZE;
        for i in 0..=slot {
            let len = u16::from_le_bytes([self.data[offset], self.data[offset + 1]]) as usize;
            if i == slot {
                return Some(offset + 2); // Skip length prefix, point to start of record data
            }
            offset += 2 + len;
        }
        None
    }

    /**
     * @brief: Pin the page (increment reference count).
     */
    #[allow(dead_code)]
    pub fn pin(&mut self) {
        self.pin_count += 1;
    }

    /**
     * @brief: Unpin the page (decrement reference count).
     */
    #[allow(dead_code)]
    pub fn unpin(&mut self) {
        if self.pin_count > 0 {
            self.pin_count -= 1;
        }
    }
}
