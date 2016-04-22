# bmemcached-rs [![Build Status](https://travis-ci.org/jaysonsantos/bmemcached-rs.svg?branch=master)](https://travis-ci.org/jaysonsantos/bmemcached-rs)
Rust binary memcached implementation (ON GOING)

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
