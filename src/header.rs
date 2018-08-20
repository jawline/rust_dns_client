use bits::*;

/**
 * Header size in bytes
 */
pub const HEADER_SIZE: usize = 12;

const ID_OFFSET: usize = 0;
const DATA_OFFSET: usize = 2;
const QD_OFFSET: usize = 4;
const AN_OFFSET: usize = 6;
const NS_OFFSET: usize = 8;
const AR_OFFSET: usize = 10;

const RESPONSE_BIT: u16 = 0x1;

const AA_BIT: u16 = 0x1 << 5;
const TC_BIT: u16 = 0x1 << 6;
const RD_BIT: u16 = 0x1 << 7;
const RA_BIT: u16 = 0x1 << 8;

const OPCODE_OFFSET: usize = 1;
const OPCODE_BITS: u16 = (0x1 << 1) | (0x1 << 2) | (0x1 << 3) | (0x1 << 4);

const Z_OFFSET: usize = 9;
const Z_BITS: u16 = (0x1 << 9) | (0x1 << 10) | (0x1 << 11) | (0x1 << 12);

#[derive(Debug)]
pub struct Header {

    /** First u16 is id **/
    pub id: u16,

    /** Second u16 is a packed data structure **/
    pub response: bool,
    pub opcode: u8,
    pub authorative_answer: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub response_code: u8,

    /** The rest is 16 bit aligned values **/
    pub questions: u16,
    pub answers: u16,
    pub nameservers: u16,
    pub additional_records: u16
}

impl Header {

    pub fn blank() -> Header {
        Header {
            id: 0,
            response: false,
            opcode: 0,
            authorative_answer: false,
            truncated: false,
            recursion_desired: false,
            recursion_available: false,
            response_code: 0,
            questions: 0,
            answers: 0,
            nameservers: 0,
            additional_records: 0
        }
    }

    pub fn write(&self, data: &mut [u8]) {
        set_u16(data, ID_OFFSET, self.id);
        
        let mut data_row: u16 = 0;
        set_bit(&mut data_row, self.response, RESPONSE_BIT);

        set_bitfield(&mut data_row, self.opcode as u16, OPCODE_BITS, OPCODE_OFFSET);

        set_bit(&mut data_row, self.authorative_answer, AA_BIT);
        set_bit(&mut data_row, self.truncated, TC_BIT);
        set_bit(&mut data_row, self.recursion_desired, RD_BIT);
        set_bit(&mut data_row, self.recursion_available, RA_BIT);

        set_bitfield(&mut data_row, self.response_code as u16, Z_BITS, Z_OFFSET);
       
        set_u16(data, DATA_OFFSET, data_row); 
        set_u16(data, QD_OFFSET, self.questions);
        set_u16(data, AN_OFFSET, self.answers);
        set_u16(data, NS_OFFSET, self.nameservers);
        set_u16(data, AR_OFFSET, self.additional_records);
    }

    pub fn read(&mut self, data: &[u8]) -> Result<(), String> {
        self.id = extract_u16(data, ID_OFFSET)?;

        let data_row = extract_u16(data, DATA_OFFSET)?;

        self.opcode = extract_bitfield(data_row, OPCODE_BITS, OPCODE_OFFSET) as u8;
        self.response = get_bit(data_row, RESPONSE_BIT);
        self.authorative_answer = get_bit(data_row, AA_BIT);
        self.truncated = get_bit(data_row, TC_BIT);
        self.recursion_desired = get_bit(data_row, RD_BIT);
        self.recursion_available = get_bit(data_row, RA_BIT);
        self.response_code = extract_bitfield(data_row, Z_BITS, Z_OFFSET) as u8;

        self.questions = extract_u16(data, QD_OFFSET)?;
        self.answers = extract_u16(data, AN_OFFSET)?;
        self.nameservers = extract_u16(data, NS_OFFSET)?;
        self.additional_records = extract_u16(data, AR_OFFSET)?;
        
        Ok(())
    }

    pub fn from(data: &[u8], current: usize) -> Result<(Header, usize), String> {
        let mut header = Header::blank();
        header.read(&data[current..])?;
        Ok((header, HEADER_SIZE))
    }

}
