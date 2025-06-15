#![allow(dead_code)]
mod http;
mod server;
mod website_handler;
use server::Server;
use std::env;
use std::path::{Path, absolute};
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // you can also ues String::from("./public") as default value
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let host = env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let server = Server::new(format!("{}:{}", host, port));
    println!("================================================");
    println!("Server is running on http://{}", server.addr);
    println!(
        "Serving files from public path: {}",
        absolute(Path::new(&public_path)).unwrap().display()
    );
    println!("================================================");
    server.run(WebsiteHandler::new(public_path));
}
