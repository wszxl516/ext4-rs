pub mod superblock;
pub mod group;
pub mod inode;
pub mod stat;

pub const DIRECT_BLOCKS_COUNT: usize = 12;
pub const INDIRECT1_BLOCKS_COUNT: usize = 13;
pub const INDIRECT2_BLOCKS_COUNT: usize = 14;
pub const INDIRECT3_BLOCKS_COUNT: usize = 15;
pub const ROOT_INODE_NUM: u64 = 2;



