use std::io::{Cursor, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
use num::FromPrimitive;

use errors::BMemcachedError;

enum Type {
    Request = 0x80,
    Response = 0x81,
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

bitflags! {
    pub flags StoredType: u32 {
        const MTYPE_STRING          = 1 << 0,
        const MTYPE_U8              = 1 << 1,
        const MTYPE_U16             = 1 << 2,
        const MTYPE_U32             = 1 << 3,
        const MTYPE_U64             = 1 << 4,
        #[allow(dead_code)]
        const MTYPE_VECTOR          = 1 << 5,
        #[allow(dead_code)]
        const MTYPE_COMPRESSED      = 1 << 6,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_1  = 1 << 10,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_2  = 1 << 11,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_3  = 1 << 13,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_4  = 1 << 14,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_5  = 1 << 15,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_6  = 1 << 16,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_7  = 1 << 17,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_8  = 1 << 18,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_9  = 1 << 19,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_10 = 1 << 20,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_11 = 1 << 21,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_12 = 1 << 22,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_13 = 1 << 23,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_14 = 1 << 24,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_15 = 1 << 25,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_16 = 1 << 26,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_17 = 1 << 27,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_18 = 1 << 28,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_19 = 1 << 29,
        #[allow(dead_code)]
        const MTYPE_USER_DEFINED_20 = 1 << 30
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
    cas: u64,
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
    cas: u64,
}

#[derive(Debug)]
pub struct Protocol {
    connection: TcpStream,
}

pub trait ToMemcached {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType), BMemcachedError>;
}

pub trait FromMemcached: Sized {
    fn get_value(flags: StoredType, buf: Vec<u8>) -> Result<Self, BMemcachedError>;
}

impl Protocol {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Protocol, BMemcachedError> {
        Ok(Protocol { connection: TcpStream::connect(addr)? })
    }

    pub fn connection_info(&self) -> String {
        self.connection.peer_addr().unwrap().to_string()
    }

    fn build_request(
        command: Command,
        key_length: usize,
        value_length: usize,
        data_type: u8,
        extras_length: usize,
        cas: u64,
    ) -> Request {
        Request {
            magic: Type::Request as u8,
            opcode: command as u8,
            key_length: key_length as u16,
            extras_length: extras_length as u8,
            data_type: data_type,
            reserved: 0,
            body_length: (key_length + value_length + extras_length) as u32,
            opaque: 0,
            cas: cas,
        }
    }

    fn write_request(&self, request: Request, final_payload: &[u8]) -> Result<(), BMemcachedError> {
        let mut buf = &self.connection;
        buf.write_u8(request.magic)?;
        buf.write_u8(request.opcode)?;
        buf.write_u16::<BigEndian>(request.key_length)?;
        buf.write_u8(request.extras_length)?;
        buf.write_u8(request.data_type)?;
        buf.write_u16::<BigEndian>(request.reserved)?;
        buf.write_u32::<BigEndian>(request.body_length)?;
        buf.write_u32::<BigEndian>(request.opaque)?;
        buf.write_u64::<BigEndian>(request.cas)?;
        buf.write(final_payload)?;
        Ok(())
    }

    fn read_response(&mut self) -> Result<Response, BMemcachedError> {
        let mut buf = &self.connection;
        let magic: u8 = buf.read_u8()?;
        if magic != Type::Response as u8 {
            // TODO Consume the stream, disconnect or something?
            debug!("Server sent an unknown magic code {:?}", magic);
            return Err(BMemcachedError::UnkownError(
                "Server sent an unknown magic code",
            ));
        }
        Ok(Response {
            magic: magic,
            opcode: buf.read_u8()?,
            key_length: buf.read_u16::<BigEndian>()?,
            extras_length: buf.read_u8()?,
            data_type: buf.read_u8()?,
            status: buf.read_u16::<BigEndian>()?,
            body_length: buf.read_u32::<BigEndian>()?,
            opaque: buf.read_u32::<BigEndian>()?,
            cas: buf.read_u64::<BigEndian>()?,
        })
    }

    fn consume_body(&mut self, size: u32) -> Result<(), BMemcachedError> {
        debug!("Consuming body");
        let mut buf: Vec<u8> = vec![0; size as usize];
        self.connection.read(&mut *buf)?;
        let str_buf = String::from_utf8(buf)?;
        debug!("Consumed body {:?}", str_buf);
        Ok(())
    }

    fn set_add_replace<K, V>(
        &mut self,
        command: Command,
        key: K,
        value: V,
        time: u32,
    ) -> Result<(), BMemcachedError>
    where
        K: AsRef<[u8]>,
        V: ToMemcached,
    {
        let key = key.as_ref();
        let (value, flags) = value.get_value()?;

        let extras_length = 8; // Flags: u32 and Expiration time: u32
        let request =
            Protocol::build_request(command, key.len(), value.len(), 0x00, extras_length, 0x00);
        let mut final_payload = vec![];
        // Flags
        final_payload.write_u32::<BigEndian>(flags.bits)?;
        final_payload.write_u32::<BigEndian>(time)?;
        // After flags key and value
        final_payload.write(key)?;
        final_payload.write(&value)?;
        self.write_request(request, final_payload.as_slice())?;
        let response = self.read_response()?;
        match Status::from_u16(response.status) {
            Some(Status::Success) => Ok(()),
            Some(rest) => {
                self.consume_body(response.body_length)?;
                Err(BMemcachedError::Status(rest))
            }
            None => Err(BMemcachedError::UnkownError(
                "Server returned an unknown status code",
            )),
        }
    }

    pub fn set<K, V>(&mut self, key: K, value: V, time: u32) -> Result<(), BMemcachedError>
    where
        K: AsRef<[u8]>,
        V: ToMemcached,
    {
        self.set_add_replace(Command::Set, key, value, time)
    }

    pub fn add<K, V>(&mut self, key: K, value: V, time: u32) -> Result<(), BMemcachedError>
    where
        K: AsRef<[u8]>,
        V: ToMemcached,
    {
        self.set_add_replace(Command::Add, key, value, time)
    }

    pub fn replace<K, V>(&mut self, key: K, value: V, time: u32) -> Result<(), BMemcachedError>
    where
        K: AsRef<[u8]>,
        V: ToMemcached,
    {
        self.set_add_replace(Command::Replace, key, value, time)
    }

    pub fn get<K, V>(&mut self, key: K) -> Result<V, BMemcachedError>
    where
        K: AsRef<[u8]>,
        V: FromMemcached,
    {
        let key = key.as_ref();
        let request = Protocol::build_request(Command::Get, key.len(), 0 as usize, 0, 0, 0x00);
        self.write_request(request, key)?;
        let response = self.read_response()?;
        match Status::from_u16(response.status) {
            Some(Status::Success) => {}
            Some(status) => {
                self.consume_body(response.body_length)?;
                return Err(BMemcachedError::Status(status));
            }
            None => {
                return Err(BMemcachedError::UnkownError(
                    "Server sent an unknown status code",
                ))
            }
        };
        let flags = StoredType::from_bits(self.connection.read_u32::<BigEndian>()?).unwrap();
        let mut outbuf = vec![0; (response.body_length - response.extras_length as u32) as usize];
        self.connection.read_exact(&mut outbuf)?;
        FromMemcached::get_value(flags, outbuf)
    }

    pub fn delete<K>(&mut self, key: K) -> Result<(), BMemcachedError>
    where
        K: AsRef<[u8]>,
    {
        let key = key.as_ref();
        let request = Protocol::build_request(Command::Delete, key.len(), 0 as usize, 0, 0, 0x00);
        self.write_request(request, key)?;
        let response = self.read_response()?;

        match Status::from_u16(response.status) {
            Some(Status::Success) => Ok(()),
            Some(Status::KeyNotFound) => {
                self.consume_body(response.body_length)?;
                Ok(())
            }
            Some(status) => {
                self.consume_body(response.body_length)?;
                Err(BMemcachedError::Status(status))
            }
            None => Err(BMemcachedError::UnkownError(
                "Server sent an unknown status code",
            )),
        }
    }

    fn increment_decrement<K>(
        &mut self,
        key: K,
        amount: u64,
        initial: u64,
        time: u32,
        command: Command,
    ) -> Result<u64, BMemcachedError>
    where
        K: AsRef<[u8]>,
    {
        let key = key.as_ref();
        let extras_length = 20; // Amount: u64, Initial: u64, Time: u32
        let request = Protocol::build_request(command, key.len(), 0, 0, extras_length, 0x00);
        let mut final_payload: Vec<u8> = vec![];
        final_payload.write_u64::<BigEndian>(amount)?;
        final_payload.write_u64::<BigEndian>(initial)?;
        final_payload.write_u32::<BigEndian>(time)?;
        final_payload.write(key)?;
        self.write_request(request, &final_payload)?;
        let response = self.read_response()?;
        match Status::from_u16(response.status) {
            Some(Status::Success) => Ok(self.connection.read_u64::<BigEndian>()?),
            Some(status) => {
                self.consume_body(response.body_length)?;
                Err(BMemcachedError::Status(status))
            }
            None => Err(BMemcachedError::UnkownError(
                "Server sent an unknown status code",
            )),
        }
    }

    pub fn increment<K>(
        &mut self,
        key: K,
        amount: u64,
        initial: u64,
        time: u32,
    ) -> Result<u64, BMemcachedError>
    where
        K: AsRef<[u8]>,
    {
        self.increment_decrement(key, amount, initial, time, Command::Increment)
    }

    pub fn decrement<K>(
        &mut self,
        key: K,
        amount: u64,
        initial: u64,
        time: u32,
    ) -> Result<u64, BMemcachedError>
    where
        K: AsRef<[u8]>,
    {
        self.increment_decrement(key, amount, initial, time, Command::Decrement)
    }
}

impl ToMemcached for u8 {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType), BMemcachedError> {
        Ok((vec![*self], MTYPE_U8))
    }
}

impl ToMemcached for u16 {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType), BMemcachedError> {
        let mut buf = vec![];
        buf.write_u16::<BigEndian>(*self)?;
        Ok((buf, MTYPE_U16))
    }
}

