use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Inode {
    pub inode_number: u32,
    pub file_type: u32,
    pub size: u32,

    pub direct_blocks: [u32; 12],
    pub indirect_block: u32,

    pub created_time: i64,
    pub modified_time: i64,
    pub accessed_time: i64,

    pub deleted_time: i64,
    pub is_deleted: u32,
    pub tamper_flag: u32,

    pub owner_id: u32,
    pub permissions: u32,
    pub link_count: u32,
}

impl Inode {
    pub fn new(inode_number: u32, file_type: u32, permissions: u32, owner_id: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Self {
            inode_number,
            file_type,
            size: 0,
            direct_blocks: [0; 12],
            indirect_block: 0,
            created_time: now,
            modified_time: now,
            accessed_time: now,
            deleted_time: 0,
            is_deleted: 0,
            tamper_flag: 0,
            owner_id,
            permissions,
            link_count: 1,
        }
    }

    pub fn size_on_disk() -> usize {
        mem::size_of::<Self>()
    }
}
