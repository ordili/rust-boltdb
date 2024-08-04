pub struct Meta {
    magic: u32, //魔数
    version: u32, //版本
    page_size: usize, //page页的大小，该值和操作系统默认的页大小保持一致
    flags: u32, //保留值，目前貌似还没用到
    // root     :bucket //所有小柜子bucket的根
    freelist: u64, //空闲列表页的id
    page_id: u64, //元数据页的id
    tx_id: u64, //最大的事务id
    checksum: u64, //用作校验的校验和
}

impl Meta{
    pub fn magic(&self) -> u32 {
        self.magic
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn page_size(&self) -> usize {
        self.page_size
    }
    pub fn flags(&self) -> u32 {
        self.flags
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
    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
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

