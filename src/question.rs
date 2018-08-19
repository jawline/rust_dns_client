use std::str;
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

    pub fn read(&mut self, data: &[u8]) -> usize {

        let mut words = Vec::new();

        let mut cur = 0;

        loop {

            let len = data[cur];
            cur += 1;

            if len == 0 {
                break;
            }

            let word = str::from_utf8(&data[0..cur]).unwrap().to_string();
            cur += len as usize;

            words.push(word);
        }

        self.portions = words;
        self.type_code = extract_u16(data, cur);
        self.class_code = extract_u16(data, cur + 2);

        cur += 4;

        cur
    }

}