impl ToMemcached for u32 {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType), BMemcachedError> {
        let mut buf = vec![];
        buf.write_u32::<BigEndian>(*self)?;
        Ok((buf, MTYPE_U32))
    }
}

impl ToMemcached for u64 {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType), BMemcachedError> {
        let mut buf = vec![];
        buf.write_u64::<BigEndian>(*self)?;
        Ok((buf, MTYPE_U64))
    }
}

impl<'a> ToMemcached for &'a String {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType), BMemcachedError> {
        let v = *self;
        Ok((v.clone().into_bytes(), MTYPE_STRING))
    }
}

impl<'a> ToMemcached for &'a str {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType), BMemcachedError> {
        Ok((self.as_bytes().to_vec(), MTYPE_STRING))
    }
}

impl FromMemcached for String {
    fn get_value(flags: StoredType, buf: Vec<u8>) -> Result<Self, BMemcachedError> {
        if flags & MTYPE_STRING != StoredType::empty() {
            Ok(String::from_utf8(buf)?)
        } else {
            Err(BMemcachedError::TypeMismatch(flags))
        }
    }
}

impl FromMemcached for u8 {
    fn get_value(flags: StoredType, buf: Vec<u8>) -> Result<Self, BMemcachedError> {
        if flags & MTYPE_U8 != StoredType::empty() {
            let mut buf = Cursor::new(buf);
            Ok(buf.read_u8()?)
        } else {
            Err(BMemcachedError::TypeMismatch(flags))
        }
    }
}

