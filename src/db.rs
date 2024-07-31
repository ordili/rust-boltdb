#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::constant::{FILE_MAX_SIZE, PAGE_SIZE};
use crate::page::Page;
use memmap2::MmapMut;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::ptr;

// 256 M

pub struct DB {
    file: File,
    mmap: MmapMut,
    start_ptr : *const u8,
    end_ptr : *const u8,
}

impl DB{
    pub fn file(&self) -> &File {
        &self.file
    }
    pub fn mmap(&self) -> &MmapMut {
        &self.mmap
    }
    pub fn start_ptr(&self) -> *const u8 {
        self.start_ptr
    }
    pub fn end_ptr(&self) -> *const u8 {
        self.end_ptr
    }

    pub fn set_file(&mut self, file: File) {
        self.file = file;
    }
    pub fn set_mmap(&mut self, mmap: MmapMut) {
        self.mmap = mmap;
    }
    pub fn set_start_ptr(&mut self, start_ptr: *const u8) {
        self.start_ptr = start_ptr;
    }
    pub fn set_end_ptr(&mut self, end_ptr: *const u8) {
        self.end_ptr = end_ptr;
    }
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
        let range = mmap.as_ptr_range();
        DB {
            file: file,
            mmap: mmap,
            start_ptr : range.start,
            end_ptr: range.end
        }
    }

    // 把Page写入DB中指定的位置
    pub fn write_page(&mut self, page: &Page) {
        let mut ptr = self.start_ptr();
        let page_id = page.get_page_id();
        unsafe {
            ptr = ptr.add(page_id as usize * PAGE_SIZE);
            let ptr = ptr as *mut Page;
            ptr::write(ptr, page.clone());
        }
    }

    // 从DB中读一个Page
    pub fn read_page(&mut self, page_id: u64) -> Page {
        let mut ptr = self.start_ptr();
        unsafe {
            ptr = ptr.add(page_id as usize * PAGE_SIZE);
            let ptr = ptr as *const Page;
            ptr::read(ptr)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::db::DB;
    use crate::page::{Page, LEAF_PAGE_FLAG};

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
        for page_id in 0..10 {
            let page = Page::new(page_id, LEAF_PAGE_FLAG, (page_id+1) as u16, (page_id*page_id) as u32);
            db.write_page(&page);
        }
    }

    #[test]
    fn test_read_page() {
        let file_name = "test111.db".to_string();
        let mut db = DB::new(&file_name);
        for page_id in 0..10 {
            let page = Page::new(page_id, LEAF_PAGE_FLAG, (page_id+1) as u16, (page_id*page_id) as u32);
            let ret_page = db.read_page(page_id);
            println!("ret page is : {:?}", ret_page);
            assert_eq!(page, ret_page);
        }
    }
}
