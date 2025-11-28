# rust-http-sever


![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Build Status](https://github.com/joseph-mpo-yeti/rust-http-server/actions/workflows/rust.yml/badge.svg)


A **very basic asynchronous HTTP server library written in Rust** — minimal, educational, and easy to extend. Inspired by Node's Express framework.

## 🚀 What is this

This project implements a simple HTTP/1.1 server using Rust (with or without dependencies, depending on version).  
It’s designed to demonstrate the core building blocks of HTTP:  
request parsing, response formatting, socket management — without needing a big framework.  

Use it to learn how HTTP works under the hood, or as a lightweight starting point for building a custom server.

## ✅ Features (current)

- Accepts TCP connections and parses basic HTTP/1.1 requests  
- Builds and sends valid HTTP responses (status line, headers, body)  
- Supports correct `Content-Length` and response formatting  
- Closes connections cleanly (when configured)  
- Minimal dependencies: works as a simple read-and-write server  
- Supports logging with `log` crate. You must add the backend of your choice and enable logging on the server.

## Dependencies

This project depends on the [`Tokio`](https://tokio.rs/) runtime to process requests asynchronously.

# Using This Library in Your Rust Project

To include this library directly from Git, add the following to your `Cargo.toml`:

```toml
[dependencies]
http = { git = "https://github.com/joseph-mpo-yeti/rust-http-server.git" }
```

## 🧪 Example 

The example below uses [`env_logger`](https://crates.io/crates/env_logger) crate as logging backend.

```rs
use env_logger::Env;
use http::prelude::*;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut router = HttpRouter::new();
    router.get("/hello", |_req| {
        HttpResponse::builder()
            .status_code(StatusCode::Ok)
            .body("{\"message\":\"Hello World!\"}".to_string())
            .header("Content-Type", "application/json")
            .build()
    });

    router.post("/api/v1", |_req| {
        HttpResponse::builder()
            .status_code(StatusCode::Ok)
            .body("{\"message\":\"Hello World!\"}".to_string())
            .header("Content-Type", "application/json")
            .build()
    });

    let mut server = HttpServer::new(router);
    server.enable_logging();
    server.listen(5050);

}

