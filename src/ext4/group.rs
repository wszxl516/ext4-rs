use crate::io::{CoreRead, CoreSeek};
use crate::SuperBlock;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GroupDesc {
    block_bitmap_lo: u32,      /* Blocks bitmap block */
    inode_bitmap_lo: u32,      /* Inodes bitmap block */
    inode_table_lo: u32,       /* Inodes table block */
    free_blocks_count_lo: u16, /* Free blocks count */
    free_inodes_count_lo: u16, /* Free inodes count */
    used_dirs_count_lo: u16,   /* Directories count */
    flags: u16,                /* EXT4_BG_flags (INODE_UNINIT, etc) */
    exclude_bitmap_lo: u32,    /* Exclude bitmap for snapshots */
    block_bitmap_csum_lo: u16, /* crc32c(s_uuid+grp_num+bbitmap) LE */
    inode_bitmap_csum_lo: u16, /* crc32c(s_uuid+grp_num+ibitmap) LE */
    itable_unused_lo: u16,     /* Unused inodes count */
    checksum: u16,             /* crc16(sb_uuid+group+desc) */
    block_bitmap_hi: u32,      /* Blocks bitmap block MSB */
    inode_bitmap_hi: u32,      /* Inodes bitmap block MSB */
    inode_table_hi: u32,       /* Inodes table block MSB */
    free_blocks_count_hi: u16, /* Free blocks count MSB */
    free_inodes_count_hi: u16, /* Free inodes count MSB */
    used_dirs_count_hi: u16,   /* Directories count MSB */
    itable_unused_hi: u16,     /* Unused inodes count MSB */
    exclude_bitmap_hi: u32,    /* Exclude bitmap block MSB */
    block_bitmap_csum_hi: u16, /* crc32c(s_uuid+grp_num+bbitmap) BE */
    inode_bitmap_csum_hi: u16, /* crc32c(s_uuid+grp_num+ibitmap) BE */
    _reserved: u32,
}
#[derive(Debug)]
pub struct Group<'a> {
    pub desc: GroupDesc,
    pub sb: &'a SuperBlock,
    num: u64
}
impl<'a> Group<'a> {
    pub const DESC_SIZE: usize = core::mem::size_of::<GroupDesc>();
    pub fn new<Disk: CoreRead + CoreSeek>(
        f: &mut Disk,
        sb: &'a SuperBlock,
        group_num: u64,
    ) -> Option<Group<'a>> {
        let block_size = sb.block_size();
        let offset = group_num * Self::DESC_SIZE as u64
            + if block_size == SuperBlock::SIZE as u64 {
            SuperBlock::SIZE as u64 * 2
            } else {
                block_size
            };
        f.seek_to(offset);
        let desc = f.read_struct::<GroupDesc>().ok()?;
        Some(Self { desc, sb, num: group_num })
    }
    pub fn first_block_num(&self) -> u64{
        self.num * self.sb.blocks_per_group() + self.sb.first_data_block()
    }
    #[inline(always)]
    pub fn first_inode_num(&self) -> u64{
        self.num * self.sb.inodes_per_group() + 1
    }
    #[inline(always)]
    pub fn inode_table(&self) -> u64{
        self.desc.inode_table_lo as u64| (self.desc.inode_table_hi as u64) << 32
    }

    pub fn get_group<Disk: CoreRead + CoreSeek>(f: &mut Disk, sb: &'a SuperBlock, inode_num: u64) -> Option<Group<'a>>{
        assert!(inode_num < 10);
        let group_num = (inode_num - 1) / sb.inodes_per_group();
        Self::new(f,sb ,group_num)
    }

}
