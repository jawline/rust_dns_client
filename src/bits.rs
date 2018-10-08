use std::str;

pub fn extract_u16(data: &[u8], offset: usize) -> Result<u16, String> {
    if data.len() >= offset + 2 {
        Ok((data[offset + 1] as u16) + ((data[offset] as u16) << 8))
    } else {
        Err("no data".to_string())
    }
}

pub fn set_u16(data: &mut [u8], offset: usize, v: u16) {
    let lo = v & 0xFF;
    let hi = (v >> 8) & 0xFF;

    data[offset + 1] = lo as u8;
    data[offset] = hi as u8;
}

pub fn extract_u32(data: &[u8], offset: usize) -> Result<u32, String> {
    let lo = extract_u16(data, offset + 2)? as u32;
    let hi = (extract_u16(data, offset)? as u32) << 16;
    Ok(lo + hi)
}

pub fn set_bitfield(data: &mut u16, val: u16, field: u16, offset: usize) {
    let to_set = (val << offset) & field;
    let remain = *data & !field;
    *data = remain | to_set;
}

pub fn extract_bitfield(data: u16, field: u16, offset: usize) -> u16 {
    (data & field) >> offset
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

pub fn extract_character_string(data: &[u8], current: usize) -> Result<(String, usize), String> {
    let len = data[current] as usize;
    let string = str::from_utf8(&data[current + 1..current + 1 + len]);

    if let Err(e) = string {
        return Err(e.to_string());
    }

    let string = string.unwrap().to_string();
    Ok((string, current + 1 + len))
}

pub fn extract_domain_name(data: &[u8], current: usize) -> Result<(Vec<String>, usize), String> {
    const PTR_BITS: u8 = (1 << 6) | (1 << 7);
    let is_done = data[current] == 0;

    if is_done {
        Ok((Vec::new(), current + 1))
    } else {
        let is_ptr = PTR_BITS & data[current] != 0;

        if is_ptr {
            let ptr = extract_u16(data, current)? & !(1 << 15 | 1 << 14);
            Ok((extract_domain_name(data, ptr as usize)?.0, current + 2))
        } else {
            let len = data[current];
            let word = str::from_utf8(&data[current + 1..current + 1 + len as usize]);
            
            if let Err(e) = word {
                return Err(e.to_string());
            }

            let word = word.unwrap().to_string();

            let mut this_parts = vec![word];
            let (next_parts, current) = extract_domain_name(data, current + 1 + len as usize)?;
            this_parts.extend(next_parts);
            Ok((this_parts, current))
        }
    }
}
