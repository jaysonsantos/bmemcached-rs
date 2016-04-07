use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{
    TcpStream,
    ToSocketAddrs
};

use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
use num::FromPrimitive;

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
    // Increment = 0x05,
    // Decrement = 0x06,
    // Quit = 0x07,
    // Flush = 0x08,
    // GetQ = 0x09,
    // NoOp = 0x0A,
    // Version = 0x0B,
    // GetK = 0x0C,
    // GetKQ = 0x0D,
    // Append = 0x0E,
    // Prepend = 0x0F,
    // Stat = 0x10,
    // SetQ = 0x11,
    // AddQ = 0x12,
    // ReplaceQ = 0x13,
    // DeleteQ = 0x14,
    // IncrementQ = 0x15,
    // DecrementQ = 0x16,
    // QuitQ = 0x17,
    // FlushQ = 0x18,
    // AppendQ = 0x19,
    // PrependQ = 0x1A
}


enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum Status {
        Success = 0x00,
        KeyNotFound = 0x01,
        KeyExists = 0x02,
        AuthError = 0x08,
        UnknownCommand = 0x81
    }
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

#[derive(Debug)]
pub struct Protocol {
    connection: TcpStream
}

impl Protocol {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Protocol, BMemcachedError> {
        Ok(Protocol{connection: try!(TcpStream::connect(addr))})
    }

    pub fn connection_info(&self) -> String {
        format!("{:?}", self.connection.peer_addr().unwrap())
    }

