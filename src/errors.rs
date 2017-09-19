use constants::StoredType;
use protocol::Status;

error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        Utf8Error(::std::string::FromUtf8Error);
    }

    errors {
        Status(s: Status) {
            description("Invalid status received")
            display("Invalid status received {:?}", s)
        }
        /// In case you tried to coerce to a value that does not match with the stored.
        /// The returned flags are inside the error.
        TypeMismatch(s: StoredType) {
            description("Requested type is different from the one stored in memcached")
            display("Requested type is different from the one stored in memcached: {:?}", s)
        }
    }
}