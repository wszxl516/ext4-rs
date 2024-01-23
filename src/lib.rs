#![feature(slice_pattern)]
#![feature(stmt_expr_attributes)]
#![feature(slice_first_last_chunk)]
#![feature(exclusive_range_pattern)]
#![no_std]
///https://ext4.wiki.kernel.org/index.php/Ext4_Disk_Layout
extern crate alloc;
extern crate core;
mod ext4;
mod error;
mod fs;
pub mod io;
mod file;

#[allow(unused_imports)]
pub use ext4::{superblock::SuperBlock,
               group::Group,
               inode::Inode,
               stat::{Mode, IFlags},
               ROOT_INODE_NUM
};
#[allow(unused_imports)]
pub use fs::FileSystem;
#[allow(unused_imports)]
pub use io::{CoreWrite, CoreRead};
#[allow(unused_imports)]
pub use error::Error;
#[allow(unused_imports)]
pub use ext4::extent_tree::ExtentTree;
#[allow(unused_imports)]
pub use file::Ext4File;
#[allow(unused_imports)]
pub use ext4::dir::DirEntry;
#[allow(unused_imports)]
pub use ext4::Disk;

