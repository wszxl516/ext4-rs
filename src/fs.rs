use alloc::format;
use alloc::string::String;
use super::{CoreRead, CoreSeek};
use super::{Inode, ROOT_INODE_NUM, SuperBlock};

pub struct FileSystem<'a, Disk: CoreRead + CoreSeek> {
    sb: SuperBlock,
    disk: &'a mut Disk
}

impl<'a, Disk: CoreRead + CoreSeek> FileSystem<'a, Disk> {
    pub fn new(f: &'a mut Disk) -> Option<Self<>>{
        let sb = SuperBlock::new(f)?;
        Some(Self{disk: f, sb })
    }
    pub fn info(&self) -> String{
        format!("{}", self.sb)
    }
    pub fn root_inode(&mut self) -> Inode{
        Inode::new(self.disk, ROOT_INODE_NUM, &self.sb).unwrap()
    }
}