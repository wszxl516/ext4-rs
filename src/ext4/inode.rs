use crate::io::{CoreRead, CoreSeek};
use crate::{Group, Mode, SuperBlock};
use crate::ext4::{DIRECT_BLOCKS_COUNT, INDIRECT1_BLOCKS_COUNT, INDIRECT2_BLOCKS_COUNT, INDIRECT3_BLOCKS_COUNT};
use crate::ext4::stat::IFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Inode {
    mode: Mode,        /* File mode */
    uid: u16,         /* Low 16 bits of Owner Uid */
    size_lo: u32,     /* Size in bytes */
    atime: u32,       /* Access time */
    ctime: u32,       /* Inode Change time */
    mtime: u32,       /* Modification time */
    dtime: u32,       /* Deletion Time */
    gid: u16,         /* Low 16 bits of Group Id */
    links_count: u16, /* Links count */
    blocks_lo: u32,   /* Blocks count */
    flags: IFlags,       /* File flags */
    version: u32,
    block: [u32; 15], /* Pointers to blocks */
    generation: u32,  /* File version (for NFS) */
    file_acl_lo: u32, /* File ACL */
    size_high: u32,
    obso_faddr: u32,    /* Obsoleted fragment address */
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
    proj_id: u32,       /* Project ID */
}

impl Inode {
    pub const SIZE: usize = core::mem::size_of::<Self>();
    pub fn new<Disk: CoreRead + CoreSeek>(f: &mut Disk, inode_num: u64, sb: &SuperBlock) -> Option<Self> {
        let group = Group::get_group(f, sb, inode_num)?;
        let block_size = sb.block_size();
        let inode_size = sb.inode_size();
        let offset =
            block_size * group.inode_table() + (inode_num - group.first_inode_num()) * inode_size;
        f.seek_to(offset);
        Some(f.read_struct::<Self>().ok()?)
    }
    #[inline]
    pub fn mode(&self) -> Mode{
        self.mode
    }
    #[inline]
    pub fn flags(&self) -> IFlags{
        self.flags
    }
    #[inline]
    pub fn blocks_count(&self) -> u64{
        self.blocks_lo as u64 | ((self.blocks_high as u64) << 32)
    }
    #[inline]
    pub fn direct_blocks(&self) -> [u32; DIRECT_BLOCKS_COUNT]{
         self.block[0..12].try_into().unwrap()
    }
    #[inline]
    pub fn indirect_block(&self) -> [u32; 3]{
        [self.block[INDIRECT1_BLOCKS_COUNT], self.block[INDIRECT2_BLOCKS_COUNT], self.block[INDIRECT3_BLOCKS_COUNT]]
    }

}
