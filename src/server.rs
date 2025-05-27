use crate::http::Request; // import sibling module need to use `crate::` keyword
use std::io::Read;
use std::net::TcpListener;

#[derive(Debug)]
pub struct Server {
    pub addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.addr);
        // TcpListener::bind accepts a string slice of the address to listen on
        // TcpListener::bind("127.0.0.1:8080")
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut sock_stream, addr)) => {
                    let mut buffer = [0; 1024]; // 1024 bytes is enough for this exercise
                    match sock_stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                println!("========== Client disconnected ==========");
                                break;
                            }
                            println!(
                                "========== Received a request ==========\n{}",
                                String::from_utf8_lossy(&buffer[..bytes_read])
                            );
                            match Request::try_from(&buffer[..bytes_read]) {
                                Ok(request) => {
                                    todo!()
                                    // println!("========== Request ==========\n{}", request);
                                }
                                Err(e) => {
                                    println!("========== Error ==========\n{}", e);
                                }
                            }
                            // 2 ways to convert between Request and &[u8] using TryFrom and TryInto:
                            // Request::try_from(&buffer[..bytes_read]);
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("========== Error ==========\n{}", e),
                    }
                }
                Err(e) => {
                    println!("========== Error ==========\n{}", e);
                }
            }
        }
    }
}
