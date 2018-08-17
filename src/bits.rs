pub fn extract_u16(data: &[u8], offset: usize) -> u16 {
    (data[offset + 1] as u16) + ((data[offset] as u16) << 8)
}

pub fn set_bitfield(data: &mut u16, val: u16, field: u16, offset: usize) {
    let to_set = (val << offset) & field;
    let remain = *data & !field;
    *data = remain | to_set;
}

pub fn extract_bitfield(data: u16, field: u16, offset: usize) -> u16 {
    (data & field) >> offset
}

pub fn set_u16(data: &mut [u8], offset: usize, v: u16) {
    data[offset + 1] = (v & 0xFF) as u8;
    data[offset] = ((v >> 8) & 0xFF) as u8;
}

pub fn set_bit(v: &mut u16, cnd: bool, bit: u16) {
    if cnd {
        *v = *v | bit;
    } else {
        *v = *v & !bit;
    }
}

pub fn get_bit(v: u16, bit: u16) -> bool {
    v & bit != 0
}
