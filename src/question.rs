use bits::*;

#[derive(Debug)]
pub struct Question {
    pub portions: Vec<String>,
    pub type_code: u16,
    pub class_code: u16
}

impl Question {

    pub fn new(domain: &str) -> Question {
        let mut portions = Vec::new();        

        for s in domain.split(".") {
            portions.push(s.to_string()); 
        }

        Question {
            portions: portions,
            type_code: 1,
            class_code: 1
        } 
    }

    pub fn write(&self, data: &mut [u8]) -> usize {
        let mut current = 0;

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

    pub fn read(&mut self, data: &[u8], current: usize) -> Result<usize, String> {

        let (words, mut current) = extract_string(data, current)?;
        
        self.portions = words;
        self.type_code = extract_u16(data, current)?;
        self.class_code = extract_u16(data, current + 2)?;
        
        Ok(current + 4)
    }

}
