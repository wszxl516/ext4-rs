use alloc::vec::Vec;
use crate::CoreRead;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExtentHeader {
    eh_magic: u16,      /* probably will support different formats */
    eh_entries: u16,    /* number of valid entries */
    eh_max: u16,        /* capacity of store in entries */
    eh_depth: u16,      /* has tree real underlying blocks? */
    eh_generation: u32, /* generation of the tree */
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
struct ExtentTail {
    et_checksum: u32, /* crc32c(uuid+inum+extent_block) */
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Extent {
    ee_block: u32,    /* first logical block extent covers */
    ee_len: u16,      /* number of blocks covered by extent */
    ee_start_hi: u16, /* high 16 bits of physical block */
    ee_start_lo: u32, /* low 32 bits of physical block */
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct ExtentIdx {
    ei_block: u32,   /* index covers logical blocks from 'block' */
    ei_leaf_lo: u32, /* pointer to the physical block of the next ** level. leaf or next index could be there */
    ei_leaf_hi: u16, /* high 16 bits of physical block */
    ei_unused: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct ExtentTree {
    header: ExtentHeader,
    extent: [Extent; 4]
}
impl ExtentTree {
    pub const MAGIC: u16 = 0xf30a;
    pub fn new(block: &[u8]) -> Option<Self>{
        let mut bytes = block;
        let header = bytes.read_struct::<ExtentHeader>().unwrap();
        let extent = bytes.read_struct::<[Extent;4]>().unwrap();
        //TODO: eh_depth > 0
        let extent_tree = Self{header, extent};
        match extent_tree.is_valid() {
            true => Some(extent_tree),
            false => None
        }
    }
    #[inline]
    pub const fn is_valid(&self) -> bool{
        self.header.eh_magic == Self::MAGIC
    }
    pub fn data_blocks(&self) -> Vec<u64>{
        let mut blocks = Vec::new();
        for extent in self.extent {
            let start = (extent.ee_start_hi as u64) << 32 | extent.ee_start_lo as u64;
            let len = extent.ee_len as u64;
            blocks.extend((start..start+len).into_iter().collect::<Vec<u64>>())
        }
        blocks

    }
}

