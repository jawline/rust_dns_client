use bits::*;
use std::net::{ IpAddr, Ipv4Addr };
use record::Record;

#[derive(Debug)]
pub struct Answer {
    pub name: Vec<String>,
    pub type_code: u16,
    pub class_code: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Record
}

const PTR_BITS: u16 = (1 << 14) | (1 << 15);

impl Answer {

    pub fn extract(data: &[u8], current: usize) -> Result<(Answer, usize), String> {

        //Grab the first 16 bits of the answer to decide if its a ptr
        let ptr = extract_u16(data, current);

        let (names, current) = if (ptr & PTR_BITS != 1) { //Names is a ptr
            let start = ptr & !PTR_BITS;
            let (names, _) = extract_string(data, start as usize)?;
            (names, current + 2) 
        } else { //names is grabbed with the extract_string function
            extract_string(data, current)?
        };

        let type_code = extract_u16(data, current);
        let class_code = extract_u16(data, current + 2);
        let ttl = extract_u32(data, current + 4);
        let rdlength = extract_u16(data, current + 8);
        let record = Record::from(type_code, &data[current + 10..])?;

        Ok((Answer {
            name: names,
            type_code: type_code,
            class_code: class_code,
            ttl: ttl,
            rdlength: rdlength,
            rdata: record
        }, current + 10 + rdlength as usize))
    }

}
