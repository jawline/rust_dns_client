use bits::*;

#[derive(Debug)]
pub struct Question {
    pub portions: Vec<String>,
    pub type_code: u16,
    pub class_code: u16
}

impl Question {

    pub fn new(domain: &str, type_code: u16) -> Question {
        let mut portions = Vec::new();        

        for s in domain.split(".") {
            portions.push(s.to_string()); 
        }

        Question {
            portions: portions,
            type_code: type_code,
            class_code: 1
        } 
    }

    pub fn write(&self, data: &mut [u8], current: usize) -> usize {
        let mut current = current;

        for portion in &self.portions {

            let bytes = portion.as_bytes();

            data[current] = bytes.len() as u8;
            current += 1;

            for i in 0..bytes.len() {
                data[i + current] = bytes[i];
            }

            current += bytes.len();
        }

        data[current] = 0;
        current += 1;

        set_u16(data, current, self.type_code);
        current += 2;

        set_u16(data, current, self.class_code);
        current += 2;

        current
    }

    pub fn from(data: &[u8], current: usize) -> Result<(Question, usize), String> {
        let (words, current) = extract_domain_name(data, current)?;
        let type_code = extract_u16(data, current)?;
        let class_code = extract_u16(data, current + 2)?;
        
        Ok((Question {
            portions: words,
            type_code: type_code,
            class_code: class_code
        }, current + 4))
    }

}
