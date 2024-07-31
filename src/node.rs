#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::page::{BranchPageElement, LeafPageElement, Page};
use std::vec;

// 代表磁盘中的一个Page；数据存储在内存中
pub struct Node {
    is_leaf: bool,
    unbalanced: bool,
    spilled: bool,
    key: Vec<u8>,
    pgid: u64,
    // parent: &'a Node<'a>,
    // children   :&nodes,
    inodes: Vec<INode>,
}

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
    pub fn key(&self) -> &Vec<u8> {
        &self.key
    }
    pub fn pgid(&self) -> u64 {
        self.pgid
    }
    pub fn inodes(&self) -> &Vec<INode> {
        &self.inodes
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
    pub fn set_key(&mut self, key: Vec<u8>) {
        self.key = key;
    }
    pub fn set_pgid(&mut self, pgid: u64) {
        self.pgid = pgid;
    }
    pub fn set_inodes(&mut self, inodes: Vec<INode>) {
        self.inodes = inodes;
    }
}

pub struct INode {
    flags: u16,
    pgid: u64,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl INode {
    pub fn flags(&self) -> u16 {
        self.flags
    }
    pub fn pgid(&self) -> u64 {
        self.pgid
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
    pub fn set_pgid(&mut self, pgid: u64) {
        self.pgid = pgid;
    }
    pub fn set_key(&mut self, key: Vec<u8>) {
        self.key = key;
    }
    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }
}
