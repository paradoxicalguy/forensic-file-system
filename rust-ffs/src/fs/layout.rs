pub const FILETYPE_FILE: u32 = 1;
pub const FILETYPE_DIR: u32 = 2;
pub const FS_MAGIC: u32 = 0xF0F03410;

pub const INODE_BLOCKS: u32 = 8;
pub const FIRST_DATA_BLOCK: u32 = 11;
pub const RESERVED_BLOCKS: u32 = 11;  // 0-10 inclusive

// Disk layout
pub const SUPERBLOCK_BLOCK: u32 = 0;
pub const BLOCK_BITMAP_BLOCK: u32 = 1;
pub const INODE_BITMAP_BLOCK: u32 = 2;
pub const FIRST_INODE_BLOCK: u32 = 3;

