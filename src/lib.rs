#![crate_name = "lys"]
#![comment = "Lys is a safe and scalable HTTP server implementation for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![feature(macro_rules, phase, globs)]

extern crate libc;
extern crate native;

pub mod io;
