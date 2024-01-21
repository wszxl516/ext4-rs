use core::fmt::{Display, Formatter};
use bitflags::bitflags;
bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Mode: u16 {
        /// FIFO
        const FIFO = 0x1000;
        /// Character device
        const CHAR_DEVICE = 0x2000;
        /// Directory
        const DIRECTORY = 0x4000;
        /// Block device
        const BLOCK_DEVICE = 0x6000;
        /// Regular file
        const FILE = 0x8000;
        /// Symbolic link
        const SYMLINK = 0xA000;
        /// Unix socket
        const SOCKET = 0xC000;
        /// Other—execute permission
        const O_EXEC = 0x001;
        /// Other—write permission
        const O_WRITE = 0x002;
        /// Other—read permission
        const O_READ = 0x004;
        /// Group—execute permission
        const G_EXEC = 0x008;
        /// Group—write permission
        const G_WRITE = 0x010;
        /// Group—read permission
        const G_READ = 0x020;
        /// User—execute permission
        const U_EXEC = 0x040;
        /// User—write permission
        const U_WRITE = 0x080;
        /// User—read permission
        const U_READ = 0x100;
        /// Sticky Bit
        const STICKY = 0x200;
        /// Set group ID
        const SET_GID = 0x400;
        /// Set user ID
        const SET_UID = 0x800;
    }
}
impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        const TYPE_NAME: [&'static str; 7] = ["f", "c", "d", "b", "-", "l", "s"];
        const PERM_NAME: [&'static str; 5] = ["-", "r", "w", "", "x"];
        write!(
            f,
            "{} ",
            TYPE_NAME[(self.file_type().bits() / 0x2000) as usize]
        )?;
        let perm = self.perm();
        for i in (0..3).rev() {
            let p = (perm >> (i * 3)) as usize;
            write!(f, "{}{}{}", PERM_NAME[p & 0b001], PERM_NAME[p & 0b010], PERM_NAME[p & 0b100])?
        }
        Ok(())
    }
}

impl Mode {
    pub fn is_dir(&self) -> bool {
        self.contains(Self::DIRECTORY)
    }

    pub fn is_file(&self) -> bool {
        self.contains(Self::FILE)
    }

    pub fn is_symlink(&self) -> bool {
        self.contains(Self::SYMLINK)
    }

    /// Returns true if this mode represents a fifo, also known as a named pipe.
    pub fn is_fifo(&self) -> bool {
        self.contains(Self::FIFO)
    }

    /// Returns true if this mode represents a character device.
    pub fn is_char_device(&self) -> bool {
        self.contains(Self::CHAR_DEVICE)
    }

    /// Returns true if this mode represents a block device.
    pub fn is_block_device(&self) -> bool {
        self.contains(Self::BLOCK_DEVICE)
    }

    /// Returns true if this mode represents a Unix-domain socket.
    pub fn is_socket(&self) -> bool {
        self.contains(Self::SOCKET)
    }
    pub fn file_type(&self) -> Mode {
        Mode::from_bits_truncate(self.bits() & 0xf000)
    }
    pub fn perm(&self) -> u32 {
        self.bits() as u32 & 0x1ff
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct IFlags: u32{
        /* Inode flags*/
        const Secrm		    =	0x00000001; /* Secure deletion */
        const Unrm		    =	0x00000002; /* Undelete */
        const Compr		    =	0x00000004; /* Compress file */
        const Sync		    =   0x00000008 ;/* Synchronous updates */
        const Immutable	    =   0x00000010; /* Immutable file */
        const Append	    =	0x00000020; /* writes to file may only append */
        const NoDump	    =	0x00000040; /* do not dump file */
        const NoAtime	    =	0x00000080; /* do not update atime */
        /* reserved for compression usage... */
        const Dirty		    =   0x00000100;
        const ComprBlk	    =   0x00000200; /* One or more compressed clusters */
        const NoCompr	    =	0x00000400; /* Don't compress */
        /* nb: was previously EXT2_ECOMPR */
        const Encrypt	    =	0x00000800; /* encrypted file */
        /* end compression flags --- maybe not all used */
        const Index		    =   0x00001000; /* hash-indexed directory */
        const IMagic	    =	0x00002000; /* AFS directory */
        const JournalData   =	0x00004000; /* file data should be journaled */
        const NoTail	    =	0x00008000; /* file tail should not be merged */
        const DirSync	    =	0x00010000; /* dirsync behaviour (directories only) */
        const TopDir	    =	0x00020000; /* Top of directory hierarchies*/
        const HugeFile     =   0x00040000; /* Set to each huge file */
        const Extents	    =	0x00080000; /* Inode uses extents */
        const Verity	    =	0x00100000; /* Verity protected inode */
        const EAInode	    =   0x00200000; /* Inode used for large EA */
        /* 0x00400000 was formerly EOFBLOCKS */
        const Dax		    =   0x02000000; /* Inode is DAX */
        const InlineData   =	0x10000000; /* Inode has inline data. */
        const ProjInHerit   =	0x20000000; /* Create with parents projid */
        const CaseFold	    =   0x40000000 ;/* Casefolded directory */
        const Reserved	    =   0x80000000; /* reserved for ext4 lib */
    }
}