impl FromMemcached for u16 {
    fn get_value(flags: StoredType, buf: Vec<u8>) -> Result<Self, BMemcachedError> {
        if flags & MTYPE_U16 != StoredType::empty() {
            let mut buf = Cursor::new(buf);
            Ok(buf.read_u16::<BigEndian>()?)
        } else {
            Err(BMemcachedError::TypeMismatch(flags))
        }
    }
}

impl FromMemcached for u32 {
    fn get_value(flags: StoredType, buf: Vec<u8>) -> Result<Self, BMemcachedError> {
        if flags & MTYPE_U32 != StoredType::empty() {
            let mut buf = Cursor::new(buf);
            Ok(buf.read_u32::<BigEndian>()?)
        } else {
            Err(BMemcachedError::TypeMismatch(flags))
        }
    }
}

impl FromMemcached for u64 {
    #[allow(unused_variables)]
    fn get_value(flags: StoredType, buf: Vec<u8>) -> Result<Self, BMemcachedError> {
        // As increment and decrement don't allow us to send flags, we don't
        // enforce type checking.
        let mut buf = Cursor::new(buf);
        Ok(buf.read_u64::<BigEndian>()?)
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
        let key = "Hello Set";
        let value = "World";
        p.set(key, value, 1000).unwrap();
        p.delete(key).unwrap();
    }

