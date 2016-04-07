use std::net::ToSocketAddrs;
use std::sync::{
    Arc,
    Mutex
};

use conhash::{
    ConsistentHash,
    Node
};

use errors;
use protocol;

#[derive(Debug, Clone)]
struct ClonableProtocol {
    connection: Arc<Mutex<protocol::Protocol>>
}

impl Node for ClonableProtocol {
    fn name(&self) -> String {
        let protocol = self.clone();
        let connection = protocol.connection.lock().unwrap();
        connection.connection_info()
    }
}

pub struct MemcachedClient {
    connections: ConsistentHash<ClonableProtocol>
}

impl MemcachedClient {
    pub fn new<A: ToSocketAddrs>(addrs: Vec<A>) -> Result<MemcachedClient, errors::BMemcachedError> {
        let mut ch = ConsistentHash::new();
        for addr in addrs.iter() {
            let protocol = try!(protocol::Protocol::connect(addr));
            ch.add(&ClonableProtocol {connection: Arc::new(Mutex::new(protocol))}, 1);
        }
        Ok(MemcachedClient {connections: ch})
    }

    pub fn set<K, V>(&self, key: K, value: V, time: u32) -> Result<(), errors::BMemcachedError>
        where K: AsRef<[u8]>, V: AsRef<[u8]> {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.set(key, value, time)
    }

    pub fn get<K>(&self, key: K) -> Result<String, errors::BMemcachedError> where K: AsRef<[u8]> {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.get(key)
    }

    pub fn delete<K>(&self, key: K) -> Result<(), errors::BMemcachedError> where K: AsRef<[u8]> {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.delete(key)
    }
}
