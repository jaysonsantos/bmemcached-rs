# bmemcached-rs [![Build Status](https://travis-ci.org/jaysonsantos/bmemcached-rs.svg?branch=master)](https://travis-ci.org/jaysonsantos/bmemcached-rs) [![Clippy Linting Result](https://clippy.bashy.io/github/jaysonsantos/bmemcached-rs/master/badge.svg)](https://clippy.bashy.io/github/jaysonsantos/bmemcached-rs/master/log)
Rust binary memcached implementation (ON GOING)

# Usage
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

# Why
I am trying to learn rust by reimplementing a python project that I wrote.

# What works
* Add
* Set
* Replace
* Get
* Delete
* Increment
* Decrement
* Consistent Hashing
* Threading Support

## Trait usage
On all supported functions we use traits to be able to send any type of values to memcached.
