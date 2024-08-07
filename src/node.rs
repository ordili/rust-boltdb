#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::common::{read_inner_node_from_page, write_inner_node_to_page};
use crate::page::{BranchPageElement, LeafPageElement, Page, BRANCH_PAGE_FLAG, LEAF_PAGE_FLAG};
use std::any::Any;
use std::ptr::NonNull;
use std::vec;

// 代表磁盘中的一个Page；数据存储在内存中
#[derive(Debug, PartialEq)]
pub struct Node {
    is_leaf: bool,
    unbalanced: bool,
    spilled: bool,
    //叶子节点没有Key； 分支节点才有Key
    key: Option<Vec<u8>>,
    page_id: u64,
    // 叶子节点，没有children_list_ptr
    // children_list_ptr   :Option<NonNull<Vec<Node>>>,
    inner_node_list: Vec<InnerNode>,
}

// 实现属性的get set
impl Node {
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }
    pub fn unbalanced(&self) -> bool {
        self.unbalanced
    }
    pub fn spilled(&self) -> bool {
        self.spilled
    }

    pub fn page_id(&self) -> u64 {
        self.page_id
    }

    pub fn set_is_leaf(&mut self, is_leaf: bool) {
        self.is_leaf = is_leaf;
    }
    pub fn set_unbalanced(&mut self, unbalanced: bool) {
        self.unbalanced = unbalanced;
    }
    pub fn set_spilled(&mut self, spilled: bool) {
        self.spilled = spilled;
    }

    pub fn set_page_id(&mut self, pgid: u64) {
        self.page_id = pgid;
    }
    pub fn key(&self) -> &Option<Vec<u8>> {
        &self.key
    }
    pub fn inner_node_list(&self) -> &Vec<InnerNode> {
        &self.inner_node_list
    }

    pub fn set_key(&mut self, key: Option<Vec<u8>>) {
        self.key = key;
    }
    pub fn set_inner_node_list(&mut self, inner_node_list: Vec<InnerNode>) {
        self.inner_node_list = inner_node_list;
    }
}

// 实现实例方法
impl Node {
    pub fn new() -> Self {
        Self {
            is_leaf: false,
            unbalanced: false,
            spilled: false,
            //叶子节点没有Key； 分支节点才有Key
            key: None,
            page_id: 0,
            // 叶子节点，没有children_list_ptr
            // children_list_ptr   :Option<NonNull<Vec<Node>>>,
            inner_node_list: vec![],
        }
    }

    // 用Page初始化Node， 数据从Page中读取；Page中的数据从磁盘中读取；
    pub fn read(&mut self, page: &mut Page) {
        self.set_is_leaf(page.is_leaf_page());
        self.set_page_id(page.get_page_id());
        let inner_node_list = read_inner_node_from_page(page);

        // branch 才有key； leaf node 里面，没有key
        let key = if page.is_branch_page() && inner_node_list.len() > 0 {
            // 保存最小的Key
            let key = &inner_node_list.get(0).unwrap().key;
            Some(key.clone())
        } else {
            None
        };
        self.set_key(key);
        self.set_inner_node_list(inner_node_list);
    }

    // 把内存中Node的数据写入Page； Page中的数据写入磁盘；
    // 只考虑 branch page and leaf page; 未考虑其他Page，如  meta page,free list page 等page
    pub fn write(&self, page: &mut Page) {
        let is_leaf = self.is_leaf();
        let page_id = self.page_id();

        let inner_node_list = self.inner_node_list();

        page.set_count(inner_node_list.len() as u16);
        page.set_page_id(self.page_id());

        let flag = if is_leaf {
            LEAF_PAGE_FLAG
        } else {
            BRANCH_PAGE_FLAG
        };
        page.set_flags(flag);
        write_inner_node_to_page(inner_node_list, page);
    }

