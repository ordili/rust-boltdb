use memmap2::MmapMut;
use rust_boltdb::constant::FILE_MAX_SIZE;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::ptr;

fn main() {
    test_mem_map();
    println!("Hello, world!");
}

fn test_mem_map() {
    let file_name = "abc.txt".to_string();
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(file_name)
        .expect("Open file failed");

    file.set_len(FILE_MAX_SIZE).expect("Set file len error");
    file.seek(SeekFrom::Start(0)).expect("Set start error");

    let mmap = unsafe { MmapMut::map_mut(&file).expect("Mmap file failed.") };

    println!("mmap is : {:#?}", mmap);

    let range = mmap.as_ptr_range();
    let end = range.end;
    let start = range.start;
    println!("address is: {:#?}", range);
    println!("address is: {:#?}", mmap.as_ptr());
}
