#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use std::ptr;

pub const BRANCH_PAGE_FLAG: u16 = 0x01;
pub const LEAF_PAGE_FLAG: u16 = 0x02;
pub const META_PAGE_FLAG: u16 = 0x04;
pub const FREELIST_PAGE_FLAG: u16 = 0x10;

const MIN_KEYS_PER_PAGE: u16 = 2;

const BUCKET_LEAF_FLAG: u16 = 0x01;

pub const PAGE_HEADER_SIZE: usize = std::mem::size_of::<Page>();

pub const BRANCH_PAGE_ELEMENT_SIZE: usize = std::mem::size_of::<BranchPageElement>();
pub const LEAF_PAGE_ELEMENT_SIZE: usize = std::mem::size_of::<LeafPageElement>();

// 每个页面，最多有多少元素
pub const MAX_PAGE_ELEMENT_COUNT: usize = 100;

#[derive(Ord, PartialEq, Eq, PartialOrd, Debug, Clone, Copy)]
pub struct Pgid(u64);

impl Pgid {
    pub fn new(pgid: u64) -> Self {
        Pgid(pgid)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    id: Pgid,
    flags: u16,
    count: u16,
    overflow: u32,
}

impl Page {
    pub fn new(id: Pgid, flags: u16, count: u16, overflow: u32) -> Self {
        Page {
            id,
            flags,
            count,
            overflow,
        }
    }

    pub fn page_type(&self) -> Option<String> {
        match self.flags {
            BRANCH_PAGE_FLAG => Some("branch".to_string()),
            LEAF_PAGE_FLAG => Some("leaf".to_string()),
            META_PAGE_FLAG => Some("meta".to_string()),
            FREELIST_PAGE_FLAG => Some("freelist".to_string()),
            _other => None,
        }
    }

    pub fn is_branch_page(&self) -> bool {
        self.flags == BRANCH_PAGE_FLAG
    }

    pub fn is_leaf_page(&self) -> bool {
        self.flags == LEAF_PAGE_FLAG
    }

    pub fn is_meta_page(&self) -> bool {
        self.flags == META_PAGE_FLAG
    }

    pub fn is_freelist_page(&self) -> bool {
        self.flags == FREELIST_PAGE_FLAG
    }

    // 返回Page对应的指针
    pub fn as_ptr(&self) -> *const u8 {
        std::ptr::from_ref(self) as *const u8
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        std::ptr::from_mut(self) as *mut u8
    }
    pub fn skip_page_header(&self) -> *const u8 {
        let mut ptr = self.as_ptr();
        unsafe { ptr.add(PAGE_HEADER_SIZE) }
    }

    // 把指针跳转到val开始的位置
    pub fn skip_to_val_start_loc(&self) -> *const u8 {
        let mut ptr = self.skip_page_header();

        let skip_size = if self.is_branch_page() {
            BRANCH_PAGE_ELEMENT_SIZE * MAX_PAGE_ELEMENT_COUNT
        } else {
            LEAF_PAGE_ELEMENT_SIZE * MAX_PAGE_ELEMENT_COUNT
        };
        // 跳过Page Element list
        unsafe { ptr.add(skip_size) }
    }

    // 获取PageId
    pub fn get_page_id(&self) -> Pgid {
        self.id
    }

    // 写入BranchPageElement
    pub fn write_branch_page_element(
        &mut self,
        branch_page_element: &BranchPageElement,
        index: usize,
    ) {
        let mut base_ptr = self.skip_page_header();
        unsafe {
            if index > 0 {
                base_ptr = base_ptr.add(index * BRANCH_PAGE_ELEMENT_SIZE);
            }
            let ptr = base_ptr as *mut BranchPageElement;
            ptr::write(ptr, branch_page_element.clone());
        }
    }

    // 写入LeafPageElement
    pub fn write_leaf_page_element(&mut self, leaf_page_element: &LeafPageElement, index: usize) {
        let mut base_ptr = self.skip_page_header();
        unsafe {
            if index > 0 {
                base_ptr = base_ptr.add(index * LEAF_PAGE_ELEMENT_SIZE);
            }
            let ptr = base_ptr as *mut LeafPageElement;
            ptr::write(ptr, leaf_page_element.clone());
        }
    }

    // 在pos处写入Key; pos 是 page 中存储key * val 的起始地址
    pub fn write_key(&mut self, key: &[u8], pos: usize) {
        let base_ptr = self.skip_to_val_start_loc();
        let key: Vec<u8> = Vec::from(key);
        unsafe {
            let ptr = base_ptr.add(pos);
            let ptr = ptr as *mut Vec<u8>;
            ptr::write(ptr, key);
        }
    }

    // 在pos处写入val
    pub fn write_val(&mut self, key: &[u8], pos: usize) {
        self.write_key(key, pos);
    }
}

// 代表叶子节点中的一个元素
#[derive(Clone)]
pub struct LeafPageElement {
    flags: u16,
    pos: usize,
    ksize: usize,
    vsize: usize,
}

// 代表分支节点中的一个元素
#[derive(Clone)]
pub struct BranchPageElement {
    pos: usize,
    ksize: usize,
    pgid: Pgid,
}

impl LeafPageElement {
    pub fn new(flags: u16, pos: usize, ksize: usize, vsize: usize) -> Self {
        Self {
            flags,
            pos,
            ksize,
            vsize,
        }
    }
}

impl BranchPageElement {
    pub fn new(pos: usize, ksize: usize, pgid: Pgid) -> Self {
        Self { pos, ksize, pgid }
    }
}

pub struct PageInfo {
    id: i32,
    page_type: String,
    count: i32,
    over_flow_count: i32,
}

pub struct Pgids {
    pub pgids: Vec<Pgid>,
}

impl Pgids {
    pub fn new() -> Self {
        Self {
            pgids: Vec::with_capacity(64),
        }
    }

    pub fn merge(&mut self, mut other: Pgids) {
        self.pgids.append(&mut other.pgids);
        self.pgids.sort();
    }
}

#[cfg(test)]
mod tests {
    use crate::page::{
        Page, Pgid, Pgids, BRANCH_PAGE_FLAG, FREELIST_PAGE_FLAG, LEAF_PAGE_FLAG, META_PAGE_FLAG,
    };

    #[test]
    pub fn test_pgids_merge() {
        let mut pg1 = Pgids::new();
        let mut pg2 = Pgids::new();

        pg1.pgids.push(Pgid(5));
        pg1.pgids.push(Pgid(3));
        pg1.pgids.push(Pgid(10));

        pg2.pgids.push(Pgid(3));
        pg2.pgids.push(Pgid(7));
        pg2.pgids.push(Pgid(6));
        pg1.merge(pg2);

        assert_eq!(
            pg1.pgids,
            vec![Pgid(3), Pgid(3), Pgid(5), Pgid(6), Pgid(7), Pgid(10)]
        );
    }

    #[test]
    fn test_page_new_type() {
        let page = Page::new(Pgid(0), BRANCH_PAGE_FLAG, 12, 0);
        assert_eq!(page.page_type(), Some("branch".to_string()));

        let page = Page::new(Pgid(0), LEAF_PAGE_FLAG, 12, 0);
        assert_eq!(page.page_type(), Some("leaf".to_string()));

        let page = Page::new(Pgid(0), META_PAGE_FLAG, 12, 0);
        assert_eq!(page.page_type(), Some("meta".to_string()));

        let page = Page::new(Pgid(0), FREELIST_PAGE_FLAG, 12, 0);
        assert_eq!(page.page_type(), Some("freelist".to_string()));
    }
}
