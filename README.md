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
- **Incremental Learning:** Concepts are introduced as needed to solve real problems.

## Project Structure

```
rust_server/
├── Cargo.toml
├── src/
│   ├── main.rs         # Entry point: starts the server
│   ├── server.rs       # Server logic: TCP listener and request handling
│   └── http/
│       ├── mod.rs      # HTTP module exports
│       ├── method.rs   # HTTP method enum (GET, POST, etc.)
│       └── request.rs  # HTTP request struct
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

3. **Test the server:**
   - Use `curl`, Postman, or your browser to send HTTP requests:
     ```sh
     curl http://127.0.0.1:8080/
     ```

   - The server will print received requests to the console.

## Key Concepts Demonstrated

- **Enums with Data:** The `Method` enum in `http/method.rs` demonstrates how Rust enums can store data (e.g., query strings, user IDs).
- **Structs and Modules:** The project is organized into modules for clarity and scalability.
- **TCP Networking:** The server uses `TcpListener` to accept and process incoming connections.
- **Memory Safety:** All code is written in safe Rust, leveraging ownership and borrowing.

## Example Code

```rust
// src/main.rs
fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    println!("========== Server is running on {} ==========", server.addr);
    server.run();
}
```

## Learning Outcomes

By the end of this project, you will:

- Understand the basics of Rust syntax and semantics.
- Be able to build and structure real Rust applications.
- Know how to implement a simple HTTP server and protocol parser.
- Gain confidence in using Rust for systems programming tasks.
