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

#[derive(Debug)]
pub struct Response {
    magic: u8,
    opcode: u8,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
    status: u16,
    body_length: u32,
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

    fn build_request(command: Commands, key_length: usize, value_length: usize, data_type: u8, extras_length: usize, cas: u64) -> Request {
        Request {
            magic: Types::Request as u8, opcode: command as u8,
            key_length: key_length as u16,
            extras_length: extras_length as u8,
            data_type: data_type, reserved: 0, body_length: (key_length + value_length + extras_length) as u32,
            opaque: 0, cas: 0x00
        }
    }

    fn write_request(&self, request: Request, final_payload: &[u8]) -> usize {
        let mut buf = &self.connection;
        buf.write_u8(request.magic).unwrap();
        buf.write_u8(request.opcode).unwrap();
        buf.write_u16::<BigEndian>(request.key_length).unwrap();
        buf.write_u8(request.extras_length).unwrap();
        buf.write_u8(request.data_type).unwrap();
        buf.write_u16::<BigEndian>(request.reserved).unwrap();
        buf.write_u32::<BigEndian>(request.body_length).unwrap();
        buf.write_u32::<BigEndian>(request.opaque).unwrap();
        buf.write_u64::<BigEndian>(request.cas).unwrap();
        buf.write(final_payload).unwrap()
    }

    fn set_add_replace(&mut self, command: Commands, key: &'static str, value: &'static str, time: u32) -> usize {
        let extras_length = size_of::<SetAddReplace>();
        let request = Protocol::build_request(command, key.len(), value.len(), 0x00, extras_length, 0x00);
        let mut final_payload = vec![];
        // Flags
        final_payload.write_u32::<BigEndian>(0);
        final_payload.write_u32::<BigEndian>(time).unwrap();
        // After flags key and value
        final_payload.write(key.as_bytes()).unwrap();
        final_payload.write(value.as_bytes()).unwrap();
        let size = self.write_request(request, final_payload.as_slice());
        self.read_response();
        size
    }

    fn read_response(&mut self) -> Response {
        let mut buf = &self.connection;
        let magic: u8 = buf.read_u8().unwrap();
        assert_eq!(magic, Types::Response as u8);
        Response {
            magic: magic,
            opcode: buf.read_u8().unwrap(),
            key_length: buf.read_u16::<BigEndian>().unwrap(),
            extras_length: buf.read_u8().unwrap(),
            data_type: buf.read_u8().unwrap(),
            status: buf.read_u16::<BigEndian>().unwrap(),
            body_length: buf.read_u32::<BigEndian>().unwrap(),
            opaque: buf.read_u32::<BigEndian>().unwrap(),
            cas: buf.read_u64::<BigEndian>().unwrap()
        }
    }

    fn set(&mut self, key: &'static str, value: &'static str, time: u32) -> usize {
        self.set_add_replace(Commands::Set, key, value, time)
    }

    fn add(&mut self, key: &'static str, value: &'static str, time: u32) -> usize {
        self.set_add_replace(Commands::Add, key, value, time)
    }

    fn replace(&mut self, key: &'static str, value: &'static str, time: u32) -> usize {
        self.set_add_replace(Commands::Replace, key, value, time)
    }

    fn get(&mut self, key: &'static str) -> String {
        let request = Protocol::build_request(Commands::Get, key.len(), 0 as usize, 0, 0, 0x00);
        self.write_request(request, key.as_bytes());
        let response = self.read_response();
        // Discard extras for now
        self.connection.read_u32::<BigEndian>().unwrap();
        let mut outbuf = vec![0; (response.body_length - response.extras_length as u32) as usize];
        self.connection.read_exact(&mut outbuf).unwrap();
        println!("{:?} {:?}", outbuf, outbuf.len());
        let a = from_utf8(&outbuf).unwrap();
        a.to_owned()
    }
}

#[test]
fn test_get_key() {
    let mut p = Protocol::connect("127.0.0.1:11211");
    p.set("Hello", "World", 100);
    assert_eq!(p.get("Hello"),  "World");
}
