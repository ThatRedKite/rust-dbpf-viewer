pub fn slicer_32(array: &[u8], offset: usize) -> u32 {
    let new_start: usize = offset * 4;
    let sliced: &[u8; 4] = &array[new_start..new_start + 4]
        .try_into()
        .expect("Not enough data in the Vector");
    u32::from_le_bytes(*sliced)
}

pub fn slicer_raw(array: &[u8], offset: usize) -> [u8; 4] {
    let new_start: usize = offset * 4;
    let sliced: &[u8; 4] = &array[new_start..new_start + 4]
        .try_into()
        .expect("Not enough data in the Vector");
    *sliced
}

pub fn slicer_16(vector: &[u8], offset: usize, sub_offset: usize) -> u32 {
    let new_start: usize = (offset * 4) + sub_offset;
    let sliced: &[u8; 2] = &vector[new_start..new_start + 2]
        .try_into()
        .expect("Not enough data in the Vector");
    u16::from_le_bytes(*sliced) as u32
}

