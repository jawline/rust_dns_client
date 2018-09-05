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

pub fn extract_string_maybe_ptr(data: &[u8], current: usize) -> Result<(Vec<String>, usize), String> {
    
    const PTR_BITS: u8 = (1 << 6) | (1 << 7);

    //Grab the first 16 bits of the answer to decide if its a ptr
    
    if data[current] & PTR_BITS != 0 { //Names is a ptr
        
        let ptr = extract_u16(data, current)?;
        let start = ptr & !(1 << 15 | 1 << 14);

        println!("POINTER TO {} in {:?} / {:?} size {}", start, data, &data[current..], data.len());
        let (names, _) = extract_string(data, start as usize)?;
        println!("Done extract {:?}", names);
        Ok((names, current + 2))
    } else { //names is grabbed with the extract_string function
        extract_string(data, current)
    }
}

pub fn extract_string(data: &[u8], current: usize) -> Result<(Vec<String>, usize), String> {

    if current >= data.len() {
        return Err("no more data".to_string())
    }

    let mut words = Vec::new();
    let mut current = current;

    loop {

        let len = data[current];
        current += 1;

        if len == 0 {
            break;
        }

        let word = str::from_utf8(&data[current..current + len as usize]);

        if let Err(e) = word {
            return Err(e.to_string());
        }

        words.push(word.unwrap().to_string());
        current += len as usize;
    }
    
    Ok((words, current))
}
