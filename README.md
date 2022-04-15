# Robust, ergonomic Rust CoAP server

An asynchronous [CoAP](https://coap.technology/) server with a modern and
ergonomic API for larger scale applications, inspired by warp and actix.  CoAP
offers an excellent alternative to HTTP for resource constrained environments
like IoT devices.

* **Ergonomic**: Fluent app-builder API makes it easy to compose rich
  applications, including those that use more advanced CoAP features.
* **Concurrent**: High concurrency is supported by using a separate spawned
  task for each request, allowing long running requests to not interfere with
  shorter running ones at scale.
* **Feature-rich**: Supports a wide range of CoAP server features including
  [Observe](https://datatracker.ietf.org/doc/html/rfc7641), and [Block-wise
  Transfer](https://datatracker.ietf.org/doc/html/rfc7959).
* **Flexible**: Supports pluggable transport backends with goals of supporting
  alternative async runtimes like
  [embassy](https://github.com/embassy-rs/embassy).

## Status

[![GitHub tag](https://img.shields.io/github/tag/jasta/coap-server-rs.svg)](https://github.com/jasta/coap-server-rs)
[![Build Status](https://github.com/jasta/coap-server-rs/workflows/Rust/badge.svg)](https://github.com/jasta/coap-server-rs/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/tokio-rs/tokio/blob/master/LICENSE)

This project is a work in progress, attempting to provide a rich and stable
codebase for further development.  See [Project
Status](https://github.com/jasta/coap-server-rs/issues/1) tracking issue for
more info.

## Example

```rust
use coap_server::app::{CoapError, Request, Response};
use coap_server::{app, CoapServer, FatalServerError, UdpTransport};

#[tokio::main]
async fn main() -> Result<(), FatalServerError> {
    let server = CoapServer::bind(UdpTransport::new("0.0.0.0:5683")).await?;
    server.serve(
        app::new().resource(
            app::resource("/hello").get(handle_get_hello))
    ).await
}

async fn handle_get_hello(request: Request<SocketAddr>) -> Result<Response, CoapError> {
    let whom = request
        .unmatched_path
        .first()
        .cloned()
        .unwrap_or_else(|| "world".to_string());

    let mut response = request.new_response();
    response.message.payload = format!("Hello, {whom}").into_bytes();
    Ok(response)
}
```

## Related Projects

- [martindisch/coap-lite](https://github.com/martindisch/coap-lite): used by
  this project as the low-level basis for CoAP protocol support
- [Covertness/coap-rs](https://github.com/Covertness/coap-rs): original server
  I used but outgrew when I needed more robust features like generic Observe
  support and /.well-known/core filtering.
- [ryankurte/rust-coap-client](https://raw.githubusercontent.com/ryankurte/rust-coap-client):
  inspired the creation of this crate based on the excellent generalization of
  client backends
- [google/rust-async-coap](https://github.com/google/rust-async-coap)
