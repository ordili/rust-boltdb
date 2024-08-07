#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]

use crate::db::Db;
use crate::db_utils;
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

    // 返回key, val, page
    pub fn seek(&mut self, key: &Vec<u8>) -> (Option<Vec<u8>>, Option<Rc<RefCell<Page>>>) {
        let elem_ref = self.stack.pop().expect("stack is empty");

        let page = elem_ref.page;
        let mut node = elem_ref.node;

        if page.is_none() && node.is_none() {
            return (None, None);
        }

        if node.is_none() {
            let page_id = page.clone().unwrap().borrow().get_page_id();
            let mut page = db_utils::read_page(page_id);
            let mut new_node = Node::new();
            new_node.read(&mut page);
            node = Some(Rc::new(RefCell::new(new_node)));
        }

        let mut node = node.unwrap();

        let val = node.borrow().search_key(key);

        return (val, page);
    }
}

struct ElemRef {
    page: Option<Rc<RefCell<Page>>>,
    node: Option<Rc<RefCell<Node>>>,
}

impl ElemRef {
    pub fn new(page: Option<Rc<RefCell<Page>>>, node: Option<Rc<RefCell<Node>>>) -> Self {
        Self { page, node }
    }

    pub fn page(&self) -> &Option<Rc<RefCell<Page>>> {
        &self.page
    }
    pub fn node(&self) -> &Option<Rc<RefCell<Node>>> {
        &self.node
    }

    pub fn set_page(&mut self, page: Option<Rc<RefCell<Page>>>) {
        self.page = page;
    }
    pub fn set_node(&mut self, node: Option<Rc<RefCell<Node>>>) {
        self.node = node;
    }
}
