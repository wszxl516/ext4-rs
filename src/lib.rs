#![feature(slice_pattern)]
#![feature(stmt_expr_attributes)]
#![feature(slice_first_last_chunk)]
#![no_std]
///https://ext4.wiki.kernel.org/index.php/Ext4_Disk_Layout
extern crate alloc;
extern crate core;
mod ext4;
mod error;
mod fs;
pub mod io;

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
pub use io::{CoreSeek, CoreWrite, CoreRead};
#[allow(unused_imports)]
pub use error::Error;
