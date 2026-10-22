/**
 * @brief: Buffer pool, providing an LRU-based in-memory page cache.
 *
 * All page accesses go through page_id, with automatic cache hit/miss handling and LRU updates.
 * Evicted dirty pages are automatically written back to disk.
 */

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use crate::storage::page::{Page, PageId};
use crate::storage::page_manager::PageManager;

/// Default buffer pool capacity (number of pages).
const DEFAULT_CAPACITY: usize = 64;

/**
 * @brief: LRU page cache.
 */
pub struct BufferPool {
    inner: Mutex<Inner>,
    page_manager: PageManager,
}

struct Inner {
    pages: HashMap<PageId, Page>,
    lru: VecDeque<PageId>,
    capacity: usize,
}

impl BufferPool {
    /**
     * @brief: Create a buffer pool.
     */
    pub fn new(page_manager: PageManager) -> Self {
        Self {
            inner: Mutex::new(Inner {
                pages: HashMap::new(),
                lru: VecDeque::new(),
                capacity: DEFAULT_CAPACITY,
            }),
            page_manager,
        }
    }

    /// Ensure page is in cache (load from disk if not), and update LRU.
    fn ensure_cached(&self, page_id: PageId) -> Result<(), String> {
        let mut inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;

        // Already in cache, just update LRU
        if inner.pages.contains_key(&page_id) {
            if let Some(pos) = inner.lru.iter().position(|id| *id == page_id) {
                inner.lru.remove(pos);
            }
            inner.lru.push_back(page_id);
            return Ok(());
        }

        // Not in cache, load from disk
        let page = self
            .page_manager
            .read_page(page_id)
            .map_err(|e| format!("read page {:?}: {}", page_id, e))?;

        // Eviction
        while inner.pages.len() >= inner.capacity {
            let victim_id = self.find_victim(&inner)?;
            if let Some(victim) = inner.pages.remove(&victim_id) {
                if victim.dirty {
                    self.page_manager
                        .write_page(&victim)
                        .map_err(|e| format!("evict write: {}", e))?;
                }
            }
            inner.lru.retain(|id| *id != victim_id);
        }

        inner.pages.insert(page_id, page);
        inner.lru.push_back(page_id);
        Ok(())
    }

    fn find_victim(&self, inner: &Inner) -> Result<PageId, String> {
        for id in &inner.lru {
            if let Some(p) = inner.pages.get(id) {
                if p.pin_count == 0 {
                    return Ok(*id);
                }
            }
        }
        Err("all pages pinned".into())
    }

    /**
     * @brief: Allocate a new page (expand file and add to cache).
     */
    pub fn allocate_page(&self, file_tag: u8) -> Result<PageId, String> {
        let page_id = self
            .page_manager
            .allocate_page(file_tag)
            .map_err(|e| format!("allocate: {}", e))?;

        let mut inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;
        while inner.pages.len() >= inner.capacity {
            let victim_id = self.find_victim(&inner)?;
            if let Some(victim) = inner.pages.remove(&victim_id) {
                if victim.dirty {
                    self.page_manager
                        .write_page(&victim)
                        .map_err(|e| format!("evict write: {}", e))?;
                }
            }
            inner.lru.retain(|id| *id != victim_id);
        }

        let page = Page::new(page_id);
        inner.pages.insert(page_id, page);
        inner.lru.push_back(page_id);
        Ok(page_id)
    }

    /**
     * @brief: Ensure that a page with the specified page number exists on disk.
     */
    pub fn ensure_page_exists(&self, file_tag: u8, page_num: u64) -> Result<(), String> {
        self.page_manager
            .ensure_page_exists(file_tag, page_num)
            .map_err(|e| format!("ensure page: {}", e))
    }

    /**
     * @brief: Append a record to the specified page.
     * @return: Returns the record slot index on success.
     */
    pub fn write_record(&self, page_id: PageId, data: &[u8]) -> Result<u16, String> {
        self.ensure_cached(page_id)?;
        let mut inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;
        let page = inner.pages.get_mut(&page_id).unwrap();
        let slot = page.write_record(data);
        Ok(slot)
    }

    /**
     * @brief: Read a record from the specified page.
     */
    pub fn read_record(&self, page_id: PageId, slot: u16) -> Result<Vec<u8>, String> {
        self.ensure_cached(page_id)?;
        let inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;
        let page = inner.pages.get(&page_id).unwrap();
        page.read_record(slot)
            .map(|d| d.to_vec())
            .ok_or_else(|| format!("slot {} not found in {:?}", slot, page_id))
    }

    /**
     * @brief: Read raw byte data from a page.
     * @param page_id: Target page.
     * @param offset: Offset within the page.
     * @param len: Length to read.
     */
    pub fn read_bytes(&self, page_id: PageId, offset: usize, len: usize) -> Result<Vec<u8>, String> {
        self.ensure_cached(page_id)?;
        let inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;
        let page = inner.pages.get(&page_id).unwrap();
        if offset + len > page.data.len() {
            return Err(format!("read out of bounds: offset={}, len={}", offset, len));
        }
        Ok(page.data[offset..offset + len].to_vec())
    }

    /**
     * @brief: Write raw bytes at a specified offset within a page.
     */
    pub fn write_bytes(&self, page_id: PageId, offset: usize, data: &[u8]) -> Result<(), String> {
        self.ensure_cached(page_id)?;
        let mut inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;
        let page = inner.pages.get_mut(&page_id).unwrap();
        if offset + data.len() > page.data.len() {
            return Err(format!("write out of bounds: offset={}, len={}", offset, data.len()));
        }
        page.data[offset..offset + data.len()].copy_from_slice(data);
        page.dirty = true;
        Ok(())
    }

    /**
     * @brief: Check whether the page has enough space for a new record.
     */
    pub fn has_space(&self, page_id: PageId, needed: usize) -> Result<bool, String> {
        self.ensure_cached(page_id)?;
        let inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;
        let page = inner.pages.get(&page_id).unwrap();
        Ok(page.free_space() >= needed)
    }

    /**
     * @brief: Write bytes at a specified record offset within a page.
     *
     * Used for in-place MVCC header field updates during commit/abort.
     * @param page_id: Target page.
     * @param slot: Record slot.
     * @param offset: Byte offset within the record data (0 = first byte of MVCC header).
     * @param data: Data to write.
     */
    pub fn write_record_bytes(
        &self,
        page_id: PageId,
        slot: u16,
        offset: usize,
        data: &[u8],
    ) -> Result<(), String> {
        self.ensure_cached(page_id)?;
        let mut inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;
        let page = inner.pages.get_mut(&page_id).unwrap();
        let record_offset = page
            .record_data_offset(slot)
            .ok_or_else(|| format!("slot {} not found in {:?}", slot, page_id))?;
        let write_offset = record_offset + offset;
        if write_offset + data.len() > page.data.len() {
            return Err(format!(
                "write out of bounds: offset={}, len={}",
                write_offset,
                data.len()
            ));
        }
        page.data[write_offset..write_offset + data.len()].copy_from_slice(data);
        page.dirty = true;
        Ok(())
    }

    /**
     * @brief: Sync all dirty pages and call fsync.
     */
    pub fn sync_all(&self) -> Result<(), String> {
        let mut inner = self.inner.lock().map_err(|e| format!("lock: {}", e))?;
        for page in inner.pages.values_mut() {
            if page.dirty {
                self.page_manager
                    .write_page(page)
                    .map_err(|e| format!("sync write: {}", e))?;
                page.dirty = false;
            }
        }
        self.page_manager
            .sync_all()
            .map_err(|e| format!("sync files: {}", e))
    }
}
