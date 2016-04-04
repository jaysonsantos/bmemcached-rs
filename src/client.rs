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
        "".to_string()
    }
}

pub struct MemcachedClient {
    connections: ConsistentHash<ClonableProtocol>
}

unsafe impl Send for MemcachedClient {}

impl MemcachedClient {
    fn new<A: ToSocketAddrs>(addrs: Vec<A>) -> Result<MemcachedClient, errors::BMemcachedError> {
        let mut ch = ConsistentHash::new();
        for addr in addrs.iter() {
            let protocol = try!(protocol::Protocol::connect(addr));
            ch.add(&ClonableProtocol {connection: Arc::new(Mutex::new(protocol))}, 1);
        }
        Ok(MemcachedClient {connections: ch})
    }

    fn set<K, V>(&mut self, key: K, value: V, time: u32) -> Result<(), errors::BMemcachedError>
        where K: AsRef<[u8]>, V: AsRef<[u8]> {
        let mut clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut lock = clonable_protocol.connection.clone();
        let mut protocol = lock.lock().unwrap();
        protocol.set(key, value, time)
    }

    fn get<K>(&mut self, key: K) -> Result<String, errors::BMemcachedError> where K: AsRef<[u8]> {
        let mut clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut lock = clonable_protocol.connection.clone();
        let mut protocol = lock.lock().unwrap();
        protocol.get(key)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::thread;
    use super::*;

    #[test]
    fn test_abc() {
        let mut client = MemcachedClient::new(vec!["127.0.0.1:11211", "127.0.0.1:11211"]).unwrap();
        let mut threads = vec![];
        for i in 1..10 {
            threads.push(thread::spawn(move || {
                let data = format!("data_n{}", i);
                client.set(&data, &data, 100);
                client.get(&data).unwrap()
            }));
        }
        for i in 1..10 {
            let result = threads[i].join();
            assert_eq!(result.unwrap(), format!("data_n{}", i));
        }
    }
}
