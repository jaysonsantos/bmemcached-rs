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
    fn new<A: ToSocketAddrs>(addrs: Vec<A>) -> Result<MemcachedClient, errors::BMemcachedError> {
        let mut ch = ConsistentHash::new();
        for addr in addrs.iter() {
            let protocol = try!(protocol::Protocol::connect(addr));
            ch.add(&ClonableProtocol {connection: Arc::new(Mutex::new(protocol))}, 1);
        }
        Ok(MemcachedClient {connections: ch})
    }

    fn set<K, V>(&self, key: K, value: V, time: u32) -> Result<(), errors::BMemcachedError>
        where K: AsRef<[u8]>, V: AsRef<[u8]> {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.set(key, value, time)
    }

    fn get<K>(&self, key: K) -> Result<String, errors::BMemcachedError> where K: AsRef<[u8]> {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.get(key)
    }

    fn delete<K>(&self, key: K) -> Result<(), errors::BMemcachedError> where K: AsRef<[u8]> {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.delete(key)
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    use std::sync::Arc;
    use std::thread;
    use super::*;

    #[test]
    fn test_multiple_threads() {
        let _ = env_logger::init();
        let mut threads = vec![];
        let client = Arc::new(MemcachedClient::new(vec!["127.0.0.1:11211", "127.0.0.1:11211", "127.0.0.1:11211", "127.0.0.1:11211"]).unwrap());
        for i in 0..4 {
            let client = client.clone();
            debug!("Starting thread {}", i);
            threads.push(thread::spawn(move || {
                debug!("Started {}", i);
                let data = format!("data_n{}", i);
                client.set(&data, &data, 100).unwrap();
                let val = client.get(&data).unwrap();
                client.delete(&data).unwrap();
                debug!("Finished {}", i);
                val
            }));
        }
        for (i, thread) in threads.into_iter().enumerate() {
            let result = thread.join();
            assert_eq!(result.unwrap(), format!("data_n{}", i));
        }
    }
}
