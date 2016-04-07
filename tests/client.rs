extern crate env_logger;
#[macro_use] extern crate log;
extern crate bmemcached;

use std::sync::Arc;
use std::thread;

use bmemcached::client::MemcachedClient;

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
