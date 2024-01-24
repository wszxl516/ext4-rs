use super::{Inode, SuperBlock, ROOT_INODE_NUM, Ext4File};
use crate::ext4::superblock::FeatureInCompat;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use alloc::boxed::Box;
use crate::ext4::Disk;

pub struct FileSystem {
    sb: SuperBlock,
    pub(crate) disk: Box<dyn Disk>,
}

impl FileSystem {
    pub fn new(mut f: Box<(dyn Disk)>) -> Option<Self> {
        let sb = SuperBlock::new(&mut f)?;
        Some(Self { disk: f, sb })
    }
    pub const fn sb(&self) -> &SuperBlock {
        &self.sb
    }
    pub fn feature_file_type(&self) -> bool {
        self.sb
            .feature_in_compat()
            .contains(FeatureInCompat::FileType)
    }
    pub fn info(&self) -> String {
        format!("{}", self.sb)
    }
    pub fn root_inode(&mut self) -> Inode {
        Inode::new(&mut self.disk, ROOT_INODE_NUM, &self.sb).unwrap()
    }
    pub fn read_inode(&mut self, inode_num: u64) -> Inode {
        Inode::new(&mut self.disk, inode_num, &self.sb).unwrap()
    }
    pub fn read_block(&mut self, block_num: u64) -> Result<Vec<u8>, ()> {
        let block_size = self.sb.block_size();
        let mut buffer = vec![0u8; block_size as usize];
        self.disk.seek_to(block_size * block_num);
        match self.disk.read(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(_) => Err(()),
        }
    }
    pub fn open(&mut self, path: &str) -> Option<Ext4File> {
        if path == "/" {
            return Some(Ext4File::new(self.root_inode(), path.to_string(), self))
        }
        let path_parts = path.split("/").filter(|p| !p.is_empty()).collect::<Vec<&str>>();
        let mut current = self.root_inode();
        let mut current_entry= Default::default();
        for part in &path_parts {
            'found: for entry in Ext4File::new(current, part.to_string(), self).read_dir().unwrap() {
                if entry.name() == *part{
                    current = entry.inode();
                    current_entry = entry;
                    break 'found
                }
            }
        }
        match &current_entry.name() == path_parts.last().unwrap(){
            true => Some(Ext4File::new(current, current_entry.name().to_string(),self)),
            false => None
        }
    }
}
