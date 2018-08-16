mod bits;
mod header;

use std::thread;
use std::net;
use std::time::Duration;
use std::net::{ UdpSocket, SocketAddr };

use header::Header;

fn main() -> std::io::Result<()> {
    println!("UDP");

    let me = "0.0.0.0:53123";
    let target_dns = "1.1.1.1:53";

    let socket = UdpSocket::bind(me).expect("Cant bind host address");
    socket.set_read_timeout(Some(Duration::from_millis(2000)))?;

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
        questions: 0,
        answers: 0,
        nameservers: 0,
        additional_records: 0
    };

    let mut msg_buf = [0; 4096];

    //Write the header to the buffer start
    header.write(&mut msg_buf);
    println!("Send: {:?}", &header);
    println!("Send {:?}", &msg_buf[0..12]);

    //Send the message
    socket.send_to(&msg_buf[0..12], target_dns).expect("Cant send data");

    //Recv the message
    let (amt, src) = socket.recv_from(&mut msg_buf)?;
    println!("RCV: {:?}", &msg_buf[0..amt]);

    //Read it into the header
    header.read(&msg_buf);

    println!("Recv: {:?}", &header);

    Ok(())
}
