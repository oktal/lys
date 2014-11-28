use libc::{c_uint, c_int, size_t, read, write, close};
use std::mem;
use io::errno::{SysCallResult, Errno, consts};
use io::event_loop::EventLoop;

use io::{
    Pollable, IoEventHandler, AsyncIoProvider,
    IoEvent, IoFlag, EventData, POLL_IN, POLL_OUT
};
use io::fd_t;

use std::ptr;

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
    fd: fd_t,
    events: IoFlag,
}


impl Notify {
    pub fn new() -> SysCallResult<Notify> {
        let fd = unsafe {
            eventfd(0, EFD_NONBLOCK.bits())
        };

        if fd < 0 {
            return Err(Errno::current());
        }

        Ok(Notify {
            fd: fd,
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

impl Pollable for Notify {
    fn poll_fd(&self) -> fd_t { self.fd }

    fn poll_flags(&self) -> IoFlag { self.events }
}

impl AsyncIoProvider for Notify {
    fn handle_event(&self, event: &EventData, handler: &IoEventHandler) {
        if event.is_readable() {
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

                handler.handle_event(IoEvent::Notify);
            }
        }

    }

}
