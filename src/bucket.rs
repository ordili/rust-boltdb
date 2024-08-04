use crate::node::Node;
use crate::tx::Tx;

pub struct Bucket<'a> {
    tx: &'a Tx<'a>,
    // 数据比较少的时候，数据存储在inline page 中
    inline_page: &'a Node,
    root_node: &'a Node,
}

impl<'a> Bucket<'a> {
    pub fn tx(&self) -> &'a Tx<'a> {
        self.tx
    }
    pub fn root_node(&self) -> &'a Node {
        self.root_node
    }
    pub fn set_tx(&mut self, tx: &'a Tx<'a>) {
        self.tx = tx;
    }
    pub fn set_root_node(&mut self, root_node: &'a Node) {
        self.root_node = root_node;
    }

    pub fn inline_page(&self) -> &'a Node {
        self.inline_page
    }

    pub fn set_inline_page(&mut self, inline_page: &'a Node) {
        self.inline_page = inline_page;
    }
    pub fn new(tx: &'a Tx<'a>, inline_page: &'a Node, root_node: &'a Node) -> Self {
        Self {
            tx,
            inline_page,
            root_node,
        }
    }

    pub fn put(key: &[u8], val: &[u8]) {}

    pub fn get(key: &[u8]) -> Option<Vec<u8>> {
        None
    }
}
