#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::bucket::{Bucket, InBucket};
use crate::db::Db;
use crate::db_utils;
use crate::meta::Meta;
use crate::node::Node;
use crate::page::Page;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub struct TxId(u64);

// txPending holds a list of pgids and corresponding allocation txns
// that are pending to be freed.
pub struct TxPending {
    ids: Vec<u64>,            // page id list
    alloc_tx: Vec<TxId>,      // txids allocating the ids
    last_release_begin: TxId, // beginning txid of last matching releaseRange
}

pub struct Tx {
    writable: bool,
    managed: bool,
    db: Rc<RefCell<Db>>,
    meta: Meta,
    pages: HashMap<u64, Rc<RefCell<Page>>>,
    stats: TxStats,
    write_flag: i32,
    bucket_to_page_map: HashMap<String, u64>, // Key bucket name; Val 这个Bucket存放数据的Root Page Id
}

pub struct TxStats {}

impl TxStats {
    pub fn new() -> Self {
        TxStats {}
    }
}

impl Tx {
    pub fn writable(&self) -> bool {
        self.writable
    }
    pub fn managed(&self) -> bool {
        self.managed
    }
    pub fn db(&self) -> &Rc<RefCell<Db>> {
        &self.db
    }
    pub fn meta(&self) -> Meta {
        self.meta
    }
    pub fn pages(&mut self) -> &mut HashMap<u64, Rc<RefCell<Page>>> {
        &mut self.pages
    }
    pub fn stats(&self) -> &TxStats {
        &self.stats
    }
    pub fn write_flag(&self) -> i32 {
        self.write_flag
    }

    pub fn set_writable(&mut self, writable: bool) {
        self.writable = writable;
    }
    pub fn set_managed(&mut self, managed: bool) {
        self.managed = managed;
    }
    pub fn set_db(&mut self, db: Rc<RefCell<Db>>) {
        self.db = db;
    }
    pub fn set_meta(&mut self, meta: Meta) {
        self.meta = meta;
    }
    pub fn set_pages(&mut self, pages: HashMap<u64, Rc<RefCell<Page>>>) {
        self.pages = pages;
    }
    pub fn set_stats(&mut self, stats: TxStats) {
        self.stats = stats;
    }
    pub fn set_write_flag(&mut self, write_flag: i32) {
        self.write_flag = write_flag;
    }
    pub fn new(
        writable: bool,
        managed: bool,
        db: Rc<RefCell<Db>>,
        meta: Meta,
        pages: HashMap<u64, Rc<RefCell<Page>>>,
        stats: TxStats,
        write_flag: i32,
        bucket_to_page_map: HashMap<String, u64>,
    ) -> Self {
        Self {
            writable,
            managed,
            db,
            meta,
            pages,
            stats,
            write_flag,
            bucket_to_page_map,
        }
    }
}

impl Tx {
    pub fn init(db: Rc<RefCell<Db>>, writable: bool) -> Rc<RefCell<Tx>> {
        let managed = false;
        let mut meta_page = db_utils::read_page(0);
        let meta = Meta::from_page(&mut meta_page);
        let pages: HashMap<u64, Rc<RefCell<Page>>> = HashMap::new();
        let stats: TxStats = TxStats::new();
        let write_flag = 0;
        let bucket_to_page_map: HashMap<String, u64> = HashMap::new();

        let tx = Tx::new(
            writable,
            managed,
            db,
            meta,
            pages,
            stats,
            write_flag,
            bucket_to_page_map,
        );

        Rc::new(RefCell::new(tx))
    }
}
