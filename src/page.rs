const BRANCH_PAGE_FLAG: u16 = 0x01;
const LEAF_PAGE_FLAG: u16 = 0x02;
const META_PAGE_FLAG: u16 = 0x04;
const FREELIST_PAGE_FLAG: u16 = 0x10;

const MIN_KEYS_PER_PAGE: u16 = 2;

const BUCKET_LEAF_FLAG: u16 = 0x01;

#[derive(Ord, PartialEq, Eq, PartialOrd, Debug)]
pub struct Pgid(u64);

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
}

// 代表叶子节点中的一个元素
pub struct LeafPageElement {
    flags: u32,
    pos: u32,
    ksize: u32,
    vsize: u32,
}

// 代表分支节点中的一个元素
pub struct BranchPageElement {
    pos: u32,
    ksize: u32,
    pgid: Pgid,
}

impl LeafPageElement {
    pub fn new(flags: u32, pos: u32, ksize: u32, vsize: u32) -> Self {
        Self {
            flags,
            pos,
            ksize,
            vsize,
        }
    }
}

impl BranchPageElement {
    pub fn new(pos: u32, ksize: u32, pgid: Pgid) -> Self {
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
