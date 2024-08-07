#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::page::Page;
use crate::tx::TxId;
use std::collections::HashMap;
use std::{mem, ptr};

// #[derive(Copy, Clone)]
pub struct Freelist {
    ids: Vec<u64>,                   // all free and available free page ids.
    allocs: HashMap<u64, TxId>,      // mapping of Txid that allocated a pgid.
    cache: HashMap<u64, TxId>,       // fast lookup of all free and pending page ids.
    forward_map: HashMap<u64, u64>,  // key is start pgid, value is its span size
    backward_map: HashMap<u64, u64>, // key is end pgid, value is its span size
}

impl Freelist {
    pub fn set_ids(&mut self, ids: Vec<u64>) {
        self.ids = ids;
    }
    pub fn set_allocs(&mut self, allocs: HashMap<u64, TxId>) {
        self.allocs = allocs;
    }
    pub fn set_cache(&mut self, cache: HashMap<u64, TxId>) {
        self.cache = cache;
    }
    pub fn set_forward_map(&mut self, forward_map: HashMap<u64, u64>) {
        self.forward_map = forward_map;
    }
    pub fn set_backward_map(&mut self, backward_map: HashMap<u64, u64>) {
        self.backward_map = backward_map;
    }

    pub fn ids(&self) -> &Vec<u64> {
        &self.ids
    }
    pub fn allocs(&self) -> &HashMap<u64, TxId> {
        &self.allocs
    }
    pub fn cache(&self) -> &HashMap<u64, TxId> {
        &self.cache
    }
    pub fn forward_map(&self) -> &HashMap<u64, u64> {
        &self.forward_map
    }
    pub fn backward_map(&self) -> &HashMap<u64, u64> {
        &self.backward_map
    }
}

impl Freelist {
    pub fn new(ids: Vec<u64>) -> Self {
        Self {
            ids,
            allocs: HashMap::new(),
            cache: HashMap::new(),
            forward_map: HashMap::new(),
            backward_map: HashMap::new(),
        }
    }
    pub fn write(&self, page: &mut Page) {
        let ptr = page.skip_page_header();
        unsafe {
            let size = self.ids.len();
            let mut size_ptr = ptr as *mut usize;
            // write size
            ptr::write(size_ptr, size);

            // write page id list
            let mut val_ptr = ptr.add(mem::size_of::<usize>());
            let val_ptr = val_ptr as *mut u64;
            ptr::copy(self.ids.as_ptr(), val_ptr, size);
        }
    }

    // 从Page中读取Page
    pub fn from_page(page: &mut Page) -> Self {
        let ptr = page.skip_page_header();
        unsafe {
            //read size
            let size_ptr = ptr as *mut usize;
            let size = ptr::read(size_ptr);

            // read page id list
            let mut ids = vec![0u64; size];
            let mut val_ptr = ptr.add(mem::size_of::<usize>());
            let val_ptr = val_ptr as *mut u64;
            ptr::copy(val_ptr, ids.as_mut_ptr(), size);

            Freelist::new(ids)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::db::Db;
    use crate::db_utils;
    use crate::freelist::Freelist;
    use crate::page::{Page, FREELIST_PAGE_FLAG};

    #[test]
    fn test_write() {
        let file_name = "test_freelist.db";
        let mut db = Db::new(file_name);
        let page_id = 3;
        let mut page = Page::new(page_id, FREELIST_PAGE_FLAG, 0, 0);
        db_utils::write_page(&page);
        let mut page = db_utils::read_page(page_id);
        println!("{:?}", page);
        let ids = vec![1, 3, 5, 7, 9, 10, 11, 13, 15];
        let free_list = Freelist::new(ids);
        free_list.write(&mut page);
    }

    #[test]
    fn test_from_page() {
        let file_name = "test_freelist.db";
        let mut db = Db::new(file_name);
        let page_id = 3;

        let mut page = db_utils::read_page(page_id);
        println!("{:?}", page);
        let ids = vec![1, 3, 5, 7, 9, 10, 11, 13, 15];
        let free_list = Freelist::new(ids);

        let read_free_list = Freelist::from_page(&mut page);

        assert_eq!(free_list.ids(), read_free_list.ids());
    }
}
