use std::io;
use std::str;

#[derive(Debug)]
pub enum BMemcachedError {
    IoError(io::Error),
    Utf8Error(str::Utf8Error)
}

impl From<io::Error> for BMemcachedError {
    fn from(err: io::Error) -> BMemcachedError {
        BMemcachedError::IoError(err)
    }
}

impl From<str::Utf8Error> for BMemcachedError {
    fn from(err: str::Utf8Error) -> BMemcachedError {
        BMemcachedError::Utf8Error(err)
    }
}
