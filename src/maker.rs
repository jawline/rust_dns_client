use log::*;

use std::io::Error;
use std::net::UdpSocket;
use std::time::Duration;

pub struct Maker {
    socket: UdpSocket,
    target: String
}

impl Maker {

    pub fn new(src: &str, tgt: &str) -> Result<Maker, Error> {

        let socket = UdpSocket::bind(src)?;
        socket.set_read_timeout(Some(Duration::from_millis(2000)))?;

        Ok(Maker {
            socket: socket,
            target: tgt.to_string()
        })
    }

    pub fn send(&self, msg: &[u8]) -> Result<usize, Error> {
        debug!("Send {} bytes {:x?}", msg.len(), msg);
        self.socket.send_to(msg, &self.target)
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let amt = self.socket.recv(buf)?;
        debug!("Recv {} bytes {:x?}", amt, &buf[0..amt]);
        Ok(amt)
    }

}
