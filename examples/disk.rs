#![allow(dead_code)]
use std::cell::UnsafeCell;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use ext4::Error;
use ext4::io::{CoreRead, CoreSeek};

pub struct  DiskFile(UnsafeCell<File>);
impl DiskFile{
    pub fn new(name: &str) -> Self{
        let file = File::options()
            .write(true)
            .read(true)
            .open(name).unwrap();
        Self(UnsafeCell::new(file))
    }
    pub fn read_block(&mut self, block_size: u64, block_num: u64, buf: &mut [u8]) -> Result<usize, Error> {
        self.seek_to(block_num * block_size);
        self.read_bytes(buf)
    }
    pub fn read_at(&mut self, offset: u64,  buf: &mut [u8]) -> Result<usize, Error> {
        self.seek_to(offset);
        self.read_bytes(buf)
    }
}
impl CoreSeek for DiskFile{
    fn seek_to(&mut self, offset: u64) {
        self.0.get_mut().seek(SeekFrom::Start(offset)).unwrap();
    }
}
impl CoreRead for DiskFile{
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        match self.0.get_mut().read(buf){
            Ok(size) => Ok(size),
            Err(_) => Err(Error::IOError("read_bytes".to_string()))
        }
    }

    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let mut buffer = [0u8; 32];
        let mut all_read = 0usize;
        loop {
            match self.read_bytes(&mut buffer){
                Ok(read_bytes) => {
                    all_read += read_bytes;
                    buf.extend_from_slice(buffer.as_slice());
                    if read_bytes == 0 {
                        break
                    }
                }
                Err(e) => return Err(Error::IOError(format!("{:?}", e)))
            }
        }
        Ok(all_read)
    }
}
