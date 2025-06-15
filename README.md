# Rust Server

This project is part of a hands-on Udemy course designed to teach the fundamentals of the Rust programming language through real-world application. Instead of isolated code snippets, you'll build practical projects—including a working HTTP server from scratch—while learning Rust concepts as you need them.

## Course Link

[Udemy: Rust Fundamentals](https://www.udemy.com/course/rust-fundamentals)

## Project Overview

Rust is a modern systems programming language that is blazingly fast, guarantees memory safety without a garbage collector, and is fun to write. It has a vibrant community and excellent tooling, making it the most loved programming language for several years in a row.

In this course, you will:

- Start with a brief theoretical section on low-level memory management.
- Build a simple command-line application to introduce Rust basics.
- Progress to a larger project: a custom HTTP server, implementing the protocol and server logic from scratch.

## Features

- **Custom HTTP Protocol Implementation:** Learn how HTTP works under the hood by building your own protocol parser and server.
- **Fundamental Rust Concepts:** Ownership, borrowing, pattern matching, enums, structs, modules, and more.
- **Real Networking:** Use Rust's standard library to handle TCP connections and process HTTP requests.
- **Configurable Server:** Configure the server's host, port, and public directory via environment variables (`HOST`, `PORT`, `PUBLIC_PATH`).
- **Static File Serving:** Serves files from a configurable public directory, with basic protection against directory traversal attacks.
- **Incremental Learning:** Concepts are introduced as needed to solve real problems.

## Project Structure

```
rust_server/
├── Cargo.toml
├── src/
│   ├── main.rs         # Entry point: starts the server
│   ├── server.rs       # Server logic: TCP listener and request handling
│   ├── website_handler.rs # Handles static file serving and routing
│   └── http/
│       ├── mod.rs      # HTTP module exports
│       ├── method.rs   # HTTP method enum (GET, POST, etc.)
│       ├── request.rs  # HTTP request struct and parser
│       ├── response.rs # HTTP response struct
│       ├── status_code.rs # HTTP status codes
│       └── query_string.rs # Query string parsing
├── public/
│   ├── index.html      # Default homepage
│   ├── hello.html      # Example hello page
│   └── style.css       # Example stylesheet
```

## Usage

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)

### Running the Server

1. **Clone the repository:**

   ```sh
   git clone <repo-url>
   cd rust_server
   ```

2. **Build and run:**

   ```sh
   cargo run
   ```

   The server will start and listen on `127.0.0.1:8080` by default.

   You can override the default host, port, and public directory using environment variables:

   ```sh
   HOST=0.0.0.0 PORT=8000 PUBLIC_PATH=./public cargo run
   ```

3. **Test the server:**
   - Use `curl`, Postman, or your browser to send HTTP requests:

     ```sh
     curl http://127.0.0.1:8080/
     ```

   - The server will print received requests to the console.

## Current Functionality

- **HTTP Methods:** Only `GET` requests are handled by the default website handler. Other methods return a 404 Not Found response, though the codebase defines all standard HTTP methods for future extensibility.
- **Routing:**
  - `/` serves `index.html` from the public directory.
  - `/hello` serves `hello.html` from the public directory.
  - Any other path attempts to serve the corresponding file from the public directory, or returns 404 if not found.
- **Security:** Basic protection against directory traversal attacks when serving files.
- **Request Parsing:** The server parses the HTTP method, path, and query string from incoming requests. HTTP headers are not yet parsed or handled.

## Key Concepts Demonstrated

- **Enums with Data:** The `Method` enum in `http/method.rs` demonstrates how Rust enums can store data (e.g., query strings, user IDs).
- **Structs and Modules:** The project is organized into modules for clarity and scalability.
- **TCP Networking:** The server uses `TcpListener` to accept and process incoming connections.
- **Memory Safety:** All code is written in safe Rust, leveraging ownership and borrowing.
- **Environment Configuration:** Host, port, and public directory can be set via environment variables.

## Example Code

```rust
// src/main.rs
fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let host = env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let server = Server::new(format!("{}:{}", host, port));
    println!("================================================");
    println!("Server is running on http://{}", server.addr);
    println!(
        "Serving files from public path: {}",
        std::path::absolute(std::path::Path::new(&public_path)).unwrap().display()
    );
    println!("================================================");
    server.run(WebsiteHandler::new(public_path));
}
```

## Learning Outcomes

By the end of this project, you will:

- Understand the basics of Rust syntax and semantics.
- Be able to build and structure real Rust applications.
- Know how to implement a simple HTTP server and protocol parser.
- Gain confidence in using Rust for systems programming tasks.

## Addendum

### Question mark operator `?`

- Explanation with example: https://medium.com/@vennilapugazhenthi/what-does-the-question-mark-operator-do-in-rust-581fe7bc4b0e

- "?" is used at the end of an expression returning a Result,
and is equivalent to a match expression, where the Err(err) branch
expands to an early return Err(From::from(err)),
and the Ok(ok) branch expands to an ok expression.

### Environment variables

See https://doc.rust-lang.org/cargo/reference/environment-variables.html for officially supported environment variables.

### String

- Explanation String vs. &str: https://ezesunday.com/blog/choosing-between-str-and-string-in-rust/

## Future Improvements

This project is a solid foundation for a basic HTTP server, but there are many ways it can be extended and improved:

- **Handle HTTP Headers:** Parse and process HTTP headers from incoming requests, and allow custom headers in responses.
- **Support More HTTP Methods:** Extend the handler logic to support POST, PUT, DELETE, and other HTTP methods already defined in the codebase.
- **Multithreading:** Use the `std::thread` module to spawn a new thread for each incoming connection, allowing the server to handle multiple requests concurrently. Use synchronization primitives from `std::sync` (such as `Mutex`, `Arc`, etc.) to safely share state between threads if needed.
- **Asynchronous Rust:** Refactor the server to use asynchronous I/O with [Tokio](https://tokio.rs/) or async-std, enabling efficient handling of many simultaneous connections with minimal threads.
- **Better Error Handling:** Improve error messages and add more robust error handling throughout the codebase.
- **Logging:** Integrate a logging framework for better observability and debugging.
- **HTTPS Support:** Add support for TLS/SSL to serve content securely.
- **Configuration File:** Allow configuration via a file in addition to environment variables.
- **Unit and Integration Tests:** Expand test coverage, especially for request parsing and handler logic.
