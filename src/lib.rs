/*!
This is a binary memcached protocol implemented only in rust with support of traits to send
and receive `T` and consistent hashing to select connections from a pool to distribute data.
# Example
```rust
extern crate bmemcached;

use std::sync::Arc;
use std::thread;

use bmemcached::MemcachedClient;

fn main() {
    // Use arc for threading support
    let client = Arc::new(MemcachedClient::new(vec!["127.0.0.1:11211"], 5).unwrap());

    // Traits examples
    let value = "value";
    client.set("string", value, 1000);
    let rv: String = client.get("string").unwrap();
    assert_eq!(rv, "value");

    client.set("integer", 10 as u8, 1000);
    let rv: u8 = client.get("integer").unwrap();
    assert_eq!(rv, 10 as u8);

    // Threads example
    let mut threads = vec![];
    for i in 0..4 {
        let client = client.clone();
        threads.push(thread::spawn(move || {
            let data = format!("data_n{}", i);
            client.set(&data, &data, 100).unwrap();
            let val: String = client.get(&data).unwrap();
            client.delete(&data).unwrap();
            val
        }));
    }
    for (i, thread) in threads.into_iter().enumerate() {
        let result = thread.join();
        assert_eq!(result.unwrap(), format!("data_n{}", i));
    }
}
```
*/
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate conhash;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate log;
extern crate num;

mod client;
pub mod errors;
mod protocol;

pub use protocol::{FromMemcached, Status, StoredType, ToMemcached};
pub use client::MemcachedClient;
