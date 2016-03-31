use std::io;
use std::string;

use protocol::Status;

#[derive(Debug)]
pub enum BMemcachedError {
    IoError(io::Error),
    Utf8Error(string::FromUtf8Error),
    UnkownError(&'static str),
    Status(Status)
}

impl From<io::Error> for BMemcachedError {
    fn from(err: io::Error) -> BMemcachedError {
        BMemcachedError::IoError(err)
    }
}

impl From<string::FromUtf8Error> for BMemcachedError {
    fn from(err: string::FromUtf8Error) -> BMemcachedError {
        BMemcachedError::Utf8Error(err)
    }
}
