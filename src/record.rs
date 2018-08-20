use bits::*;
use std::net::{ IpAddr, Ipv4Addr };

pub const A_CODE: u16 = 0x1;
pub const NS_CODE: u16 = 0x2;
pub const CNAME_CODE: u16 = 0x5;
pub const SOA_CODE: u16 = 0x6;
pub const WKS_CODE: u16 = 0xB;
pub const PTR_CODE: u16 = 0xC;
pub const MX_CODE: u16 = 0xF;
pub const SRV_CODE: u16 = 0x21;
pub const AAAA_CODE: u16 = 0x1C;
pub const ANY_CODE: u16 = 0xFF;

#[derive(Debug)]
pub enum Record {
    A(Ipv4Addr),
    CNAME(Vec<String>)
}

impl Record { 

    pub fn from(type_code: u16, data: &[u8], current: usize) -> Result<Record, String> {
        match type_code {

            A_CODE => if data.len() >= current + 4 {
                Ok(Record::A(Ipv4Addr::new(data[current], data[current + 1], data[current + 2], data[current + 3])))
            } else { Err("a record no data".to_string()) },

            CNAME_CODE => {
                let (name, _) = extract_string_maybe_ptr(data, current)?;
                Ok(Record::CNAME(name))
            },

            _ => Err( format!("unknown type code {}", type_code) )
        } 
    }

}
