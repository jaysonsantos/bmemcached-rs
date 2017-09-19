use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};

use conhash::{ConsistentHash, Node};

use errors::Result;
use protocol;

#[derive(Debug, Clone)]
struct ClonableProtocol {
    connection: Arc<Mutex<protocol::Protocol>>,
}

impl Node for ClonableProtocol {
    fn name(&self) -> String {
        let protocol = self.clone();
        let connection = protocol.connection.lock().unwrap();
        connection.connection_info()
    }
}

/// Struct that holds all connections and proxy commands to the right server based on the key
pub struct MemcachedClient {
    connections: ConsistentHash<ClonableProtocol>,
}

impl MemcachedClient {
    pub fn new<A: ToSocketAddrs>(
        addrs: Vec<A>,
        connections_per_addr: u8,
    ) -> Result<MemcachedClient> {
        let mut ch = ConsistentHash::new();
        for addr in &addrs {
            for _ in 0..connections_per_addr {
                let protocol = protocol::Protocol::connect(addr)?;
                ch.add(
                    &ClonableProtocol { connection: Arc::new(Mutex::new(protocol)) },
                    1,
                );
            }
        }
        Ok(MemcachedClient { connections: ch })
    }

    pub fn set<K, V>(&self, key: K, value: V, time: u32) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: protocol::ToMemcached,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.set(key, value, time)
    }

    pub fn add<K, V>(&self, key: K, value: V, time: u32) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: protocol::ToMemcached,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.add(key, value, time)
    }

    pub fn replace<K, V>(&self, key: K, value: V, time: u32) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: protocol::ToMemcached,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.replace(key, value, time)
    }

    pub fn get<K, V>(&self, key: K) -> Result<V>
    where
        K: AsRef<[u8]>,
        V: protocol::FromMemcached,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.get(key)
    }

    pub fn delete<K>(&self, key: K) -> Result<()>
    where
        K: AsRef<[u8]>,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.delete(key)
    }

    pub fn increment<K>(
        &self,
        key: K,
        amount: u64,
        initial: u64,
        time: u32,
    ) -> Result<u64>
    where
        K: AsRef<[u8]>,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.increment(key, amount, initial, time)
    }

    pub fn decrement<K>(
        &self,
        key: K,
        amount: u64,
        initial: u64,
        time: u32,
    ) -> Result<u64>
    where
        K: AsRef<[u8]>,
    {
        let clonable_protocol = self.connections.get(key.as_ref()).unwrap();
        let mut protocol = clonable_protocol.connection.lock().unwrap();
        protocol.decrement(key, amount, initial, time)
    }
}
