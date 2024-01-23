use alloc::boxed::Box;
use crate::ext4::stat::IFlags;

use crate::io::{CoreRead};
use crate::{Disk, ExtentTree, Group, Mode, SuperBlock};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Inode {
    mode: Mode,       /* File mode */
    uid: u16,         /* Low 16 bits of Owner Uid */
    size_lo: u32,     /* Size in bytes */
    atime: u32,       /* Access time */
    ctime: u32,       /* Inode Change time */
    mtime: u32,       /* Modification time */
    dtime: u32,       /* Deletion Time */
    gid: u16,         /* Low 16 bits of Group Id */
    links_count: u16, /* Links count */
    blocks_lo: u32,   /* Blocks count */
    flags: IFlags,    /* File flags */
    version: u32,
    block: [u8; 60],    /* Pointers to blocks */
    generation: u32,  /* File version (for NFS) */
    file_acl_lo: u32, /* File ACL */
    size_high: u32,
    obso_faddr: u32,  /* Obsoleted fragment address */
    blocks_high: u16, /* were l_i_reserved1 */
    file_acl_high: u16,
    uid_high: u16,    /* these 2 fields */
    gid_high: u16,    /* were reserved2[0] */
    checksum_lo: u16, /* crc32c(uuid+inum+inode) LE */
    reserved: u16,
    extra_isize: u16,
    checksum_hi: u16,  /* crc32c(uuid+inum+inode) BE */
    ctime_extra: u32,  /* extra Change time      (nsec << 2 | epoch) */
    mtime_extra: u32,  /* extra Modification time(nsec << 2 | epoch) */
    atime_extra: u32,  /* extra Access time      (nsec << 2 | epoch) */
    crtime: u32,       /* File Creation time */
    crtime_extra: u32, /* extra FileCreationtime (nsec << 2 | epoch) */
    version_hi: u32,   /* high 32 bits for 64-bit version */
    proj_id: u32,      /* Project ID */
}
impl Default for Inode{
    fn default() -> Self {
        Inode{
            mode: Default::default(),
            uid: 0,
            size_lo: 0,
            atime: 0,
            ctime: 0,
            mtime: 0,
            dtime: 0,
            gid: 0,
            links_count: 0,
            blocks_lo: 0,
            flags: Default::default(),
            version: 0,
            block: [Default::default(); 60],
            generation: 0,
            file_acl_lo: 0,
            size_high: 0,
            obso_faddr: 0,
            blocks_high: 0,
            file_acl_high: 0,
            uid_high: 0,
            gid_high: 0,
            checksum_lo: 0,
            reserved: 0,
            extra_isize: 0,
            checksum_hi: 0,
            ctime_extra: 0,
            mtime_extra: 0,
            atime_extra: 0,
            crtime: 0,
            crtime_extra: 0,
            version_hi: 0,
            proj_id: 0,
        }
    }
}

impl Inode {
    pub const SIZE: usize = core::mem::size_of::<Self>();
    pub fn new(f: &mut Box<dyn Disk>, inode_num: u64, sb: &SuperBlock) -> Option<Self> {
        let group = Group::get_group(f, sb, inode_num)?;
        let block_size = sb.block_size();
        let inode_size = sb.inode_size();
        let offset =
            block_size * group.inode_table() + (inode_num - group.first_inode_num()) * inode_size;
        f.seek_to(offset);
        let inode = f.read_struct::<Self>().unwrap();
        Some(inode)
    }
    #[inline]
    pub fn mode(&self) -> Mode {
        self.mode
    }
    #[inline]
    pub fn flags(&self) -> IFlags {
        self.flags
    }
    #[inline]
    pub fn blocks_count(&self) -> u64 {
        self.blocks_lo as u64 | ((self.blocks_high as u64) << 32)
    }
    #[inline]
    pub fn blocks(&self) -> Option<ExtentTree> {
        ExtentTree::new(&self.block)
    }
    #[inline]
    pub fn size(&self) -> u64{
        (self.size_lo as u64) | ((self.size_high as u64)<< 32)
    }
}


