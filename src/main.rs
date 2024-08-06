use rust_boltdb::db::Db;
use rust_boltdb::tx::Tx;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

fn main() {
    env_logger::init();
    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
    test_rc();
    // next db::create_bucket() method
}

fn test_db() {
    let file_name = "testxx.db".to_string();
    let db = Db::new(&file_name);
    let db = Rc::new(RefCell::new(db));

    let tx = Db::begin(db, true);
    let bucket_name = "abc";
    let bucket = Db::create_bucket(tx.clone(), bucket_name.as_bytes());

    bucket.put(b"k1", b"v1");
}
fn test_rc() {
    let s = Rc::new(RefCell::new(String::from("hello, world")));
    modify(s.clone());
    println!("{:?}", s);
}

fn modify(rc: Rc<RefCell<String>>) {
    println!("Input is {}", rc.borrow());
    let mut x = rc.borrow_mut();
    x.push_str(" Good Good.");
}
