use bits::*;
use std::net::{ IpAddr, Ipv4Addr };

#[derive(Debug)]
pub enum RData {
    A(Ipv4Addr)
}

#[derive(Debug)]
pub struct Answer {
    pub name: Vec<String>,
    pub type_code: u16,
    pub class_code: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: RData
}

const PTR_BITS: u16 = (1 << 14) | (1 << 15);

fn extract_rdata(type_code: u16, class_code: u16, data: &[u8]) -> Option<RData> {
    if type_code == 1 {
        Some(RData::A(Ipv4Addr::new(data[0], data[1], data[2], data[3])))
    } else {
        None
    }
}

impl Answer {

    pub fn extract(data: &[u8], current: usize) -> Option<(Answer, usize)> {

        //Grab the first 16 bits of the answer to decide if its a ptr
        let ptr = extract_u16(data, current);

        let (names, current) = if (ptr & PTR_BITS != 1) { //Names is a ptr
            let start = ptr & !PTR_BITS;
            let (names, _) = extract_string(data, start as usize);
            (names, current + 2) 
        } else { //names is grabbed with the extract_string function
            extract_string(data, current)
        };


        let type_code = extract_u16(data, current);
        let class_code = extract_u16(data, current + 2);
        let ttl = extract_u32(data, current + 4);
        let rdlength = extract_u16(data, current + 8);

        if let Some(rdata) = extract_rdata(type_code, class_code, &data[current + 10..]) {
 
            Some((Answer {
                name: names,
                type_code: type_code,
                class_code: class_code,
                ttl: ttl,
                rdlength: rdlength,
                rdata: rdata
            }, current + 10 + rdlength as usize))

        } else {
            None
        }
    }

}
