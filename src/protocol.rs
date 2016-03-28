use std::io::{Cursor, Read, Write};
use std::mem::size_of;
use std::net::{
    TcpStream,
    ToSocketAddrs
};
use std::str::{
    from_utf8
};

use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};

enum Types {
    Request = 0x80,
    Response = 0x81
}

#[derive(Debug)]
enum Commands {
    Get = 0x00,
    Set = 0x01,
    Add = 0x02,
    Replace = 0x03,
    Delete = 0x04,
    Increment = 0x05,
    Decrement = 0x06,
    Quit = 0x07,
    Flush = 0x08,
    GetQ = 0x09,
    NoOp = 0x0A,
    Version = 0x0B,
    GetK = 0x0C,
    GetKQ = 0x0D,
    Append = 0x0E,
    Prepend = 0x0F,
    Stat = 0x10,
    SetQ = 0x11,
    AddQ = 0x12,
    ReplaceQ = 0x13,
    DeleteQ = 0x14,
    IncrementQ = 0x15,
    DecrementQ = 0x16,
    QuitQ = 0x17,
    FlushQ = 0x18,
    AppendQ = 0x19,
    PrependQ = 0x1A
}

pub struct Request {
    magic: u8,
    opcode: u8,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
    reserved: u16,
    body_length: u32,
    opaque: u32,
    cas: u64
}

pub struct Response {
    magic: u8,
    opcode: u8,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
    status: u16,
    opaque: u32,
    cas: u64
}

pub struct SetAddReplace {
    flags: u32,
    expiration: u32
}

pub struct Protocol {
    connection: TcpStream
}

impl Protocol {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Protocol {
        assert_eq!(size_of::<Response>(), 24);
        Protocol{connection: TcpStream::connect(addr).unwrap()}
    }

    fn set(&mut self, key: &'static str, value: &'static str, time: u32) -> usize {
        let extras_length = size_of::<SetAddReplace>() as u8;
        println!("Using command {:?} with value {:?}", Commands::Set, Commands::Set as u8);
        println!("Extras length {:?}", extras_length);
        let request = Request{magic: Types::Request as u8, opcode: Commands::Set as u8,
            key_length: key.len() as u16,
            extras_length: extras_length,
            data_type: 0, reserved: 0, body_length: key.len() as u32 + value.len() as u32 + extras_length as u32,
            opaque: 0, cas: 0x00};
        let mut buf = vec![];
        buf.write_u8(request.magic).unwrap();
        buf.write_u8(request.opcode).unwrap();
        buf.write_u16::<BigEndian>(request.key_length).unwrap();
        buf.write_u8(request.extras_length).unwrap();
        buf.write_u8(request.data_type).unwrap();
        buf.write_u16::<BigEndian>(request.reserved).unwrap();
        buf.write_u32::<BigEndian>(request.body_length).unwrap();
        buf.write_u32::<BigEndian>(request.opaque).unwrap();
        buf.write_u64::<BigEndian>(request.cas).unwrap();
        // Flags
        buf.write_u32::<BigEndian>(0);
        buf.write_u32::<BigEndian>(time).unwrap();
        buf.write(key.as_bytes()).unwrap();
        buf.write(value.as_bytes()).unwrap();
        println!("{:?} {}", buf, buf.len());
        let wrote_size = self.connection.write(buf.as_slice()).unwrap();
        let mut outbuf = [0; 24];
        self.connection.read_exact(&mut outbuf).unwrap();
        println!("{:?} {}", outbuf, outbuf.len());
        wrote_size
    }

    fn get(&mut self, key: &'static str) -> String {
        println!("Get");
        let request = Request{magic: Types::Request as u8, opcode: Commands::Get as u8,
            key_length: key.len() as u16,
            extras_length: 0,
            data_type: 0, reserved: 0, body_length: key.len() as u32,
            opaque: 0, cas: 0x00};
            let mut buf = vec![];
            buf.write_u8(request.magic).unwrap();
            buf.write_u8(request.opcode).unwrap();
            buf.write_u16::<BigEndian>(request.key_length).unwrap();
            buf.write_u8(request.extras_length).unwrap();
            buf.write_u8(request.data_type).unwrap();
            buf.write_u16::<BigEndian>(request.reserved).unwrap();
            buf.write_u32::<BigEndian>(request.body_length).unwrap();
            buf.write_u32::<BigEndian>(request.opaque).unwrap();
            buf.write_u64::<BigEndian>(request.cas).unwrap();
            buf.write(key.as_bytes()).unwrap();
            println!("Request read {:?}", buf);
            self.connection.write(buf.as_slice()).unwrap();
            let mut outbuf = [0; 24];
            self.connection.read_exact(&mut outbuf).unwrap();
            println!("Got {:?} of size {:?}", outbuf, outbuf.len());
            let mut cur = Cursor::new(outbuf);
            assert_eq!(Types::Response as u8, cur.read_u8().unwrap());
            let mut read_return = [0; 5];
            self.connection.read_u32::<BigEndian>().unwrap();
            self.connection.read_exact(&mut read_return);
            let a = from_utf8(&read_return).unwrap();
            a.to_owned()
    }
}

#[test]
fn test_set_key() {
    let mut p = Protocol::connect("127.0.0.1:11211");
    assert_eq!(p.set("Hello", "World", 100), 201);
}

#[test]
fn test_get_key() {
    let mut p = Protocol::connect("127.0.0.1:11211");
    p.set("Hello", "World", 100);
    assert_eq!(p.get("Hello"),  "World");
}
