use std::io;
use std::string;

use protocol::{Status, StoredType};

#[derive(Debug)]
pub enum BMemcachedError {
    IoError(io::Error),
    Utf8Error(string::FromUtf8Error),
    UnkownError(&'static str),
    Status(Status),
    /// In case you tried to coerse to a value that does not match with the stored.
    /// The returned flags are inside the error.
    TypeMismatch(StoredType),
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
