#[macro_use] extern crate bitflags;
extern crate byteorder;
extern crate conhash;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate log;
extern crate num;

mod client;
pub mod errors;
mod protocol;

pub use protocol::{
    FromMemcached,
    Status,
    StoredType,
    ToMemcached
};
pub use client::MemcachedClient;
