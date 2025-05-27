use super::method::Method;
use std::convert::TryFrom; // convert::From doesn't handle errors, convert::TryFrom handles errors
pub struct Request {
    method: Method,
    query_string: Option<String>,
    path: String,
}
impl Request {
    // don't need to implement convert method on own own, just use std::convert::TryFrom in idiomatic Rust (see below)
    // fn from_byte_array(byte_array: &[u8]) -> Result<Self, String>
}

// TypeFrom is generic, so we have assign a concrete type to it, in this project it's &[u8]
// type parameter is needed because the compiler has to know what type the result will
// contain as the error type
impl TryFrom<&[u8]> for Request {
    // need to assign `Error` type and implement `try_from` method
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
