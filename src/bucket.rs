#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::meta::Meta;
use crate::node::Node;
use crate::page::Page;
use crate::tx::Tx;
use std::collections::HashMap;

pub struct Bucket<'a> {
    in_bucket: InBucket,
    // 因为Bucket 与 Tx 相互包含；故而采用Option 来规避初始化的问题；
    tx: Option<&'a Tx<'a>>,
    // 缓存sub Bucket
    buckets: HashMap<String, &'a Bucket<'a>>, //map[string]*Bucket
    // inline Page ， 数据较少的时候，数据直接直接存储在inline page 中；
    page: Option<&'a Page>,

    //根节点，数据较多时；数据存储在B+Tree中， 这个是B+Tree 的根节点
    root_node: Option<&'a Node>,
    // 缓存Page
    nodes: HashMap<u64, &'a Node>,
    fill_percent: f64,
}

impl<'a> Bucket<'a> {
    pub fn root_node(&self) -> Option<&'a Node> {
        self.root_node
    }
    pub fn bucket_map(&self) -> &HashMap<String, &'a Bucket<'a>> {
        &self.buckets
    }

    pub fn set_root_node(&mut self, root_node: Option<&'a Node>) {
        self.root_node = root_node;
    }
    pub fn set_bucket_map(&mut self, bucket_map: HashMap<String, &'a Bucket<'a>>) {
        self.buckets = bucket_map;
    }
    pub fn in_bucket(&self) -> InBucket {
        self.in_bucket
    }

    pub fn buckets(&self) -> &HashMap<String, &'a Bucket<'a>> {
        &self.buckets
    }
    pub fn page(&self) -> Option<&'a Page> {
        self.page
    }
    pub fn nodes(&self) -> &HashMap<u64, &'a Node> {
        &self.nodes
    }
    pub fn fill_percent(&self) -> f64 {
        self.fill_percent
    }
    pub fn set_in_bucket(&mut self, in_bucket: InBucket) {
        self.in_bucket = in_bucket;
    }
    pub fn set_buckets(&mut self, buckets: HashMap<String, &'a Bucket<'a>>) {
        self.buckets = buckets;
    }
    pub fn set_page(&mut self, page: Option<&'a Page>) {
        self.page = page;
    }
    pub fn set_nodes(&mut self, nodes: HashMap<u64, &'a Node>) {
        self.nodes = nodes;
    }
    pub fn set_fill_percent(&mut self, fill_percent: f64) {
        self.fill_percent = fill_percent;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InBucket {
    root: u64,     // page id of the bucket's root-level page
    sequence: u64, // monotonically incrementing, used by NextSequence()
}

impl InBucket {
    pub fn root(&self) -> u64 {
        self.root
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn set_root(&mut self, root: u64) {
        self.root = root;
    }
    pub fn set_sequence(&mut self, sequence: u64) {
        self.sequence = sequence;
    }
    pub fn new(root: u64, sequence: u64) -> Self {
        Self { root, sequence }
    }

    pub fn new_empty() -> Self {
        Self {
            root: 0,
            sequence: 0,
        }
    }
}

impl<'a> Bucket<'a> {
    pub fn new() -> Self {
        Self {
            in_bucket: InBucket::new_empty(),
            tx: None,
            page: None,
            root_node: None,
            buckets: HashMap::new(),
            nodes: HashMap::new(),
            fill_percent: 0.75,
        }
    }

    // to do ..
    pub fn init(&'a mut self, meta: &'a Meta) {}
    pub fn put(key: &[u8], val: &[u8]) {}

    pub fn get(key: &[u8]) -> Option<Vec<u8>> {
        None
    }
}
