pub const FILETYPE_FILE: u32 = 1;
pub const FILETYPE_DIR: u32 = 2;
pub const FS_MAGIC: u32 = 0xF0F03410;

// Disk layout
pub const SUPERBLOCK_BLOCK: u32 = 0;
pub const BLOCK_BITMAP_BLOCK: u32 = 1;
pub const INODE_BITMAP_BLOCK: u32 = 2;
pub const FIRST_INODE_BLOCK: u32 = 3;
