/// Server module for handling incoming TCP connections and parsing HTTP requests.
///
/// This module defines the `Server` struct, which binds to a specified address,
/// listens for incoming TCP connections, reads HTTP requests from clients, and
/// attempts to parse them using the `Request` struct from the `http` module.
///
/// # Example
///
/// ```
/// let server = Server::new("127.0.0.1:8080".to_string());
/// server.run();
/// ```
use crate::http::Request; // Import the Request struct from the http module
use std::io::Read; // For reading from the TCP stream
use std::net::TcpListener; // For listening to TCP connections

#[derive(Debug)]
pub struct Server {
    pub addr: String, // Address to bind the server to (e.g., "127.0.0.1:8080")
}
impl Server {
    /// Creates a new Server instance with the given address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to bind the server to (e.g., "127.0.0.1:8080").
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    /// Runs the server, listening for incoming TCP connections and handling requests.
    pub fn run(&self) {
        println!("Listening on {}", self.addr);
        // Bind the TCP listener to the specified address
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            // Accept an incoming connection
            match listener.accept() {
                Ok((mut sock_stream, addr)) => {
                    let mut buffer = [0; 1024]; // Buffer to store incoming request data
                    // Read data from the TCP stream into the buffer
                    match sock_stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                // If no bytes were read, the client disconnected
                                println!("========== Client disconnected ==========");
                                break;
                            }
                            // Print the raw HTTP request received
                            println!(
                                "========== Received a request ==========\n{}",
                                String::from_utf8_lossy(&buffer[..bytes_read])
                            );
                            // Attempt to parse the HTTP request from the buffer
                            match Request::try_from(&buffer[..bytes_read]) {
                                Ok(request) => {
                                    dbg!(request); // Debug print the parsed request
                                    // println!("========== Request ==========\n{}", request);
                                }
                                Err(e) => {
                                    // Print any parsing errors
                                    println!("========== Error ==========\n{}", e);
                                }
                            }
                            // 2 ways to convert between Request and &[u8] using TryFrom and TryInto:
                            // Request::try_from(&buffer[..bytes_read]);
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("========== Error ==========\n{}", e), // Error reading from stream
                    }
                }
                Err(e) => {
                    // Error accepting a new connection
                    println!("========== Error ==========\n{}", e);
                }
            }
        }
    }
}
