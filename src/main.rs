#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod http;
mod server;

use http::Method;
use server::Server;

fn main() {
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);
    println!("========== Server is running on {} ==========", server.addr);
    server.run();
}
