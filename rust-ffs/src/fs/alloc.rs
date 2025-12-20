use crate::disk::io::{read_block, write_block};
use crate::fs::bitmap::*;
use crate::fs::superblock::Superblock;

pub fn allocate_block( path: &str, sb: &mut Superblock,) -> std::io::Result<Option<u32>> {
    let mut bitmap = vec![0u8; sb.block_size as usize];
    read_block(path, &mut bitmap, sb.block_size, sb.bitmap_block)?;

    let free = (sb.first_data_block..sb.total_blocks)
        .find(|&b| !bitmap_test(&bitmap, b));

    let block = match free {
        Some(b) => b,
        None => return Ok(None),
    };

    bitmap_set(&mut bitmap, block);
    write_block(path, &bitmap, sb.block_size, sb.bitmap_block)?;

    sb.free_blocks -= 1;
    Ok(Some(block))
}

pub fn allocate_inode( path: &str, sb: &mut Superblock) -> std::io::Result<Option<u32>> {
    let mut bitmap = vec![0u8; sb.block_size as usize];
    read_block(path, &mut bitmap, sb.block_size, sb.inode_bitmap_block)?;

    let free = (0..sb.inode_count)
        .find(|&ino| !bitmap_test(&bitmap, ino));

    let inode_no = match free {
        Some(ino) => ino,
        None => return Ok(None),
    };

    bitmap_set(&mut bitmap, inode_no);
    write_block(path, &bitmap, sb.block_size, sb.inode_bitmap_block)?;

    sb.free_inodes -= 1;
    Ok(Some(inode_no))
}
