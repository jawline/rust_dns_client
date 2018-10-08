use bits::*;
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

impl Answer {

    pub fn from(data: &[u8], current: usize) -> Result<(Answer, usize), String> {

        println!("Start answer {:?} {}", &data, current);

        let (names, current) = extract_domain_name(data, current)?;

        println!("Extract name");

        let type_code = extract_u16(data, current)?;
        let class_code = extract_u16(data, current + 2)?;
        let ttl = extract_u32(data, current + 4)?;
        let rdlength = extract_u16(data, current + 8)?;

        println!("Record {} {} {} {} {:?}", type_code, class_code, ttl, rdlength, &data[current + 10..]);
        let record = Record::from(type_code, data, current + 10)?;

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
