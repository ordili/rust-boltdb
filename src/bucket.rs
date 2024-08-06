#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::meta::Meta;
use crate::node::Node;
use crate::page::Page;
use crate::tx::Tx;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Bucket {
    in_bucket: InBucket,
    tx: Rc<RefCell<Tx>>,
    // 缓存sub Bucket
    // buckets: HashMap<String,  Bucket>, //map[string]*Bucket
    // inline Page ， 数据较少的时候，数据直接直接存储在inline page 中；
    page: Option<Page>,
    //根节点，数据较多时；数据存储在B+Tree中， 这个是B+Tree 的根节点
    root_node: Option<Node>,
    // 缓存Page
    nodes: HashMap<u64, Node>,
    fill_percent: f64,
}

impl Bucket {
    pub fn in_bucket(&self) -> InBucket {
        self.in_bucket
    }
    pub fn tx(&self) -> &Rc<RefCell<Tx>> {
        &self.tx
    }
    pub fn page(&self) -> Option<Page> {
        self.page
    }
    pub fn root_node(&self) -> &Option<Node> {
        &self.root_node
    }
    pub fn nodes(&self) -> &HashMap<u64, Node> {
        &self.nodes
    }
    pub fn fill_percent(&self) -> f64 {
        self.fill_percent
    }

    pub fn set_in_bucket(&mut self, in_bucket: InBucket) {
        self.in_bucket = in_bucket;
    }
    pub fn set_tx(&mut self, tx: Rc<RefCell<Tx>>) {
        self.tx = tx;
    }
    pub fn set_page(&mut self, page: Option<Page>) {
        self.page = page;
    }
    pub fn set_root_node(&mut self, root_node: Option<Node>) {
        self.root_node = root_node;
    }
    pub fn set_nodes(&mut self, nodes: HashMap<u64, Node>) {
        self.nodes = nodes;
    }
    pub fn set_fill_percent(&mut self, fill_percent: f64) {
        self.fill_percent = fill_percent;
    }
    pub fn new(
        in_bucket: InBucket,
        tx: Rc<RefCell<Tx>>,
        page: Option<Page>,
        root_node: Option<Node>,
        nodes: HashMap<u64, Node>,
        fill_percent: f64,
    ) -> Self {
        Self {
            in_bucket,
            tx,
            page,
            root_node,
            nodes,
            fill_percent,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InBucket {
    root_page_id: u64, // page id of the bucket's root-level page
    sequence: u64,     // monotonically incrementing, used by NextSequence()
}
impl InBucket {
    pub fn root_page_id(&self) -> u64 {
        self.root_page_id
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn set_root_page_id(&mut self, root_page_id: u64) {
        self.root_page_id = root_page_id;
    }
    pub fn set_sequence(&mut self, sequence: u64) {
        self.sequence = sequence;
    }
    pub fn new(root: u64, sequence: u64) -> Self {
        Self {
            root_page_id: root,
            sequence,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            root_page_id: 0,
            sequence: 0,
        }
    }
}

impl Bucket {
    pub fn put(&self, key: &[u8], val: &[u8]) {}

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        None
    }
}
