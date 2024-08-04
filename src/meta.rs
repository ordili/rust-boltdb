use crate::page::{Page, META_PAGE_FLAG};
use std::ptr;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Meta {
    magic: u32,       //魔数
    version: u32,     //版本
    page_size: usize, //page页的大小，该值和操作系统默认的页大小保持一致
    // root     :bucket //所有小柜子bucket的根
    freelist: u64, //空闲列表页的id
    page_id: u64,  //元数据页的id
    tx_id: u64,    //最大的事务id
    checksum: u64, //用作校验的校验和
}

impl Meta {
    pub fn magic(&self) -> u32 {
        self.magic
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn page_size(&self) -> usize {
        self.page_size
    }
    pub fn freelist(&self) -> u64 {
        self.freelist
    }
    pub fn page_id(&self) -> u64 {
        self.page_id
    }
    pub fn tx_id(&self) -> u64 {
        self.tx_id
    }
    pub fn checksum(&self) -> u64 {
        self.checksum
    }

    pub fn set_magic(&mut self, magic: u32) {
        self.magic = magic;
    }
    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }
    pub fn set_page_size(&mut self, page_size: usize) {
        self.page_size = page_size;
    }
    pub fn set_freelist(&mut self, freelist: u64) {
        self.freelist = freelist;
    }
    pub fn set_page_id(&mut self, page_id: u64) {
        self.page_id = page_id;
    }
    pub fn set_tx_id(&mut self, tx_id: u64) {
        self.tx_id = tx_id;
    }
    pub fn set_checksum(&mut self, checksum: u64) {
        self.checksum = checksum;
    }
}

impl Meta {
    pub fn new(page_id: u64, page_size: usize, tx_id: u64, checksum: u64) -> Self {
        Self {
            magic: 123456u32, //魔数
            version: 1u32,    //版本
            page_size,        //page页的大小，该值和操作系统默认的页大小保持一致
            // root     :bucket //所有小柜子bucket的根
            freelist: 3, //空闲列表页的id
            page_id,     //元数据页的id
            tx_id,       //最大的事务id
            checksum,    //用作校验的校验和
        }
    }

    pub fn new_empty() -> Self {
        Self {
            magic: 123456u32, //魔数
            version: 1u32,    //版本
            page_size: 1,     //page页的大小，该值和操作系统默认的页大小保持一致
            freelist: 3,      //空闲列表页的id
            page_id: 0,       //元数据页的id
            tx_id: 0,         //最大的事务id
            checksum: 0,      //用作校验的校验和
        }
    }

    pub fn as_page(&self) -> Page {
        Page::new(self.page_id, META_PAGE_FLAG, 1, 0)
    }

    //把 Meta 写入 Page 中
    pub fn write(&self, page: &mut Page) {
        let ptr = page.skip_page_header();
        unsafe {
            let meta_ptr = ptr as *mut Meta;
            ptr::write(meta_ptr, *self);
        }
    }

    // 从Page中读取Page
    pub fn from_page(page: &mut Page) -> Self {
        let ptr = page.skip_page_header();
        unsafe {
            let meta_ptr = ptr as *mut Meta;
            ptr::read(meta_ptr)
        }
    }

    // 计算 Meta 的checksum
    pub fn calculate_checksum(&self) -> u64 {
        // to do
        64u64
    }
}

#[cfg(test)]
pub mod test {
    use crate::constant::PAGE_SIZE;
    use crate::db::Db;
    use crate::meta::Meta;
    use crate::page::{Page, META_PAGE_FLAG};

    #[test]
    fn test_write() {
        let file_name = "tst.db";
        let page_id = 1;
        let db = Db::new(file_name);
        let meta = Meta::new(page_id, PAGE_SIZE, 128, 12);
        let mut page = Page::new(page_id, META_PAGE_FLAG, 0, 0);
        db.write_page(&mut page);
        let mut page = db.read_page(page_id);
        meta.write(&mut page);
    }

    #[test]
    fn test_read() {
        let file_name = "tst.db";
        let page_id = 1;
        let db = Db::new(file_name);
        let meta = Meta::new(page_id, PAGE_SIZE, 128, 12);
        let mut page = Page::new(page_id, META_PAGE_FLAG, 0, 0);
        db.write_page(&mut page);

        let mut page = db.read_page(page_id);
        let ret_meta = Meta::from_page(&mut page);

        assert_eq!(meta, ret_meta);
    }

    #[test]
    fn test_checksum() {
        let meta = Meta::new(1, 1024, 110, 0);
        let check_sum = meta.calculate_checksum();
        for _ in 0..10 {
            assert_eq!(check_sum, meta.calculate_checksum());
        }
    }
}
