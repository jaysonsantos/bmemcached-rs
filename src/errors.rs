use std::io;

#[derive(Debug)]
pub enum BMemcachedError {
    IoError(io::Error)
}

impl From<io::Error> for BMemcachedError {
    fn from(err: io::Error) -> BMemcachedError {
        BMemcachedError::IoError(err)
    }
}
