use std::io::{Cursor, Read, Write, Error};
use std::mem::size_of;
use std::net::{
    TcpStream,
    ToSocketAddrs
};
use std::str::{
    from_utf8
};

use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};

use errors::BMemcachedError;

enum Type {
    Request = 0x80,
    Response = 0x81
}

#[derive(Debug)]
enum Command {
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

#[derive(Debug)]
enum Status {
    Success = 0x00,
    KeyNotFound = 0x01,
    KeyExists = 0x02,
    AuthError = 0x08,
    UnknownCommand = 0x81
}

#[derive(Debug)]
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

    fn build_request(command: Command, key_length: usize, value_length: usize, data_type: u8, extras_length: usize, cas: u64) -> Request {
        Request {
            magic: Type::Request as u8, opcode: command as u8,
            key_length: key_length as u16,
            extras_length: extras_length as u8,
            data_type: data_type, reserved: 0,
            body_length: (key_length + value_length + extras_length) as u32,
            opaque: 0, cas: cas
        }
    }

    fn write_request(&self, request: Request, final_payload: &[u8]) -> Result<(), BMemcachedError> {
        let mut buf = &self.connection;
        try!(buf.write_u8(request.magic));
        try!(buf.write_u8(request.opcode));
        try!(buf.write_u16::<BigEndian>(request.key_length));
        try!(buf.write_u8(request.extras_length));
        try!(buf.write_u8(request.data_type));
        try!(buf.write_u16::<BigEndian>(request.reserved));
        try!(buf.write_u32::<BigEndian>(request.body_length));
        try!(buf.write_u32::<BigEndian>(request.opaque));
        try!(buf.write_u64::<BigEndian>(request.cas));
        try!(buf.write(final_payload));
        Ok(())
    }

    fn set_add_replace(&mut self, command: Command, key: String, value: String, time: u32) -> Result<(), BMemcachedError> {
        let extras_length = size_of::<SetAddReplace>();
        let request = Protocol::build_request(command, key.len(), value.len(), 0x00, extras_length, 0x00);
        let mut final_payload = vec![];
        // Flags
        try!(final_payload.write_u32::<BigEndian>(0));
        try!(final_payload.write_u32::<BigEndian>(time));
        // After flags key and value
        try!(final_payload.write(key.as_bytes()));
        try!(final_payload.write(value.as_bytes()));
        let size = try!(self.write_request(request, final_payload.as_slice()));
        self.read_response();
        Ok(())
    }

    fn read_response(&mut self) -> Response {
        let mut buf = &self.connection;
        let magic: u8 = buf.read_u8().unwrap();
        assert_eq!(magic, Type::Response as u8);
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

    fn set(&mut self, key: String, value: String, time: u32) -> Result<(), BMemcachedError> {
        self.set_add_replace(Command::Set, key, value, time)
    }

    fn add(&mut self, key: String, value: String, time: u32) -> Result<(), BMemcachedError> {
        self.set_add_replace(Command::Add, key, value, time)
    }

    fn replace(&mut self, key: String, value: String, time: u32) -> Result<(), BMemcachedError> {
        self.set_add_replace(Command::Replace, key, value, time)
    }

    fn get(&mut self, key: String) -> Result<String, BMemcachedError> {
        let request = Protocol::build_request(Command::Get, key.len(), 0 as usize, 0, 0, 0x00);
        self.write_request(request, key.as_bytes());
        let response = self.read_response();
        // Discard extras for now
        try!(self.connection.read_u32::<BigEndian>());
        let mut outbuf = vec![0; (response.body_length - response.extras_length as u32) as usize];
        try!(self.connection.read_exact(&mut outbuf));
        let a = try!(from_utf8(&outbuf));
        Ok(a.to_owned())
    }
}

#[test]
fn test_set_key() {
    let mut p = Protocol::connect("127.0.0.1:11211");
    let key = "Hello".to_string();
    let value = "World".to_string();
    p.set(key.to_owned(), value.to_owned(), 100).unwrap()
}

#[test]
fn test_get_key() {
    let mut p = Protocol::connect("127.0.0.1:11211");
    let key = "Hello".to_string();
    let value = "World".to_string();
    p.set(key.to_owned(), value.to_owned(), 100).unwrap();
    assert_eq!(p.get(key.to_owned()).unwrap(),  value.to_owned())
}