    #[test]
    fn set_key_u8() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello";
        let value = 1 as u8;
        p.set(key, value, 1000).unwrap();
        p.delete(key).unwrap();
    }

    #[test]
    fn set_key_u16() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello";
        let value = 1 as u16;
        p.set(key, value, 1000).unwrap();
        p.delete(key).unwrap();
    }

    #[test]
    fn set_key_u32() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello";
        let value = 1 as u32;
        p.set(key, value, 100).unwrap();
        p.delete(key).unwrap();
    }

    #[test]
    fn set_key_u64() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello";
        let value = 1 as u64;
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
            Err(BMemcachedError::Status(Status::KeyExists)) => {}
            Err(_) => panic!("Some strange error that should not happen"),
        };
        p.delete(key).unwrap();
    }

    #[test]
    fn get_key() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello Get";
        let value = "World";
        p.set(key, value, 10000).unwrap();
        let rv: String = p.get(key).unwrap();
        assert_eq!(rv, value);

        let not_found: Result<String, BMemcachedError> = p.get("not found".to_string());
        match not_found {
            Ok(_) => panic!("This key should not exist"),
            Err(BMemcachedError::Status(Status::KeyNotFound)) => {}
            Err(_) => panic!("This should return KeyNotFound"),
        };
        p.delete(key).unwrap();
    }

    #[test]
    fn delete_key() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello Delete";
        let value = "World";
        p.set(key, value, 1000).unwrap();
        p.delete(key).unwrap();
        p.delete(key).unwrap();
    }

    #[test]
    fn increment() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello Increment";
        assert_eq!(p.increment(key, 1, 0, 1000).unwrap(), 0);
        assert_eq!(p.increment(key, 1, 0, 1000).unwrap(), 1);
        assert_eq!(p.increment(key, 1, 0, 1000).unwrap(), 2);
        p.delete(key).unwrap();
    }

    #[test]
    fn decrement() {
        let _ = env_logger::init();
        let mut p = Protocol::connect("127.0.0.1:11211").unwrap();
        let key = "Hello Decrement";
        assert_eq!(p.decrement(key, 1, 0, 1000).unwrap(), 0);
        assert_eq!(p.decrement(key, 1, 0, 1000).unwrap(), 0);
        assert_eq!(p.increment(key, 1, 0, 1000).unwrap(), 1);
        assert_eq!(p.increment(key, 1, 0, 1000).unwrap(), 2);
        assert_eq!(p.decrement(key, 1, 0, 1000).unwrap(), 1);
        p.delete(key).unwrap();
    }
}
