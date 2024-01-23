use crate::{CoreRead, Error};

pub mod superblock;
pub mod group;
pub mod inode;
pub mod stat;
pub mod dir;
pub mod extent_tree;
pub const ROOT_INODE_NUM: u64 = 2;

#[macro_export]
macro_rules! int_get {
    ($slice: ident,$value_type: ty) => {{
        let b: (&[u8; core::mem::size_of::<$value_type>()], &[u8]) =
            $slice.split_first_chunk().unwrap();
        #[allow(unused_assignments)]
        $slice = b.1;
        <$value_type>::from_le_bytes(*b.0)
    }};
}

#[macro_export]
macro_rules! align_up {
    ($len:expr, $size:expr) => {
        (($len as u64) + ($size as u64) - 1) & !(($size as u64) - 1)
    };
}

pub trait Disk{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

    fn read_block(&mut self, block_size: u64, block_num: u64, buf: &mut [u8]) -> Result<usize, Error>;
    fn read_at(&mut self, offset: u64,  buf: &mut [u8]) -> Result<usize, Error>;
    fn seek_to(&mut self, offset: u64);
}

impl CoreRead for dyn Disk{
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.read(buf)
    }
}
