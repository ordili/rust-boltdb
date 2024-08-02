use crate::node::InnerNode;
use crate::page::{BranchPageElement, LeafPageElement, Page, BRANCH_PAGE_FLAG, LEAF_PAGE_FLAG};

// 从Page中读出Page中的内容，保存到Node 中的 inner node list中；
pub fn read_inner_node_from_page(page: &mut Page) -> Vec<InnerNode> {
    let sz = page.count();
    let mut inner_node_list = Vec::<InnerNode>::with_capacity(sz as usize);
    for index in 0..(sz as usize) {
        let flags = if page.is_leaf_page() {
            LEAF_PAGE_FLAG
        } else {
            BRANCH_PAGE_FLAG
        };
        let (key, val) = read_key_and_val_from_page(page, index);
        let inner_node = InnerNode::new(flags, page.page_id(), key, val);
        inner_node_list.push(inner_node);
    }
    inner_node_list
}

// 从Page中读取第index个元素对应的Key, Val
fn read_key_and_val_from_page(page: &mut Page, index: usize) -> (Vec<u8>, Vec<u8>) {
    if page.is_leaf_page() {
        let leaf_page_element = page.read_leaf_page_element(index);
        let pos = leaf_page_element.pos();
        let val_pos = pos + leaf_page_element.ksize();
        (
            page.read_key(pos, leaf_page_element.ksize()),
            page.read_val(val_pos, leaf_page_element.vsize()),
        )
    } else {
        let branch_page_element = page.read_branch_page_element(index);
        let pos = branch_page_element.pos();
        (page.read_key(pos, branch_page_element.ksize()), vec![])
    }
}

// 把 Node 中的内容写入Page中
// Page 的格式是： PageHeader + BranchPageElement List / LeafPageElement List + Key1[Val1]..KeyN[ValN]
// 其中 BranchPageElement List / LeafPageElement List 保存了后面Key & Val 的位置信息；
pub fn write_inner_node_to_page(inner_node_list: &Vec<InnerNode>, page: &mut Page) {
    let is_leaf = page.is_leaf_page();
    let page_id = page.get_page_id();

    let mut pos = 0;
    let mut index = 0;
    for inode in inner_node_list {
        let key = inode.key();
        let val = inode.value();

        assert!(key.len() > 0, "write: zero-length inode key");

        let sz = key.len() + val.len();

        if is_leaf {
            let leaf_page_element = LeafPageElement::new(inode.flags(), pos, key.len(), val.len());
            page.write_leaf_page_element(&leaf_page_element, index);

            page.write_key(&key, pos);
            page.write_val(&val, pos + key.len());
        } else {
            let branch_element = BranchPageElement::new(pos, key.len(), page_id);
            page.write_branch_page_element(&branch_element, index);

            page.write_key(&key, pos);
        }

        pos += sz;
        index += 1;
    }
}
