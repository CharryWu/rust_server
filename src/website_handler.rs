use super::server::Handler;
use crate::http::{Method, Request, Response, StatusCode};
use std::fs;

pub struct WebsiteHandler {
    public_path: String, // path to the public directory
}

impl WebsiteHandler {
    /// Creates a new `WebsiteHandler` with the given public path.
    ///
    /// # Arguments
    ///
    /// * `public_path` - The path to the public directory. Usually comes from the command line arguments.
    ///
    /// # Returns
    ///
    /// A `WebsiteHandler` object.
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    /// Reads a file from the public directory and returns its contents as a string.
    /// This method provides a basic level of security by checking for directory traversal attacks.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file to read.
    ///
    /// # Returns
    ///
    /// A `String` containing the contents of the file if it exists, otherwise `None`.
    fn read_file(&self, file_path: &str) -> Option<String> {
        let raw_path = format!("{}/{}", self.public_path, file_path);
        // For security reasons, we need to check if the requested path is under the public path
        match fs::canonicalize(&raw_path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    // ok() returns an Option<String>
                    // if the file is not found, ok() returns None
                    // if the file is found, ok() returns Some(String)
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(e) => {
                println!("Error resolving path: {}, error: {}", &raw_path, e);
                None
            }
        }
    }
}

impl Handler for WebsiteHandler {
    /// Handles HTTP requests for the website.
    ///
    /// # Note
    ///
    /// This method is called by the `Server` when a request is received.
    /// It handles GET requests for the root path ("/") and returns a default body.
    /// For other paths, it returns a 404 Not Found response.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to the HTTP request.
    ///
    /// # Returns
    ///
    /// A `Response` object containing the HTTP response.
    fn handle_request(&mut self, request: &Request) -> Response {
        // Use double nested match to handle both the method and the path
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    // if the file exists, read the contents and return a 200 OK response
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    // if the file does not exist, return a 404 Not Found response
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
