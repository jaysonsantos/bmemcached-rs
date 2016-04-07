extern crate env_logger;
#[macro_use] extern crate log;
extern crate bmemcached;

use std::sync::Arc;
use std::thread;

use bmemcached::{
    MemcachedClient,
    Status
};
use bmemcached::errors::BMemcachedError;

#[test]
fn multiple_threads() {
    let _ = env_logger::init();
    let mut threads = vec![];
    let client = Arc::new(MemcachedClient::new(vec!["127.0.0.1:11211"], 5).unwrap());
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

#[test]
fn get_set_delete() {
    let _ = env_logger::init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).unwrap();
    let key = "Hello Get, Set, Delete Client";
    let value = "World";
    client.set(key, value, 1000).unwrap();
    assert_eq!(client.get(key).unwrap(), value);
    client.delete(key).unwrap();
}

#[test]
fn add() {
    let _ = env_logger::init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).unwrap();
    let key = "Hello Add Client";
    let value = "World";
    client.add(key, value, 1000).unwrap();
    assert_eq!(client.get(key).unwrap(), value);
    match client.add(key, value, 1000) {
        Err(BMemcachedError::Status(Status::KeyExists)) => (),
        e => panic!("Wrong status returned {:?}", e)
    }
    client.delete(key).unwrap();
}

#[test]
fn replace() {
    let _ = env_logger::init();
    let client = MemcachedClient::new(vec!["127.0.0.1:11211"], 1).unwrap();
    let key = "Hello Replace Client";
    let value = "World";
    client.add(key, value, 1000).unwrap();
    assert_eq!(client.get(key).unwrap(), value);
    client.replace(key, "New value", 100).unwrap();
    assert_eq!(client.get(key).unwrap(), "New value");
    client.delete(key).unwrap();
}
