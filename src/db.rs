#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::bucket::{Bucket, InBucket};
use crate::constant::{FILE_MAX_SIZE, MAX_PAGE_ID, PAGE_SIZE};
use crate::cursor::DBCursor;
use crate::node::Node;
use crate::page::Page;
use crate::tx::Tx;
use memmap2::MmapMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::ptr;
use std::rc::Rc;

// 256 M

#[derive(Debug)]
pub struct Db {
    file: File,
    mmap: MmapMut,
    start_ptr: *const u8,
    end_ptr: *const u8,
}

impl Db {
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

impl Db {
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
        let range = mmap.as_ptr_range();
        let db = Db {
            file: file,
            mmap: mmap,
            start_ptr: range.start,
            end_ptr: range.end,
        };
        Db::init(&db);
        log::info!("db is : {:#?}", db);
        db
    }

    // 把Page写入DB中指定的位置
    pub fn write_page(&self, page: &Page) {
        let mut ptr = self.start_ptr();
        let page_id = page.get_page_id();
        if page_id > MAX_PAGE_ID {
            let err_msg = format!(
                "page id {} more then the max page is: {}",
                page_id, MAX_PAGE_ID
            );
            panic!("{}", err_msg);
        }
        unsafe {
            ptr = ptr.add(page_id as usize * PAGE_SIZE);
            let ptr = ptr as *mut Page;
            ptr::write(ptr, page.clone());
        }
    }

    // 从DB中读一个Page
    pub fn read_page(&self, page_id: u64) -> Page {
        let mut ptr = self.start_ptr();
        unsafe {
            let ptr = ptr.add(page_id as usize * PAGE_SIZE);
            let page_ptr = Some(ptr);
            let ptr = ptr as *const Page;
            let mut page = ptr::read(ptr);
            page.set_page_ptr(page_ptr);
            page
        }
    }
}

impl Db {
    pub fn begin(db: Rc<RefCell<Db>>, writable: bool) -> Rc<RefCell<Tx>> {
        // Tx::init(self, writable)
        if writable {
            Db::begin_rwtx(db)
        } else {
            Db::begin_tx(db)
        }
    }

    pub fn begin_rwtx(db: Rc<RefCell<Db>>) -> Rc<RefCell<Tx>> {
        Tx::init(db, true)
    }

    pub fn begin_tx(db: Rc<RefCell<Db>>) -> Rc<RefCell<Tx>> {
        Tx::init(db, false)
    }

    pub fn create_bucket(tx: Rc<RefCell<Tx>>, bucket_name: &[u8]) -> Bucket {
        let in_bucket = InBucket::new(0, 0);
        let page: Option<Page> = None;
        let meta = tx.borrow().meta();
        let bucket_root_page_id = meta.root_bucket().root_page_id();

        let map = tx.borrow().pages(); //.get(&bucket_root_page_id).unwrap();
        let bucket_root_page = map.get(&bucket_root_page_id).unwrap().clone();

        //
        let mut cursor = DBCursor::new(Some(bucket_root_page), None);

        // 1. 找到 bucket, 获取bucket存储数据的 root_page_id
        cursor.seek(bucket_name);

        let nodes: HashMap<u64, Node> = HashMap::new();
        let fill_percent: f64 = 0.75;

        // 用bucket存储数据的 root_page_id初始化
        let root_node: Option<Node> = None;
        // to do ...
        Bucket::new(in_bucket, tx, None, root_node, nodes, fill_percent)
    }

    // 1. 初始化 Meta page
    // 2. 初始化 Bucket Page
    // 3. 初始化 FreeListPage
    pub fn init(db: &Db) {
        // to do
    }
}

#[cfg(test)]
pub mod test {
    use crate::db::Db;
    use crate::page::{Page, BRANCH_PAGE_FLAG, LEAF_PAGE_FLAG};

    #[test]
    fn test_new_db() {
        let file_name = "test.db".to_string();
        let _db = Db::new(&file_name);
        std::fs::remove_file(&file_name).unwrap();
    }

    #[test]
    fn test_write_leaf_page() {
        // env_logger::init();
        let file_name = "test111.db".to_string();
        let mut db = Db::new(&file_name);
        for page_id in 0..10 {
            let page = Page::new(
                page_id,
                LEAF_PAGE_FLAG,
                (page_id + 1) as u16,
                (page_id * page_id) as u32,
            );
            db.write_page(&page);
            log::info!("write page is : {:?}", page);
        }
        log::debug!("hello");
    }

    #[test]
    fn test_read_leaf_page() {
        // env_logger::init();
        let file_name = "test111.db".to_string();
        let mut db = Db::new(&file_name);
        for page_id in 0..10 {
            let page = Page::new(
                page_id,
                LEAF_PAGE_FLAG,
                (page_id + 1) as u16,
                (page_id * page_id) as u32,
            );
            let ret_page = db.read_page(page_id);
            log::info!("ret page is : {:?}", ret_page);
        }
    }

    #[test]
    fn test_write_branch_page() {
        // env_logger::init();
        let file_name = "test111.db".to_string();
        let mut db = Db::new(&file_name);
        for page_id in 0..10 {
            let page = Page::new(
                page_id,
                BRANCH_PAGE_FLAG,
                (page_id + 1) as u16,
                (page_id * page_id) as u32,
            );
            db.write_page(&page);
            log::info!("write page is : {:?}", page);
        }
        log::debug!("hello");
    }

    #[test]
    fn test_read_branch_page() {
        // env_logger::init();
        let file_name = "test111.db".to_string();
        let mut db = Db::new(&file_name);
        for page_id in 0..10 {
            let page = Page::new(
                page_id,
                BRANCH_PAGE_FLAG,
                (page_id + 1) as u16,
                (page_id * page_id) as u32,
            );
            let ret_page = db.read_page(page_id);
            log::info!("ret page is : {:?}", ret_page);
        }
    }
}
