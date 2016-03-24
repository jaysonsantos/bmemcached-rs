use std::io::Write;
use byteorder::{Read}

use std::net::{
    TcpStream,
    ToSocketAddrs
};
// transu
pub struct Request {
    magic: u8,
    opcode: u8,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
    reserved: u16,
    body_length: u32,
    opaque: u32,
    cas: u32
}

pub struct Response {
    magic: u8,
    opcode: u8,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
    status: u16,
    opaque: u32,
    cas: u32
}

pub struct Protocol {
    connection: TcpStream
}

impl Protocol {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Protocol {
        Protocol{connection: TcpStream::connect(addr).unwrap()}
    }

    fn set(&mut self, key: &'static str, value: &'static str, time: u16) -> usize {
        let request = Request{magic: 0x80, opcode: 0x01, key_length: key.len() as u16,
            extras_length: 8, data_type: 0, reserved: 0, body_length: value.len() as u32 + 8,
            opaque: 0, cas: 0x00};
        let mut buf = vec!();
        // let set = SetRequest{0x00, time, key.to_bytes(), value.to_bytes()};
        let wrote_size = self.connection.write(&[request.magic, request.opcode, request.key_length]).unwrap();
        wrote_size
    }
}

#[test]
fn test_set_key() {
    let mut p = Protocol::connect("127.0.0.1:11211");
    assert_eq!(p.set("abc", "123", 100), 2);
}
