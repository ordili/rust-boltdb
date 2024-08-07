use crate::constant::{GLOBAL_BEGIN_PTR, MAX_PAGE_ID, PAGE_SIZE};
use crate::page::Page;
use std::ptr;

pub fn read_page(page_id: u64) -> Page {
    let ptr = unsafe { *GLOBAL_BEGIN_PTR.clone().unwrap() };
    unsafe {
        let ptr = ptr.add(page_id as usize * PAGE_SIZE);
        let page_ptr = Some(ptr);
        let ptr = ptr as *const Page;
        let mut page = ptr::read(ptr);
        page.set_page_ptr(page_ptr);
        page
    }
}

pub fn write_page(page: &Page) {
    let mut ptr = unsafe { *GLOBAL_BEGIN_PTR.clone().unwrap() };
    let page_id = page.get_page_id();
    if page_id > MAX_PAGE_ID {
        let err_msg = format!(
            "page id {} more then the max page is: {}",
            page_id, MAX_PAGE_ID
        );
        panic!("{}", err_msg);
    }
    unsafe {
        ptr = ptr.add(page_id as usize * PAGE_SIZE);
        let ptr = ptr as *mut Page;
        ptr::write(ptr, page.clone());
    }
}
