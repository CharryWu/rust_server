mod http;
mod server;

use http::Method;
use server::Server;

fn main() {
    let get = Method::GET;
    let post = Method::POST;
    let put = Method::PUT;
    let delete = Method::DELETE;
    let head = Method::HEAD;
    let connect = Method::CONNECT;
    let options = Method::OPTIONS;
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..14];
    let string_borrow: &str = &string;
    dbg!(&string);
    let server = Server::new(string);
    println!("========== Server is running on {} ==========", server.addr);
    server.run();
}
