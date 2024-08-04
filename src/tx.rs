use crate::bucket::Bucket;
use crate::db::Db;
use crate::meta::Meta;
pub struct Tx<'a> {
    writable: bool,
    meta: Meta,
    db: &'a Db,
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
}

impl<'a> Tx<'a> {
    pub fn init(db: &'a Db, writable: bool) -> Self {
        let mut meta_page = db.read_page(0);
        let meta = Meta::from_page(&mut meta_page);
        Self { writable, db, meta }
    }

    // to do
    pub fn new_bucket(&self) {
        // to do...
    }
}
