#![crate_name = "lio"]
#![comment = "Lys IO is an asynchronous event handling mechanism for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![feature(macro_rules, phase, globs)]

extern crate libc;

pub use event_loop::EventLoop;
pub use errno::{SysCallResult, Errno};
pub use timer::Timer;
pub use notify::Notify;

pub trait AsyncEvent {
    fn process(&self);
}

mod event_loop;
mod backend;
mod errno;
mod timer;
mod notify;
