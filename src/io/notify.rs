use libc::{c_uint, c_int, size_t, read, write, close};
use native::io::file::fd_t;
use std::mem;
use io::errno::{SysCallResult, Errno, consts};
use io::event_loop::EventLoop;

use io::{AsyncEvent, IoFlag, POLL_IN, POLL_OUT};

extern {
    fn eventfd(init_val: c_uint, flags: c_int) -> c_int;
}

bitflags!(
    flags TimerFdFlag: c_int {
        const EFD_CLOEXEC   = 0o2000000,
        const EFD_NONBLOCK  = 0o0004000
    }
)

pub type OnNotify = fn(notify: &Notify);

pub struct Notify {
    callback: OnNotify,
    active: bool,

    fd: fd_t,
    events: IoFlag
}


impl Notify {
    pub fn new(callback: OnNotify) -> SysCallResult<Notify> {
        let fd = unsafe {
            eventfd(0, EFD_NONBLOCK.bits())
        };

        if fd < 0 {
            return Err(Errno::current());
        }

        Ok(Notify {
            callback: callback,
            fd: fd,
            active: false,
            events: POLL_IN
        })
    }

    pub fn notify(&self) -> SysCallResult<()> {
        let to_write: u64 = 1;

        let res = unsafe {
            write(self.fd, mem::transmute(&to_write), 8 as size_t)
        };

        if res == -1 {
            return Err(Errno::current());
        }

        Ok( () )
    }
}

impl AsyncEvent for Notify {
    fn process(&self) {
        let value: u64 = 0;
        loop {
            let res = unsafe {
                read(self.fd, mem::transmute(&value), 8)
            };

            if res == -1 {
                match Errno::current().value() {
                    consts::EAGAIN => break,
                    err => panic!(err)
                }
            }

            if res != 8 {
                panic!("Notify: failed to read the right number of bytes");
            }

            (self.callback)(self)
        }
    }

    fn poll_fd(&self) -> fd_t { self.fd }

    fn stop(&mut self) { unsafe { close(self.fd) }; }

    fn flags(&self) -> IoFlag { self.events }
}
