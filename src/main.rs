mod bits;
mod header;
mod question;
mod maker;
mod answer;

use std::thread;
use std::net;

use question::Question;
use header::{ Header, HEADER_SIZE };
use maker::Maker;
use answer::Answer;

fn main() -> std::io::Result<()> {
    println!("UDP");

    let me = "0.0.0.0:53123";
    let target_dns = "1.1.1.1:53";

    let mut maker = Maker::new(me, target_dns).unwrap();

    println!("Sending handshake!");

    let mut header = Header{
        id: 0,
        response: false,
        opcode: 0,
        authorative_answer: false,
        truncated: false,
        recursion_desired: false,
        recursion_available: false,
        response_code: 0,
        questions: 1,
        answers: 0,
        nameservers: 0,
        additional_records: 0
    };

    let mut question = Question::new("www.google.com");

    let mut msg_buf = [0; 4096];

    //Write the header to the buffer start
    header.write(&mut msg_buf);
    let size = question.write(&mut msg_buf[HEADER_SIZE..]);

    println!("Send: {:?}", header);
    println!("Send: {:?}", question);

    maker.send(&msg_buf[0..HEADER_SIZE + size])?; 
    let amt = maker.recv(&mut msg_buf)?;

    //Read it into the header
    header.read(&msg_buf);
    let mut current = HEADER_SIZE + question.read(&msg_buf[HEADER_SIZE..]);

    println!("Recv: {:?}", &header);
    println!("Recv: {:?}", &question);

    Ok(())
}
