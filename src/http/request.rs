use super::method::{Method, MethodError}; // Import Method and MethodError from the method module
use super::{QueryString, Value};
use std::convert::TryFrom; // convert::From doesn't handle errors, convert::TryFrom handles errors
use std::error::Error; // Error trait is used for error handling in Rust
use std::fmt::{self, Debug, Display, Result as FmtResult};
use std::str;
use std::str::Utf8Error; // Utf8Error is used to handle errors when converting bytes to a string

/// Extracts the first word from a string, separated by spaces or carriage returns.
///
/// # Arguments
///
/// * `request` - A string slice containing the text to parse
///
/// # Returns
///
/// Returns `Some((word, rest))` where:
/// * `word` is the first word found before a space or carriage return
/// * `rest` is the remaining string after the delimiter
///
/// Returns `None` if no delimiter is found
///
/// # Examples
///
/// ```
/// let text = "GET /path HTTP/1.1";
/// let (word, rest) = get_next_word(text).unwrap();
/// assert_eq!(word, "GET");
/// assert_eq!(rest, "/path HTTP/1.1");
/// ```
fn get_next_word<'buf>(request: &'buf str) -> Option<(&'buf str, &'buf str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..])); // Adding 1 to skip the space character, however in non-utf-8 encoded strings this could cause issues
            // + 1 means adding one byte, not just adding one character
            // here is fine since space is exactly one byte in UTF-8
            // Return the word and the rest of the string; IMPORTANT: `return` statement is needed here
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_word_basic() {
        let input = "GET /path HTTP/1.1";
        let result = get_next_word(input);
        assert_eq!(result, Some(("GET", "/path HTTP/1.1")));
    }

    #[test]
    fn test_get_next_word_with_carriage_return() {
        let input = "GET\r/path HTTP/1.1";
        let result = get_next_word(input);
        assert_eq!(result, Some(("GET", "/path HTTP/1.1")));
    }

    #[test]
    fn test_get_next_word_no_delimiter() {
        let input = "GET";
        let result = get_next_word(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_next_word_empty_string() {
        let input = "";
        let result = get_next_word(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_next_word_multiple_spaces() {
        let input = "GET    /path HTTP/1.1";
        let result = get_next_word(input);
        assert_eq!(result, Some(("GET", "   /path HTTP/1.1")));
    }
}

#[derive(Debug)]
pub struct Request<'buf> {
    method: Method,
    query_string: Option<QueryString<'buf>>, // query string may or may not exist on URL
    path: &'buf str,
}
impl<'buf> Request<'buf> {
    // don't need to implement convert method on own own, just use std::convert::TryFrom in idiomatic Rust (see below)
    // NOT NEEDED: fn from_byte_array(byte_array: &[u8]) -> Result<Self, String>
}

// Convert &[u8] byte array into Request using TryFrom trait
// **Parsing logic of request headers goes here**
// Extract URL path, query param, and HTTP method & protocol into Request struct
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // need to assign `Error` type and implement `try_from` method
    type Error = ParseError;

    // Return a ParseError::InvalidEncoding if the request is not valid UTF-8
    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        let request = str::from_utf8(buffer)?; // '?' propagates the error to caller
        let (method, rest) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; // If get_next_word returns None, return ParseError::InvalidRequest
        let (mut path, rest) = get_next_word(rest).ok_or(ParseError::InvalidRequest)?; // If get_next_word returns None, return ParseError::InvalidRequest, rest is variable shadowing
        let (protocol, _) = get_next_word(rest).ok_or(ParseError::InvalidRequest)?; // If get_next_word returns None, return ParseError::InvalidRequest

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol); // If the protocol is not HTTP/1.1 or HTTP/2.0, return ParseError::InvalidProtocol
        }

        let method: Method = method.parse()?;
        let mut query_string: Option<QueryString<'buf>> = None;
        if let Some(i) = path.find('?') {
            // `if let` syntax allows you to only match on variants you care about
            path = &path[..i]; // Update path to exclude the query string
            query_string = Some(QueryString::from(&path[i + 1..])); // If the path contains a query string, extract it
        }

        Ok(Self {
            method,
            path,
            query_string,
        })
    }
}

pub enum ParseError {
    InvalidRequest,  // General error for invalid requests
    InvalidMethod,   // Error for unsupported HTTP methods, we only support GET and POST for now
    InvalidEncoding, // Error for non-utf-8 encoded requests
    InvalidProtocol, // Error for unsupported HTTP protocols, we only support HTTP/1.1 for now
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod // Convert MethodError to ParseError::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding // Convert Utf8Error to ParseError::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "The request is invalid",
            ParseError::InvalidMethod => "The method is not supported",
            ParseError::InvalidEncoding => "The request is not valid UTF-8",
            ParseError::InvalidProtocol => "The protocol is not supported",
        }
    }
}

impl Error for ParseError {}
