mod bits;
mod header;
mod question;
mod maker;
mod answer;
mod record;

use std::thread;
use std::net;

use question::Question;
use header::{ Header, HEADER_SIZE };
use maker::Maker;
use answer::Answer;

pub fn print_response(buf: &[u8]) -> Result<(), String> {

    //Process the response
    let (header, mut current) = Header::from(buf, 0)?;

    println!("Recv: {:?} {}", &header, current);

    for i in 0..header.questions {
        let (question, nc) = Question::from(buf, current)?;
        current = nc;
        println!("Recv: {:?}", &question);
    }

    for i in 0..header.answers {
        let (answer, nc) = Answer::from(buf, current)?;
        current = nc;
        println!("Recv: {:?}", &answer);
    }

    Ok(())
}

fn send(buf: &mut [u8], question: &Question, maker: &Maker) -> std::io::Result<usize> {

    let mut header = Header::blank();
    header.questions = 1;
    header.write(buf);

    let size = question.write(buf, HEADER_SIZE);
    maker.send(&buf[0..size]) 
}

fn response(buf: &mut [u8], maker: &Maker) -> std::io::Result<()> {
    let amt = maker.recv(buf)?;
    let msg_buf = &buf[0..amt];

    print_response(msg_buf).unwrap();
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("UDP");

    let me = "0.0.0.0:53123";
    let target_dns = "1.1.1.1:53";

    let mut maker = Maker::new(me, target_dns).unwrap();
    let mut question = Question::new(&std::env::args().last().unwrap(), record::A_CODE);

    let mut msg_buf = [0; 65536];

    send(&mut msg_buf, &question, &maker)?;
    response(&mut msg_buf, &maker)?;

    Ok(())
}
