/**
 * @brief: Page manager, responsible for opening/creating data files and page-level reads/writes.
 *
 * Uses Unix FileExt (pread/pwrite) for lock-free positioned I/O,
 * all methods accept &self without internal mutex.
 */

use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

use crate::storage::layout;
use crate::storage::page::{Page, PageId};

/**
 * @brief: Manages page-level reads/writes for three data files: vertices, edges, adjacency.
 */
pub struct PageManager {
    vertices_file: File,
    edges_file: File,
    adj_file: File,
    #[allow(dead_code)]
    vertices_path: PathBuf,
    #[allow(dead_code)]
    edges_path: PathBuf,
    #[allow(dead_code)]
    adj_path: PathBuf,
}

impl PageManager {
    /**
     * @brief: Open or create all data files.
     * @param base_dir: Base directory for data file storage.
     * @return: Returns PageManager instance on success.
     */
    pub fn open_or_create(base_dir: &Path) -> io::Result<Self> {
        std::fs::create_dir_all(base_dir)?;

        let vp = layout::data_file_path(base_dir, layout::VERTICES_FILE);
        let ep = layout::data_file_path(base_dir, layout::EDGES_FILE);
        let ap = layout::data_file_path(base_dir, layout::ADJ_FILE);

        let vf = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&vp)?;
        let ef = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&ep)?;
        let af = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&ap)?;

        Ok(Self {
            vertices_file: vf,
            edges_file: ef,
            adj_file: af,
            vertices_path: vp,
            edges_path: ep,
            adj_path: ap,
        })
    }

    /**
     * @brief: Get the corresponding file handle by file tag.
     */
    fn file_for(&self, file_tag: u8) -> &File {
        match file_tag {
            layout::FILE_VERTICES => &self.vertices_file,
            layout::FILE_EDGES => &self.edges_file,
            layout::FILE_ADJ => &self.adj_file,
            _ => &self.vertices_file, // unreachable
        }
    }

    /**
     * @brief: Read a specified page from disk.
     * @param page_id: Target page identifier.
     * @return: Returns an in-memory Page object on success.
     */
    pub fn read_page(&self, page_id: PageId) -> io::Result<Page> {
        let file = self.file_for(page_id.file_tag);
        let offset = page_id.page_num * layout::PAGE_SIZE as u64;
        let mut buf = [0u8; layout::PAGE_SIZE];
        let bytes_read = file.read_at(&mut buf, offset)?;
        if bytes_read < layout::PAGE_SIZE {
            for b in buf.iter_mut().skip(bytes_read) {
                *b = 0;
            }
        }
        Ok(Page::from_bytes(page_id, &buf))
    }

    /**
     * @brief: Write an in-memory page back to disk.
     * @param page: Page to write.
     */
    pub fn write_page(&self, page: &Page) -> io::Result<()> {
        let file = self.file_for(page.page_id.file_tag);
        let offset = page.page_id.page_num * layout::PAGE_SIZE as u64;
        file.write_all_at(&page.data, offset)
    }

    /**
     * @brief: Allocate a new page in the specified file.
     * @param file_tag: Target file tag.
     * @return: Returns the PageId of the new page on success.
     */
    pub fn allocate_page(&self, file_tag: u8) -> io::Result<PageId> {
        let file = self.file_for(file_tag);
        let len = file.metadata()?.len();
        let page_num = len / layout::PAGE_SIZE as u64;
        file.set_len(len + layout::PAGE_SIZE as u64)?;
        let page = Page::new(PageId::new(file_tag, page_num));
        let offset = page_num * layout::PAGE_SIZE as u64;
        file.write_all_at(&page.data, offset)?;
        file.sync_all()?;
        Ok(PageId::new(file_tag, page_num))
    }

    /**
     * @brief: Ensure the specified file has enough pages to cover the target page number.
     * @param file_tag: File tag.
     * @param page_num: Minimum page number that must exist (inclusive).
     */
    pub fn ensure_page_exists(&self, file_tag: u8, page_num: u64) -> io::Result<()> {
        let file = self.file_for(file_tag);
        let len = file.metadata()?.len();
        let required_len = (page_num + 1) * layout::PAGE_SIZE as u64;
        if len < required_len {
            file.set_len(required_len)?;
        }
        Ok(())
    }

    /**
     * @brief: Sync all files to disk.
     */
    pub fn sync_all(&self) -> io::Result<()> {
        self.vertices_file.sync_all()?;
        self.edges_file.sync_all()?;
        self.adj_file.sync_all()?;
        Ok(())
    }
}
