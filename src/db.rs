use crate::page::Page;
use memmap2::MmapMut;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::ptr;

// 256 M
const FILE_MAX_SIZE: u64 = 1024 * 1024 * 256;
pub struct DB {
    file: File,
    mmap: MmapMut,
}

impl DB {
    pub fn new(file_name: &str) -> Self {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(file_name)
            .expect("Open file failed");

        file.set_len(FILE_MAX_SIZE).expect("Set file len error");
        file.seek(SeekFrom::Start(0)).expect("Set start error");

        let mmap = unsafe { MmapMut::map_mut(&file).expect("Mmap file failed.") };
        println!("mmap address is : {:p}", mmap.as_ptr());
        DB {
            file: file,
            mmap: mmap,
        }
    }

    // 把Page写入DB中指定的位置
    pub fn write_page(&mut self, page: &Page, offset: usize) {
        let mut ptr = self.mmap.as_ptr();
        println!("write mmap ptr is :{:p}", ptr);
        unsafe {
            if offset > 0 {
                ptr = ptr.add(offset);
            }
            let ptr = ptr as *mut Page;
            println!("write ptr is :{:p}", ptr);
            ptr::write(ptr, page.clone());
        }
    }

    // 从DB中读一个Page
    pub fn read_page(&mut self, offset: usize) -> Page {
        let mut ptr = self.mmap.as_ptr();
        println!("read mmap ptr is :{:p}", ptr);
        unsafe {
            if offset > 0 {
                ptr = ptr.add(offset);
            }
            let ptr = ptr as *mut Page;
            println!("read ptr is :{:p}", ptr);
            ptr::read(ptr)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::db::DB;
    use crate::page::{Page, Pgid, LEAF_PAGE_FLAG};

    #[test]
    fn test_new_db() {
        let file_name = "test.db".to_string();
        let _db = DB::new(&file_name);
        std::fs::remove_file(&file_name).unwrap();
    }
    #[test]
    fn test_write_page() {
        let file_name = "test111.db".to_string();
        let mut db = DB::new(&file_name);
        let page = Page::new(Pgid::new(32), LEAF_PAGE_FLAG, 32, 12);
        db.write_page(&page, 10);
    }

    #[test]
    fn test_read_page() {
        let file_name = "test111.db".to_string();
        let mut db = DB::new(&file_name);
        let page = Page::new(Pgid::new(32), LEAF_PAGE_FLAG, 32, 12);
        let ret_page = db.read_page(10);
        println!("ret page is : {:?}", ret_page);
        assert_eq!(page, ret_page);
        std::fs::remove_file(&file_name).expect("Delete tempory file failed.");
    }
}
