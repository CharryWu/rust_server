use super::StatusCode;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Result as IoResult, Write},
};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    /// Sends the response to the client, without creating a new string.
    /// This method invokes write! macro directly passing its internal data, instead of returning a copy of the response string.
    ///
    /// # Note
    ///
    /// This method is more efficient than using `Display` because it avoids the overhead of creating a new string.
    /// Especially when the response body is large (e.g. HTML pages with megabytes size of HTML content).
    ///
    /// # Example
    ///
    /// ```
    /// let response = Response::new(StatusCode::Ok, Some("Hello".to_string()));
    /// response.send(&mut stream);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `stream` - The stream to write the response to.
    ///
    /// # Returns
    /// * `IoResult<()>` - The result of the write operation.
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = self.body.as_deref().unwrap_or("");
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

// DEPRECATED: use `Response::send` instead
// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         let body = self.body.as_deref().unwrap_or("");
//         write!(
//             f,
//             "HTTP/1.1 {} {}\r\n\r\n{}",
//             self.status_code,
//             self.status_code.reason_phrase(),
//             body
//         )
//     }
// }
