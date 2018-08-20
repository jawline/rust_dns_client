use bits::*;

#[derive(Debug)]
pub struct Answer {
    pub name: Vec<String>,
    pub type_code: u16,
    pub class_code: u16,
    pub ttl: u32,
    pub rdlength: u16
}

impl Answer {

    pub fn extract(data: &[u8]) -> (Answer, usize) {
        let mut cur = 2;
 
        (Answer {
            name: Vec::new(),
            type_code: extract_u16(data, cur),
            class_code: extract_u16(data, cur + 2),
            ttl: extract_u32(data, cur + 4),
            rdlength: extract_u16(data, cur + 8)
        }, cur + 10)
    }

}
