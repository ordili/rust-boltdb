use crate::page::{BranchPageElement, LeafPageElement, Page, Pgid};
use std::vec;

// 代表磁盘中的一个Page；数据存储在内存中
pub struct Node {
    is_leaf: bool,
    unbalanced: bool,
    spilled: bool,
    key: Vec<u8>,
    pgid: Pgid,
    // parent: &'a Node<'a>,
    // children   :&nodes,
    inodes: INodes,
}

pub struct INode {
    flags: u16,
    pgid: Pgid,
    key: Vec<u8>,
    value: Vec<u8>,
}

pub struct INodes(Vec<INode>);

impl INodes {
    pub fn new() -> Self {
        INodes(vec![])
    }

    // 把INodes 写入Page中
    // page 的内存布局是 ： PageHeader + PageElement List + Page Value
    // Branch Page Value 的布局是： key1, key2, .... keyN
    // Leaf Page Value 的布局是： key1, val1, key2, val2, ...., keyN, valN.
    pub fn write_inodes_to_page(&mut self, page: &mut Page) {
        let is_leaf = page.is_leaf_page();
        let page_id = page.get_page_id();

        let mut pos = 0;
        let mut index = 0;
        for inode in &self.0 {
            assert!(inode.key.len() > 0, "write: zero-length inode key");

            let sz = inode.key.len() + inode.value.len();

            if is_leaf {
                let leaf_page_element =
                    LeafPageElement::new(inode.flags, pos, inode.key.len(), inode.value.len());
                page.write_leaf_page_element(&leaf_page_element, index);

                page.write_key(&inode.key, pos);
                page.write_val(&inode.value, pos + inode.key.len());
            } else {
                let branch_element = BranchPageElement::new(pos, inode.key.len(), page_id);
                page.write_branch_page_element(&branch_element, index);

                page.write_key(&inode.key, pos);
            }

            pos += sz;
            index += 1;
        }
    }

    // 把Page中的内容读入到INodes中
    pub fn read_inodes_from_page(&mut self, page: &mut Page) {
        todo!()
    }
}
