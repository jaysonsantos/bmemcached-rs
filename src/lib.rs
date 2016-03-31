#[allow(dead_code)]
#[macro_use] extern crate enum_primitive;
extern crate byteorder;
extern crate num;

pub mod errors;
mod protocol;

// TODO: The idea is to implement some sort of a front-end to Protocol to be able to do some consistent hashing to distribute data between servers.
