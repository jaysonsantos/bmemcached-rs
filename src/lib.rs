extern crate byteorder;
mod protocol;
use std::io::Write;
use std::net::{
    ToSocketAddrs
};

pub struct BMemcached {
    protocol: protocol::Protocol
}

impl BMemcached {
    fn new<A: ToSocketAddrs>(addr: A) -> BMemcached {
        BMemcached{protocol: protocol::Protocol::connect(addr)}
    }
}
