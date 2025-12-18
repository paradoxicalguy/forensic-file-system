mod fs;
mod disk;

use fs::superblock::Superblock;
use fs::inode::Inode;
use fs::layout::*;
use disk::io::*;

fn main() -> std::io::Result<()> {
    let path = "disk.img";

    create_disk_image(path, 4096 * 5000)?;

    let sb = Superblock::new(4096, 5000);
    write_block(
        path,
        unsafe {
            std::slice::from_raw_parts(
                &sb as *const Superblock as *const u8,
                std::mem::size_of::<Superblock>(),
            )
        },
        sb.block_size,
        SUPERBLOCK_BLOCK,
    )?;

    println!("filesystem initialized");

    Ok(())
}
