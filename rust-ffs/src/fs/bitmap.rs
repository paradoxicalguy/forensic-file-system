pub fn bitmap_set(bitmap: &mut [u8], bit: u32) {
    let byte = (bit / 8) as usize;
    let offset = (bit % 8) as u8;
    bitmap[byte] |= 1 << offset;
}

pub fn bitmap_clear(bitmap: &mut [u8], bit: u32) {
    let byte = (bit / 8) as usize;
    let offset = (bit % 8) as u8;
    bitmap[byte] &= !(1 << offset);
}

pub fn bitmap_test(bitmap: &[u8], bit: u32) -> bool {
    let byte = (bit / 8) as usize;
    let offset = (bit % 8) as u8;
    (bitmap[byte] & (1 << offset)) != 0
}

pub fn bitmap_find_free(bitmap: &[u8], max: u32) -> Option<u32> {
    (0..max).find(|&i| !bitmap_test(bitmap, i))
}
