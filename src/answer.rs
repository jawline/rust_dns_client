pub struct Answer {
    pub name: Vec<String>,
    pub type_code: u16,
    pub class_code: u16,
    pub ttl: u32,
    pub rdlength: u16
}

impl Answer {

    pub fn extract(data: &[u8]) {
        
    }

}
