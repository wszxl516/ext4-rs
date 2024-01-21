#![allow(dead_code)]
use alloc::string::{String, ToString};
use alloc::{format};
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

use bitflags::{bitflags};
use crate::io::{CoreRead, CoreSeek};

bitflags! {
    #[derive(Debug, Default, Copy, Clone)]
    pub struct FeatureCompat:u32{
        const DirPrealloc = 0x0001;
        const IMagicInodes = 0x0002;
        const HasJournal = 0x0004;
        const ExtAttr = 0x0008;
        const ResizeInode = 0x0010;
        const DirIndex = 0x0020;
        const SparseSuper2 = 0x0200;
        const FastCommit = 0x0400;
        const StableInodes = 0x0800;
        const OrphanFile = 0x1000;
    }
    #[derive(Debug, Default, Copy, Clone)]
    pub struct FeatureInCompat: u32{
        const Compression=	0x0001;
        const FileType=		0x0002;
        const Recove=		0x0004; /* Needs recovery */
        const JournalDev=	0x0008; /* Journal device */
        const MetaBg=		0x0010;
        const Extents=		0x0040; /* extents support */
        const Is64bit=		0x0080;
        const MMP =         0x0100;
        const Flexbg=		0x0200;
        const EAInode=		0x0400; /* EA in inode */
        const DirData=		0x1000; /* data in dirent */
        const CsumSeed=	    0x2000;
        const LargeDir=     0x4000; /* >2GB or 3-lvl htree */
        const InlineData=   0x8000; /* data in inode */
        const Encrypt   =	0x10000;
        const CaseFold  =	0x20000;
    }
    #[derive(Debug, Default, Copy, Clone)]
    pub struct FeatureRoCompat: u32{
        const SparseSuper=	0x0001;
        const LargeFile=	0x0002;
        const BtreeDir=     0x0004;
        const HugeFile=     0x0008;
        const GdtCsum=		0x0010;
        const DirNlink=     0x0020;
        const ExtraIsize=	0x0040;
        const Quota=		0x0100;
        const BigAlloc=		0x0200;
        const MetadataCsum= 0x0400;
        const Readonly=		0x1000;
        const Project=		0x2000;
        const Verity=		0x8000;
        const OrphanPresent=0x10000;
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum FsState {
    Cleanly = 0x1,
    Errors = 0x2,
    Recovered = 0x4,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum OnErrors {
    Continue = 0x1,
    Remount = 0x2,
    Panic = 0x3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum OS {
    Linux = 0,
    Hurd = 1,
    Masix = 2,
    FreeBSD = 3,
    Lites = 4,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum HashAlgorithm {
    Legacy = 0x0,
    HalfMD4 = 0x1,
    Tea = 0x2,
    LegacyUnsigned = 0x3,
    HalfMD4Unsigned = 0x4,
    TeaUnsigned = 0x5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum EncryptAlgos {
    None = 0,
    Aes256Xts = 1,
    Aes256Gcm = 2,
    Aes256Cbc = 3,
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct MountOpt:u32{
        const Debug = 0x0001;
        const BSDGroups = 0x0002;
        const XattrUser= 0x0004;
        const ACL=0x0008;
        const Uid16=0x0010;
        const JmodeData=0x0020;
        const JmodeOrdered= 0x0040;
        const JmodeWback=0x0060;
        const NoBarrier=0x0100;
        const BlockValidity= 0x0200;
        const Discard= 0x0400;
        const NodelAlloc= 0x0800;
    }
    #[derive(Debug, Copy, Clone)]
    pub struct SBFlags: u32{
        const SignedDirectoryHash =0x0001;
        const UnsignedDirectoryHash =0x0002;
        const ToTestDevelopmentCode =0x0004;
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SuperBlock {
    inodes_count: u32,                  /* Inodes count */
    blocks_count_lo: u32,               /* Blocks count */
    r_blocks_count_lo: u32,             /* Reserved blocks count */
    free_blocks_count_lo: u32,          /* Free blocks count */
    free_inodes_count: u32,             /* Free inodes count */
    first_data_block: u32,              /* First Data Block */
    log_block_size: u32,                /* Block size */
    log_cluster_size: u32,              /* Allocation cluster size */
    blocks_per_group: u32,              /* # Blocks per group */
    clusters_per_group: u32,            /* # Clusters per group */
    inodes_per_group: u32,              /* # Inodes per group */
    mtime: u32,                         /* Mount time */
    wtime: u32,                         /* Write time */
    mnt_count: u16,                     /* Mount count */
    max_mnt_count: u16,                 /* Maximal mount count */
    magic: u16,                         /* Magic signature */
    state: FsState,                      /* File system state */
    errors: OnErrors,                         /* Behaviour when detecting errors */
    minor_rev_level: u16,               /* minor revision level */
    last_check: u32,                    /* time of last check */
    check_interval: u32,                /* max. time between checks */
    creator_os: OS,                     /* OS */
    rev_level: u32,                     /* Revision level */
    def_res_uid: u16,                   /* Default uid for reserved blocks */
    def_res_gid: u16,                   /* Default gid for reserved blocks */
    first_ino: u32,                     /* First non-reserved inode */
    inode_size: u16,                    /* size of inode structure */
    block_group_nr: u16,                /* block group # of this superblock */
    feature_compat: FeatureCompat,      /* compatible feature set */
    feature_in_compat: FeatureInCompat, /* incompatible feature set */
    feature_ro_compat: FeatureRoCompat, /* readonly-compatible feature set */
    uuid: [u8; 16],                     /* 128-bit uuid for volume */
    volume_name: [u8; 16],              /* volume name */
    last_mounted: [u8; 64],             /* directory where last mounted */
    algorithm_usage_bitmap: u32,        /* For compression */
    pre_alloc_blocks: u8,               /* Nr of blocks to try to preallocate*/
    pre_alloc_dir_blocks: u8,           /* Nr to preallocate for dirs */
    reserved_gdt_blocks: u16,           /* Per group desc for online growth */
    journal_uuid: [u8; 16],             /* uuid of journal superblock */
    journal_inum: u32,                  /* inode number of journal file */
    journal_dev: u32,                   /* device number of journal file */
    last_orphan: u32,                   /* start of list of inodes to delete */
    hash_seed: [u32; 4],                /* HTREE hash seed */
    def_hash_version: HashAlgorithm,               /* Default hash version to use */
    jnl_backup_type: u8,
    desc_size: u16,                 /* size of group descriptor */
    default_mount_opts: MountOpt,
    first_meta_bg: u32,           /* First metablock block group */
    mkfs_time: u32,               /* When the filesystem was created */
    jnl_blocks: [u32; 17],        /* Backup of the journal inode */
    blocks_count_hi: u32,         /* Blocks count */
    r_blocks_count_hi: u32,       /* Reserved blocks count */
    free_blocks_count_hi: u32,    /* Free blocks count */
    min_extra_isize: u16,         /* All inodes have at least # bytes */
    want_extra_isize: u16,        /* New inodes should reserve # bytes */
    flags: SBFlags,                   /* Miscellaneous flags */
    raid_stride: u16,             /* RAID stride */
    mmp_update_interval: u16,     /* # seconds to wait in MMP checking */
    mmp_block: u64,               /* Block for multi-mount protection */
    raid_stripe_width: u32,       /* blocks on all data disks (N*stride)*/
    log_groups_per_flex: u8,      /* FLEX_BG group size */
    checksum_type: u8,            /* metadata checksum algorithm used */
    encryption_level: u8,         /* versioning level for encryption */
    reserved_pad: u8,             /* Padding to next 32bits */
    k_bytes_written: u64,         /* nr of lifetime kilobytes written */
    snapshot_inum: u32,           /* Inode number of active snapshot */
    snapshot_id: u32,             /* sequential ID of active snapshot */
    snapshot_r_blocks_count: u64, /* reserved blocks for active snapshot's future use */
    snapshot_list: u32,           /* inode number of the head of the on-disk snapshot list */
    error_count: u32,             /* number of fs errors */
    first_error_time: u32,        /* first time an error happened */
    first_error_ino: u32,         /* inode involved in first error */
    first_error_block: u64,       /* block involved of first error */
    first_error_func: [u8; 32],   /* function where the error happened */
    first_error_line: u32,        /* line number where error happened */
    last_error_time: u32,         /* most recent time of an error */
    last_error_ino: u32,          /* inode involved in last error */
    last_error_line: u32,         /* line number where error happened */
    last_error_block: u64,        /* block involved of last error */
    last_error_func: [u8; 32],    /* function where the error happened */
    mount_opts: [u8; 64],
    usr_quota_inum: u32,         /* inode for tracking user quota */
    grp_quota_inum: u32,         /* inode for tracking group quota */
    overhead_clusters: u32,      /* overhead blocks/clusters in fs */
    backup_bgs: [u32; 2],        /* groups with sparse_super2 SBs */
    encrypt_algos: EncryptAlgos, /* Encryption algorithms in use  */
    encrypt_pw_salt: [u8; 16],   /* Salt used for string2key algorithm */
    lpf_ino: u32,                /* Location of the lost+found inode */
    prj_quota_inum: u32,         /* inode for tracking project quota */
    checksum_seed: u32,          /* crc32c(uuid) if csum_seed set */
    wtime_hi: u8,
    mtime_hi: u8,
    mkfs_time_hi: u8,
    last_check_hi: u8,
    first_error_time_hi: u8,
    last_error_time_hi: u8,
    first_error_errcode: u8,
    last_error_errcode: u8,
    encoding: u16,         /* Filename charset encoding */
    encoding_flags: u16,   /* Filename charset encoding flags */
    orphan_file_inum: u32, /* Inode for tracking orphan inodes */
    reserved: [u32; 94],   /* Padding to the end of the block */
    checksum: u32,         /* crc32c(superblock) */
}

impl SuperBlock {
    pub const SIZE: usize = core::mem::size_of::<Self>();
    pub const OFFSET: usize = Self::SIZE;
    pub const MAGIC: u16 = 0xef53;

    pub fn new<Disk: CoreRead + CoreSeek>(f: &mut Disk) -> Option<Self>{
        f.seek_to(Self::OFFSET as u64);
        let sb = f.read_struct::<Self>().unwrap();
        match sb.is_valid() {
            true => Some(sb),
            false => None
        }
    }
    pub fn from_buffer(buffer: &[u8]) -> Option<Self>{
        let mut buf = buffer;
        let sb = buf.read_struct::<Self>().unwrap();
        match sb.is_valid() {
            true => Some(sb),
            false => None
        }
    }
    #[inline]
    pub fn is_valid(&self)-> bool{
        self.magic == Self::MAGIC
    }
    #[inline]
    pub fn feature_compat(&self) -> FeatureCompat{
        self.feature_compat
    }

    #[inline]
    pub fn feature_in_compat(&self) -> FeatureInCompat{
        self.feature_in_compat
    }
    #[inline]
    pub fn feature_ro_compat(&self) -> FeatureRoCompat{
        self.feature_ro_compat
    }
    #[inline]
    pub fn fs_state(&self) -> FsState{
        self.state
    }
    #[inline]
    pub fn on_errors(&self) -> OnErrors{
        self.errors
    }
    #[inline]
    pub fn os(&self) -> OS {
        self.creator_os
    }
    #[inline]
    pub fn encrypt_algo(&self) -> EncryptAlgos {
        self.encrypt_algos
    }
    #[inline]
    pub fn block_count(&self) -> u64{
        self.blocks_count_lo as u64| ((self.blocks_count_hi as u64) << 32)
    }
    #[inline]
    pub fn free_block_count(&self) -> u64{
        self.free_blocks_count_lo as u64| ((self.free_blocks_count_hi as u64) << 32)
    }
    #[inline]
    pub fn inodes_count(&self) -> u64{
        self.inodes_count as u64
    }
    #[inline]
    pub fn free_inodes_count(&self) -> u64{
        self.free_inodes_count as u64
    }
    #[inline]
    pub fn inodes_per_group(&self) -> u64{
        self.inodes_per_group as u64
    }
    #[inline]
    pub fn blocks_per_group(&self) -> u64{
        self.blocks_per_group as u64
    }
    #[inline]
    pub fn first_data_block(&self) -> u64{
        self.first_data_block as u64
    }
    #[inline]
    pub fn block_size(&self) -> u64{
        (1024 << self.log_block_size) as u64
    }
    #[inline]
    pub fn inode_size(&self) -> u64{
        self.inode_size as u64
    }
    pub fn get_groups_count(&self) -> usize {
        let block_count = self.block_count();
        let blocks_per_group = self.blocks_per_group();
        let count = (block_count / blocks_per_group) as usize;
        match block_count % blocks_per_group == 0 {
            true => count,
            false => count + 1
        }
    }
    #[inline]
    pub fn uuid(&self) -> String{
        format!("{:x}{:x}{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}{:x}{:x}{:x}{:x}",
                self.uuid[0],self.uuid[1],self.uuid[2],self.uuid[3],
                self.uuid[4], self.uuid[5], self.uuid[6], self.uuid[7],
                self.uuid[8], self.uuid[9], self.uuid[10], self.uuid[11],
                self.uuid[12], self.uuid[13], self.uuid[14], self.uuid[15],
        )
    }
    #[inline]
    pub fn name(&self) -> String{
        String::from_utf8_lossy(&self.volume_name).to_string()

    }
    #[inline]
    pub fn has_sb_backup(&self) -> bool{
        self.feature_compat().contains(FeatureCompat::SparseSuper2)
    }
    #[inline]
    pub fn backups(&self) -> [u32; 2]{
        self.backup_bgs
    }
    pub fn find_backup<Disk: CoreRead + CoreSeek>(&self, f: &mut Disk) -> Vec<u64>{
        let mut backup = Vec::new();
        for num in 0..self.get_groups_count(){
            if (num % 2).eq(&1) {
                let blk_num = self.blocks_per_group() * num as u64 + 1;
                f.seek_to(blk_num * self.block_size());
                match f.read_struct::<Self>(){
                    Ok(sb) => {
                        if sb.is_valid() {
                            backup.push(blk_num)
                        }
                    },
                    Err(_) => {}
                }
            }
        }
        backup
    }
}

impl Display for SuperBlock{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?} rev {}.{} UUID={} volume name: {} block: {}/{}, inode: {}/{} {} {} {} {}",
               self.os(), self.rev_level,
               self.minor_rev_level, self.uuid(),
            self.name(),
            self.free_block_count(), self.block_count(),
            self.free_inodes_count(), self.inodes_count(),
            match self.feature_in_compat.contains(FeatureInCompat::Is64bit){
                true => "(64bit)",
                false => ""
            },
               match self.feature_in_compat.contains(FeatureInCompat::Extents){
                   true => "(extents)",
                   false => ""
               },
               match self.feature_ro_compat.contains(FeatureRoCompat::LargeFile){
                   true => "(large files)",
                   false => ""
               },
               match self.feature_ro_compat.contains(FeatureRoCompat::HugeFile){
                   true => "(huge files)",
                   false => ""
               },

        )
    }
}