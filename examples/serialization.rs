/*!
This is an example of how to make bmemcached save your own types and it uses serde for serializing
and deserializing.
*/
extern crate bmemcached;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use bmemcached::{ToMemcached, FromMemcached, StoredType};
use bmemcached::errors::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    age: u8,
    registered: bool
}

impl<'a> ToMemcached for &'a Data {
    fn get_value(&self) -> Result<(Vec<u8>, StoredType)> {
        Ok((serde_json::to_vec(self).unwrap(), StoredType::MTYPE_USER_DEFINED_1))
    }
}

impl FromMemcached for Data {
    fn get_value(flags: StoredType, buf: Vec<u8>) -> Result<Self> {
        assert!(flags == StoredType::MTYPE_USER_DEFINED_1);
        Ok(serde_json::from_slice(&*buf).unwrap())
    }
}

fn main() {
    let data = Data { name: "Testing".to_owned(), age: 8, registered: false };
    let memcached = bmemcached::MemcachedClient::new(
        vec!["127.0.0.1:11211"], 5).unwrap();
    println!("Storing {:?}", data);
    memcached.set("testing", &data, 10000).unwrap();
    let rv: Vec<u8> = memcached.get("testing").unwrap();
    let string = String::from_utf8(rv).unwrap();
    println!("Raw data {:?}", string);
    let rv: Data = memcached.get("testing").unwrap();
    println!("Parsed data {:?}", rv);
}
