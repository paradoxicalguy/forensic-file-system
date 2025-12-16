use std::path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
const FILETYPE_FILE: u32 = 1;
const FILETYPE_DIR: u32 = 2;
const FS_MAGIC: u32 = 0xF0F03410;


// Superblock
struct Superblock {
    magic_number: u32,
    version: u32,

    block_size: u32,
    total_blocks: u32,
    fs_size: u64,

    inode_count: u32,
    free_inodes: u32,
    first_inode_block: u32,
    inode_blocks: u32,
    inode_bitmap_block: u32,

    free_blocks: u32,
    first_data_block: u32,
    bitmap_block: u32,

    root_inode: u32,

    created_time: i64,
    last_mount_time: i64,
    last_write_time: i64,

    mount_count: u32,
    state: u32,

    backup_superblock: [i32; 5],
    reserved: [i8; 128],
}

impl Superblock {
    fn new(block_size: u32, total_blocks: u32) -> Superblock {
        let created_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Superblock {
            magic_number: FS_MAGIC,
            version: 1,

            block_size,
            total_blocks,
            fs_size: block_size as u64 * total_blocks as u64,

            inode_count: 320,
            free_inodes: 320,
            first_inode_block: 3,
            inode_blocks: 8,
            inode_bitmap_block: 2,

            free_blocks: total_blocks - 11,
            first_data_block: 11,
            bitmap_block: 1,

            root_inode: 1,

            created_time,
            last_mount_time: created_time,
            last_write_time: created_time,

            mount_count: 0,
            state: 0,

            backup_superblock: [0; 5],
            reserved: [0; 128],
        }
    }
}


// Inode
struct Inode {
    inode_number: u32,
    file_type: u32,
    size: u32,

    direct_blocks: [u32; 12],
    indirect_blocks: u32,

    created_time: i64,
    modified_time: i64,
    accessed_time: i64,

    deleted_time: i64,
    is_deleted: u32,
    tamper_flag: u32,

    owner_id: u32,
    permissions: u32,
    link_count: u32,
}

impl Inode {
    fn new(inode_number: u32, file_type: u32, permissions: u32, owner_id: u32) -> Inode {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Inode {
            inode_number,
            file_type,
            size: 0,

            direct_blocks: [0; 12],
            indirect_blocks: 0,

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
}

fn main() {
    let sb = Superblock::new(4096, 5000);
    let inode = Inode::new(1, FILETYPE_DIR, 0o755, 0);
}

fn create_disk_image(path: &str, size: u64) -> std::io::Result<()>{
    if size == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput, "disk size must be > 0",
        ));
    }
    let mut file = File::create(path)?;

    file.seek(SeekFrom::Start(size -1))?;
    file.write_all(&[0])?;

    Ok(())
}   

fn write_superblock(path: &str, sb: &Superblock, block_size: u64, block_number: u64) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
    .read(true)
    .write(true)
    .open(path)?;

    file.seek(SeekFrom::Start(block_number * block_size))?;

    Ok(())
}