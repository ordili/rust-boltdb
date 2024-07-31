#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::common::{read_inner_node_from_page, write_inner_node_to_page};
use crate::page::{BranchPageElement, LeafPageElement, Page, BRANCH_PAGE_FLAG, LEAF_PAGE_FLAG};
use std::any::Any;
use std::ptr::NonNull;
use std::vec;

// 代表磁盘中的一个Page；数据存储在内存中
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
    pub fn read(&mut self, page: &Page) {
        self.set_is_leaf(page.is_leaf_page());
        self.set_page_id(page.get_page_id());
        let inner_node_list = read_inner_node_from_page(page);
        let key = if inner_node_list.len() > 0 {
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
}

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