    fn build_request(command: Command, key_length: usize, value_length: usize, data_type: u8,
        extras_length: usize, cas: u64) -> Request {
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

    fn read_response(&mut self) -> Result<Response, BMemcachedError> {
        let mut buf = &self.connection;
        let magic: u8 = try!(buf.read_u8());
        if magic != Type::Response as u8 {
            // TODO Consume the stream, disconnect or something?
            debug!("Server sent an unknown magic code {:?}", magic);
            return Err(BMemcachedError::UnkownError("Server sent an unknown magic code"))
        }
        Ok(Response {
            magic: magic,
            opcode: try!(buf.read_u8()),
            key_length: try!(buf.read_u16::<BigEndian>()),
            extras_length: try!(buf.read_u8()),
            data_type: try!(buf.read_u8()),
            status: try!(buf.read_u16::<BigEndian>()),
            body_length: try!(buf.read_u32::<BigEndian>()),
            opaque: try!(buf.read_u32::<BigEndian>()),
            cas: try!(buf.read_u64::<BigEndian>())
        })
    }

    fn consume_body(&mut self, size: u32) -> Result<(), BMemcachedError> {
        debug!("Consuming body");
        let mut buf: Vec<u8> = vec![0; size as usize];
        try!(self.connection.read(&mut *buf));
        let str_buf = try!(String::from_utf8(buf));
        debug!("Consumed body {:?}", str_buf);
        Ok(())
    }

    fn set_add_replace<K, V>(&mut self, command: Command, key: K, value: V, time: u32) -> Result<(), BMemcachedError>
        where K: AsRef<[u8]>, V: AsRef<[u8]> {
        let key = key.as_ref();
        let value = value.as_ref();
        let extras_length = 8; // Flags: u32 and Expiration time: u32
        let request = Protocol::build_request(command, key.len(), value.len(), 0x00, extras_length, 0x00);
        let mut final_payload = vec![];
        // Flags
        try!(final_payload.write_u32::<BigEndian>(0));
        try!(final_payload.write_u32::<BigEndian>(time));
        // After flags key and value
        try!(final_payload.write(key));
        try!(final_payload.write(value));
        try!(self.write_request(request, final_payload.as_slice()));
        let response = try!(self.read_response());
        match Status::from_u16(response.status) {
            Some(Status::Success) => Ok(()),
            Some(rest) => {
                try!(self.consume_body(response.body_length));
                Err(BMemcachedError::Status(rest))
            },
            None => Err(BMemcachedError::UnkownError("Server returned an unknown status code"))
        }
    }

    pub fn set<K, V>(&mut self, key: K, value: V, time: u32) -> Result<(), BMemcachedError>
        where K: AsRef<[u8]>, V: AsRef<[u8]> {
        self.set_add_replace(Command::Set, key, value, time)
    }

    pub fn add<K, V>(&mut self, key: K, value: V, time: u32) -> Result<(), BMemcachedError>
        where K: AsRef<[u8]>, V: AsRef<[u8]> {
        self.set_add_replace(Command::Add, key, value, time)
    }

    pub fn replace<K, V>(&mut self, key: K, value: V, time: u32) -> Result<(), BMemcachedError>
        where K: AsRef<[u8]>, V: AsRef<[u8]> {
        self.set_add_replace(Command::Replace, key, value, time)
    }

    pub fn get<K>(&mut self, key: K) -> Result<String, BMemcachedError> where K: AsRef<[u8]> {
        let key = key.as_ref();
        let request = Protocol::build_request(Command::Get, key.len(), 0 as usize, 0, 0, 0x00);
        try!(self.write_request(request, key));
        let response = try!(self.read_response());
        match Status::from_u16(response.status) {
            Some(Status::Success) => {},
            Some(status) => {
                try!(self.consume_body(response.body_length));
                return Err(BMemcachedError::Status(status))
            },
            None => return Err(BMemcachedError::UnkownError("Server sent an unknown status code"))
        };
        // Discard extras for now
        try!(self.connection.read_u32::<BigEndian>());
        let mut outbuf = vec![0; (response.body_length - response.extras_length as u32) as usize];
        try!(self.connection.read_exact(&mut outbuf));
        Ok(try!(String::from_utf8(outbuf)))
    }

    pub fn delete<K>(&mut self, key: K) -> Result<(), BMemcachedError> where K: AsRef<[u8]> {
        let key = key.as_ref();
        let request = Protocol::build_request(Command::Delete, key.len(), 0 as usize, 0, 0, 0x00);
        try!(self.write_request(request, key));
        let response = try!(self.read_response());

        match Status::from_u16(response.status) {
            Some(Status::Success) => Ok(()),
            Some(Status::KeyNotFound) => {
                try!(self.consume_body(response.body_length));
                Ok(())
            },
            Some(status) => {
                try!(self.consume_body(response.body_length));
                Err(BMemcachedError::Status(status))
            },
            None => Err(BMemcachedError::UnkownError("Server sent an unknown status code"))
        }
    }
}
#[cfg(test)]
mod tests {
    extern crate env_logger;
    use errors::BMemcachedError;
    use super::*;

    #[test]
    fn set_key() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello";
        let value = "World";
        p.set(key, value, 1000).unwrap();
        p.delete(key).unwrap();
    }

    #[test]
    fn add_key() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello Add";
        let value = "World";
        p.add(key, value, 10).unwrap();
        let result = p.add(key, value, 10);
        match result {
            Ok(()) => panic!("Add key should return error"),
            Err(BMemcachedError::Status(Status::KeyExists)) => {},
            Err(_) => panic!("Some strange error that should not happen")
        };
        p.delete(key).unwrap();
    }

    #[test]
    fn get_key() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello";
        let value = "World";
        p.set(key, value, 10000).unwrap();
        assert_eq!(p.get(key).unwrap(),  value);
        match p.get("not found".to_string()) {
            Ok(_) => panic!("This key should not exist"),
            Err(BMemcachedError::Status(Status::KeyNotFound)) => {},
            Err(_) => panic!("This should return KeyNotFound")
        };
        p.delete(key).unwrap();
    }

    #[test]
    fn delete_key() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello";
        let value = "World";
        p.set(key, value, 1000).unwrap();
        p.delete(key).unwrap();
        p.delete(key).unwrap();
    }
}
