#![allow(dead_code)]
use std::cell::UnsafeCell;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use ext4::{Disk, Error};
use ext4::io::{CoreRead};
#[derive(Debug)]
pub struct  DiskFile(UnsafeCell<File>);
impl DiskFile {
    pub fn new(name: &str) -> Self {
        let file = File::options()
            .write(true)
            .read(true)
            .open(name).unwrap();
        Self(UnsafeCell::new(file))
    }
}
impl CoreRead for DiskFile{
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.read(buf)
    }
}

impl Disk for DiskFile{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        match self.0.get_mut().read(buf){
            Ok(size) => Ok(size),
            Err(_) => Err(Error::IOError("".to_string()))
        }
    }

    fn read_block(&mut self, block_size: u64, block_num: u64, buf: &mut [u8]) -> Result<usize, Error> {
        self.seek_to(block_num * block_size);
        self.read(buf)
    }
    fn read_at(&mut self, offset: u64,  buf: &mut [u8]) -> Result<usize, Error> {
        self.seek_to(offset);
        self.read(buf)
    }

    fn seek_to(&mut self, offset: u64) {
        self.0.get_mut().seek(SeekFrom::Start(offset)).unwrap();

    }
}
