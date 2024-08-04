use crate::bucket::Bucket;
use crate::db::Db;
use crate::meta::Meta;
use crate::page::Page;
use std::collections::HashMap;

pub struct Tx<'a> {
    writable: bool,
    managed: bool,
    db: &'a Db,
    meta: Meta,
    root: Bucket<'a>,
    pages: HashMap<u64, &'a Page>,
    stats: TxStats,
    write_flag: i32,
}

pub struct TxStats {}

impl TxStats {
    pub fn new() -> Self {
        TxStats {}
    }
}

impl<'a> Tx<'a> {
    pub fn writable(&self) -> bool {
        self.writable
    }
    pub fn meta(&self) -> Meta {
        self.meta
    }
    pub fn db(&self) -> &'a Db {
        self.db
    }
    pub fn set_writable(&mut self, writable: bool) {
        self.writable = writable;
    }
    pub fn set_meta(&mut self, meta: Meta) {
        self.meta = meta;
    }
    pub fn set_db(&mut self, db: &'a Db) {
        self.db = db;
    }

    pub fn managed(&self) -> bool {
        self.managed
    }

    pub fn pages(&self) -> &HashMap<u64, &'a Page> {
        &self.pages
    }
    pub fn stats(&self) -> &TxStats {
        &self.stats
    }
    pub fn write_flag(&self) -> i32 {
        self.write_flag
    }

    pub fn set_managed(&mut self, managed: bool) {
        self.managed = managed;
    }

    pub fn set_pages(&mut self, pages: HashMap<u64, &'a Page>) {
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
        db: &'a Db,
        meta: Meta,
        root: Bucket<'a>,
        pages: HashMap<u64, &'a Page>,
        stats: TxStats,
        write_flag: i32,
    ) -> Self {
        Self {
            writable,
            managed,
            db,
            meta,
            root,
            pages,
            stats,
            write_flag,
        }
    }
    pub fn root(&self) -> &Bucket<'a> {
        &self.root
    }

    pub fn set_root(&mut self, root: Bucket<'a>) {
        self.root = root;
    }
}

impl<'a> Tx<'a> {
    pub fn init(db: &'a Db, writable: bool) -> Self {
        let managed = false;
        let mut meta_page = db.read_page(0);
        let meta = Meta::from_page(&mut meta_page);
        let root = Bucket::new();
        // root.init(&meta);
        let pages: HashMap<u64, &'a Page> = HashMap::new();
        let stats: TxStats = TxStats::new();
        let write_flag = 1;
        Tx::new(writable, managed, db, meta, root, pages, stats, write_flag)
    }
    // to do
    pub fn new_bucket(&self) -> Bucket {
        let bucket = Bucket::new();
        bucket
    }
}
