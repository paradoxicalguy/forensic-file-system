use std::path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{File, OpenOptions, read};
use std::io::{Seek, SeekFrom, Write};
use std::mem;
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

fn read_block(path: &str, block_size: u64, block_number: u64, buffer: &mut [u8]) -> std::io::Result<()>{
      if buffer.len() != block_size as usize {
        return Err(std::io::Error::new( std::io::ErrorKind::InvalidInput,
            "buffer size must equal block_size",
        ));
    }

   let mut file = OpenOptions::new()
   .read(true)
   .open(path)?;

   let offset = block_number * block_size;
   file.seek(SeekFrom::Start(offset))?;
   Ok(())

}

fn write_block_bitmap(path: &str, block_size: u64, block_number: u64, bitmap: &[u8]) -> std::io::Result<()> {
     if bitmap.len() != block_size as usize {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "bitmap size must be equal to block_size"));
     }
    let mut file: File = std::fs::OpenOptions::new()   
    .read(true)
    .write(true)
    .open(path)?;

    let offset = block_number * block_size;
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&bitmap)?;
    Ok(())
}

fn bitmap_set (bitmap: &mut[u8], block_number: u32) {
    let byte_index = (block_number / 8) as usize;
    let bit_index = (block_number % 8) as u8;
    bitmap[byte_index] |= 1 << bit_index;
}

fn bitmap_test(bitmap: &[u8], block_number: u32)->bool {
    let byte = (block_number / 8) as usize;
    let bit = (block_number % 8) as u8;
    (bitmap[byte] & (1<<bit)) != 0
}

fn bitmap_clear(bitmap: &mut[u8], block_number: u32) {
    let byte = (block_number / 8) as usize;
    let bit = (block_number % 8) as u8;
    bitmap[byte] &= !(1<<bit);
}

fn bitmap_find_tree(bitmap: &[u8], max_bits: u32) ->Option<u32> {
    for i in 0..max_bits {
        if !bitmap_test(bitmap, i) {
            return Some(i);
        }
    }
    None
}

fn init_block_bitmap(path: &str, block_size: u64) ->std::io::Result<()> {
    let mut block_bitmap = vec![0u8; block_size as usize];

    for i in 0..=10 {
        bitmap_set(&mut block_bitmap, i);
    }
    write_block_bitmap(path, block_size, 1, &block_bitmap)?;
    Ok(())
}

fn init_inode_bitmap (path: &str, block_size: u64) ->std::io::Result<()> {
    let mut inode_bitmap = vec![0u8; block_size as usize];
    bitmap_set(&mut inode_bitmap, 0);
    bitmap_set(&mut inode_bitmap, 1);
    write_block_bitmap(path, block_size, 2, &inode_bitmap)?;
    Ok(())
}

fn init_inode_table(path: &str, sb: &Superblock) ->std::io::Result<()> {
    let total_size = sb.inode_blocks as usize * sb.block_size as usize;
    let mut table: Vec<u8> = vec![0u8; total_size];

    let root = Inode::new(1, FILETYPE_DIR, 0o755, 0);
    let inode_size = mem::size_of::<Inode>();
    let root_bytes = unsafe {
        std::slice::from_raw_parts(&root as *const Inode as *const u8,inode_size )
    };
    table[..inode_size].copy_from_slice(root_bytes);
    let mut file: File = OpenOptions::new()
    .read(true)
    .write(true)
    .open(path)?;   

    for i in 0..sb.inode_blocks {
      let disk_block = sb.first_inode_block + i;
      let disk_offset = disk_block as u64 * sb.block_size as u64;   

      let table_offset = i as usize * sb.block_size as usize;

      file.seek(SeekFrom::Start(disk_offset))?;
      file.write_all(&table[table_offset..table_offset + sb.block_size as usize],)?;
    }
    Ok(())
}

