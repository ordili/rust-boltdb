// DB File size
pub const FILE_MAX_SIZE: u64 = 1024 * 1024 * 100;

// 每个页面，最多有多少元素
pub const MAX_PAGE_ELEMENT_COUNT: usize = 50;

// 页面大小
pub const PAGE_SIZE: usize = 1024 * 1024 * 10;

pub const MAX_PAGE_ID: u64 = FILE_MAX_SIZE / PAGE_SIZE as u64 - 1;