    // to do ...
    pub fn search_key(&self, key: &[u8]) -> Option<Vec<u8>> {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct InnerNode {
    flags: u16,
    page_id: u64,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl InnerNode {
    pub fn new(flags: u16, page_id: u64, key: Vec<u8>, value: Vec<u8>) -> Self {
        Self {
            flags,
            page_id,
            key,
            value,
        }
    }
}

impl InnerNode {
    pub fn flags(&self) -> u16 {
        self.flags
    }
    pub fn page_id(&self) -> u64 {
        self.page_id
    }
    pub fn key(&self) -> &Vec<u8> {
        &self.key
    }
    pub fn value(&self) -> &Vec<u8> {
        &self.value
    }

    pub fn set_flags(&mut self, flags: u16) {
        self.flags = flags;
    }
    pub fn set_page_id(&mut self, pgid: u64) {
        self.page_id = pgid;
    }
    pub fn set_key(&mut self, key: Vec<u8>) {
        self.key = key;
    }
    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }
}

#[cfg(test)]
pub mod test {
    use crate::db::Db;
    use crate::db_utils;
    use crate::node::{InnerNode, Node};
    use crate::page::{Page, BRANCH_PAGE_FLAG, LEAF_PAGE_FLAG};

    #[test]
    fn test_write_and_read_for_branch_node() {
        env_logger::init();
        let file_name = "data.db".to_string();
        let mut db = Db::new(&file_name);
        let page_id = 5;
        let flags = BRANCH_PAGE_FLAG;
        let count = 20;
        let page = Page::new(page_id, flags, count as u16, (page_id * page_id) as u32);
        db.write_page(&page);
        let mut page = db_utils::read_page(page_id);
        let mut node = init_branch_node(page_id, flags, count as usize);
        node.write(&mut page);
        let mut ret_node = Node::new();
        ret_node.read(&mut page);
        assert_eq!(node, ret_node);
    }

    fn init_branch_node(page_id: u64, flags: u16, count: usize) -> Node {
        let mut node = Node::new();
        node.set_page_id(page_id);
        node.set_is_leaf(false);
        let mut inner_node_list = Vec::<InnerNode>::new();
        let count = 5;

        for index in 0..count {
            let key = format!("key_{}", index);
            let key = Vec::<u8>::from(&key[..]);
            let inner_node = InnerNode::new(flags, page_id, key, vec![]);
            inner_node_list.push(inner_node);
        }

        let key = format!("key_{}", 0);
        let key = Vec::<u8>::from(&key[..]);
        node.set_key(Some(key));
        node.set_inner_node_list(inner_node_list);

        node
    }

    fn init_leaf_node(page_id: u64, flags: u16, count: usize) -> Node {
        let mut node = Node::new();
        node.set_page_id(page_id);
        node.set_is_leaf(true);
        let mut inner_node_list = Vec::<InnerNode>::new();
        let count = 5;
        for index in 0..count {
            let key = format!("key_{}", index);
            let val = format!("val_{}", index);
            let key = Vec::<u8>::from(&key[..]);
            let val = Vec::<u8>::from(&val[..]);
            let inner_node = InnerNode::new(flags, page_id, key, val);
            inner_node_list.push(inner_node);
        }
        node.set_inner_node_list(inner_node_list);

        node
    }

    #[test]
    fn test_write_and_read_for_leaf_node() {
        // env_logger::init();
        let file_name = "data11.db".to_string();
        let mut db = Db::new(&file_name);
        let page_id = 5;
        let flags = LEAF_PAGE_FLAG;
        let count = 20;
        let page = Page::new(page_id, flags, count as u16, (page_id * page_id) as u32);
        db.write_page(&page);
        let mut page = db_utils::read_page(page_id);
        let mut node = init_leaf_node(page_id, flags, count as usize);
        node.write(&mut page);
        let mut ret_node = Node::new();
        ret_node.read(&mut page);
        assert_eq!(node, ret_node);
    }
}
