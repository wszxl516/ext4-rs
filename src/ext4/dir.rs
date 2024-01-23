use crate::{CoreRead, Inode, Mode};
use alloc::string::{String, ToString};
use core::slice::SlicePattern;

#[repr(C)]
#[derive(Default,Debug,Copy, Clone)]
pub struct DirEntry {
    pub(crate) inode: u32,   /* Inode number */
    rec_len: u16, /* Directory entry length */
    name_len: u8, /* Name length */
    file_type: u8,
}


impl DirEntry {
    pub fn from_bytes(buffer: &[u8]) -> Option<(DirEntry, String)> {
        let mut buf = buffer;
        match buf.read_struct::<DirEntry>() {
            Ok(entry) => {
                if entry.inode != 0 {
                    let name = String::from_utf8_lossy(buf[..entry.name_len as usize].as_slice()).to_string();
                    Some((entry, name))
                } else {
                    None
                }
            }
            Err(_) => return None,
        }
    }
    pub fn len(&self) -> usize{
        self.rec_len as usize
    }

}

#[derive(Default,Debug, Clone)]
pub struct  Entry{
    entry: DirEntry,
    name: String,
    inode: Inode
}
impl Entry {
    pub fn new(entry: DirEntry, name: String, inode: Inode) -> Self{
        Self{entry, name, inode}
    }
    pub fn len(&self) -> usize{
        self.entry.len()
    }
    pub  fn name(&self) ->&str{
        &self.name
    }
    pub  fn mode(&self) -> Mode{
        self.inode.mode()
    }
    pub fn inode(&self) -> Inode{
        self.inode
    }
}