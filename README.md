Lys
=====

Lys is a projet that aims to implementation an experimental REST SDK in Rust

Rust, even though yet not very popular, is a very promising language made-in Mozilla.
However, for real use cases, Rust lacks a good standard library support for networking.

For example, it is not yet possible to start an HttpServer that can handle incoming
REST requests in Rust.
As opposed to Rust, Go has a very good networking standard library that provides both
an HTTP server and HTTP client implementations. In a few lines of code, it is possible
in Go to create an http rest service.

The goal of this projet is to try implementing a robust and scalable REST SDK in Rust.
This project is essentially a research project as it involves many things like parsing
(HTTP request parsing), routing, non-blocking event handling, concurrency, asynchronous
programming, ...

