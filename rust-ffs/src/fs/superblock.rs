use std::time::{SystemTime, UNIX_EPOCH};
use crate::fs::layout::*;

#[repr(C)]
pub struct Superblock {
    pub magic_number: u32,
    pub version: u32,
    pub block_size: u32,
    pub total_blocks: u32,
    pub fs_size: u64,

    pub inode_count: u32,
    pub free_inodes: u32,
    pub first_inode_block: u32,
    pub inode_blocks: u32,
    pub inode_bitmap_block: u32,

    pub free_blocks: u32,
    pub first_data_block: u32,
    pub bitmap_block: u32,

    pub root_inode: u32,

    pub created_time: i64,
    pub last_mount_time: i64,
    pub last_write_time: i64,

    pub mount_count: u32,
    pub state: u32,

    pub backup_superblock: [i32; 5],
    pub reserved: [i8; 128],
}

impl Superblock {
    pub fn new(block_size: u32, total_blocks: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Self {
            magic_number: FS_MAGIC,
            version: 1,
            block_size,
            total_blocks,
            fs_size: block_size as u64 * total_blocks as u64,

            inode_count: 320,
            free_inodes: 320,
            first_inode_block: FIRST_INODE_BLOCK,
            inode_blocks: 8,
            inode_bitmap_block: INODE_BITMAP_BLOCK,

            free_blocks: total_blocks - 11,
            first_data_block: 11,
            bitmap_block: BLOCK_BITMAP_BLOCK,

            root_inode: 1,

            created_time: now,
            last_mount_time: now,
            last_write_time: now,

            mount_count: 0,
            state: 0,

            backup_superblock: [0; 5],
            reserved: [0; 128],
        }
    }
}
