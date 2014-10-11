use libc::{c_uint, c_int, size_t, read, write};
use native::io::file::fd_t;
use std::mem;
use errno::{SysCallResult, Errno, consts};
use event_loop::EventLoop;
use super::AsyncEvent;

extern {
    fn eventfd(init_val: c_uint, flags: c_int) -> c_int;
}

bitflags!(
    flags TimerFdFlag: c_int {
        const EFD_CLOEXEC   = 0o2000000,
        const EFD_NONBLOCK  = 0o0004000
    }
)

pub struct Notify {
    callback: fn(),

    fd: fd_t
}

impl Notify {
    pub fn new(callback: fn()) -> SysCallResult<Notify> {
        let fd = unsafe {
            eventfd(0, EFD_NONBLOCK.bits())
        };

        if (fd < 0) {
            return Err(Errno::current());
        }

        Ok(Notify { callback: callback, fd: fd })
    }

    pub fn attach_to<'a>(&'a self, ev_loop: &mut EventLoop<'a>) {
        ev_loop.poller.register(self.fd);

        ev_loop.events.insert(self.fd, self);
    }

    pub fn poll_fd(&self) -> fd_t { self.fd }

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
                    err => fail!(err)
                }
            }

            if res != 8 {
                fail!("Notify: failed to read the right number of bytes");
            }

            (self.callback)()
        }
    }
}
