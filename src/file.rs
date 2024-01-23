use alloc::string::String;
use crate::ext4::dir::Entry;
use crate::{CoreWrite, DirEntry, FileSystem, Inode, Mode};
use alloc::vec::Vec;
use core::slice::SlicePattern;
pub struct Ext4File<'a> {
    name: String,
    fs: &'a mut FileSystem,
    inode: Inode,
    blocks: Vec<u64>,
    pos: u64,
}

impl<'a> Ext4File<'a> {
    pub fn new(inode: Inode, name: String, fs: &'a mut FileSystem) -> Ext4File<'a> {
        Self {
            name,
            fs,
            inode,
            blocks: inode.blocks().unwrap().data_blocks(),
            pos: 0,
        }
    }
    pub fn read_block(&mut self, index: usize) -> Result<Vec<u8>, ()> {
        self.fs.read_block(self.blocks[index])
    }
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, ()> {
        let block_size = self.fs.sb().block_size();
        assert!(buffer.len() <= block_size as usize);
        let block_start_index = (self.pos / block_size) as usize;
        let block_offset = self.pos % block_size;
        let buffer_len = buffer.len() as u64;
        let n_blocks = buffer_len / block_size;
        let buffer_offset = buffer_len % block_size;
        let n_blocks = match buffer_offset.eq(&0) {
            true => n_blocks,
            false => n_blocks + 1,
        } as usize;
        let mut buffer = buffer;
        let mut write_bytes = 0;
        let block_end_index = block_start_index + n_blocks;
        for blk_index in block_start_index..block_end_index {
            let data = self.read_block(blk_index)?;
            let mut buf = data.as_slice();
            if blk_index == block_start_index {
                buf = buf[block_offset as usize..].as_slice()
            }
            if blk_index == block_end_index - 1 && buffer_offset != 0 {
                buf = buf[..buffer_offset as usize].as_slice()
            }

            let mut n = buffer.write_bytes(buf).unwrap() as u64;
            if self.pos + n >= self.size() {
                n = n - (self.pos + n - self.size())
            }
            self.pos += n;
            write_bytes += n;
        }
        Ok(write_bytes as usize)
    }
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.inode.size()
    }
    #[inline]
    pub fn name(&self) -> &str{
        self.name.as_str()
    }
    #[inline]
    pub fn mode(&self) -> Mode {
        self.inode.mode()
    }
    #[inline]
    pub fn size(&self) -> u64 {
        self.inode.size()
    }
    pub fn read_dir(&mut self) -> Result<Vec<Entry>, ()> {
        let mut entrys = Vec::new();
        for index in 0..self.blocks.len() {
            let data = self.read_block(index)?;
            let buffer = data.as_slice();
            let mut offset = 0usize;
            while offset < buffer.len() {
                let entry = DirEntry::from_bytes(buffer[offset..].as_slice());
                match entry {
                    None => break,
                    Some((entry, name)) => {
                        offset += entry.len();
                        let i_num = entry.inode;
                        let inode = self.fs.read_inode(i_num as u64);
                        entrys.push(Entry::new(entry, name, inode));
                    }
                }
            }
        }
        Ok(entrys)
    }
}
