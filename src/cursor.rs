use crate::node::Node;
use crate::page::Page;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DBCursor {
    stack: Vec<ElemRef>,
}

impl DBCursor {
    pub fn new(page: Option<Rc<RefCell<Page>>>, node: Option<Rc<RefCell<Node>>>) -> Self {
        DBCursor {
            stack: vec![ElemRef::new(page, node)],
        }
    }

    pub fn seek(&mut self, key: &[u8]) {}
}

struct ElemRef {
    page: Option<Rc<RefCell<Page>>>,
    node: Option<Rc<RefCell<Node>>>,
}

impl ElemRef {
    pub fn new(page: Option<Rc<RefCell<Page>>>, node: Option<Rc<RefCell<Node>>>) -> Self {
        Self { page, node }
    }
}
