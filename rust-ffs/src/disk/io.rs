use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub fn create_disk_image(path: &str, size: u64) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.seek(SeekFrom::Start(size - 1))?;
    file.write_all(&[0])?;
    Ok(())
}

pub fn read_block( path: &str, buffer: &mut [u8], block_size: u32, block_number: u32,) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let offset = block_number as u64 * block_size as u64;
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(buffer)?;
    Ok(())
}

pub fn write_block(path: &str, buffer: &[u8], block_size: u32, block_number: u32, ) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let offset = block_number as u64 * block_size as u64;
    file.seek(SeekFrom::Start(offset))?;
    file.write_all(buffer)?;
    Ok(())
}
