use std::mem;
use std::ptr;
use libc::{c_int, c_void, time_t, size_t, timespec, read, close, CLOCK_MONOTONIC};
use native::io::file::fd_t;
use io::event_loop::EventLoop;
use io::errno::{SysCallResult, Errno, consts};
use super::{AsyncEvent, IoFlag, POLL_IN, POLL_OUT};

#[repr(C, packed)]
struct TimerSpec {
    it_interval: timespec,
    it_value: timespec
}

#[link(name = "rt")]
extern {
    fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
    fn timerfd_settime(fd: c_int, flags: c_int, new_value: *const TimerSpec,
                       old_value: *mut TimerSpec) -> c_int;
    fn timerfd_gettime(fd: c_int, curr_value: *mut TimerSpec) -> c_int;

    fn clock_gettime(clockid: c_int, tp: *mut timespec) -> c_int;
}

#[allow(dead_code)]
bitflags!(
    flags TimerFdFlag: c_int {
        const TFD_CLOEXEC   = 0o2000000,
        const TFD_NONBLOCK  = 0o0004000
    }
)


fn create_timerfd(interval: u64, single_shot: bool) -> SysCallResult<fd_t>
{
    let fd = unsafe { timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK.bits()) };
    if fd < 0 {
        return Err(Errno::current());
    }

    let mut now = timespec { tv_sec: 0, tv_nsec: 0};

    let ct = unsafe { clock_gettime(CLOCK_MONOTONIC, &mut now as *mut timespec) };
    if ct < 0 {
        return Err(Errno::current());
    }

    let sec_interval = if single_shot { 0 } else { interval };

    let new_value = TimerSpec {
        it_value: timespec {
            tv_sec: interval as time_t,
            tv_nsec: 0
        },
        it_interval: timespec {
             tv_sec: sec_interval as time_t,
             tv_nsec: 0
         }
    };

    let st = unsafe {
        timerfd_settime(fd, 0, mem::transmute(&new_value), 0 as *mut TimerSpec)
    };

    if st < 0 {
         return Err(Errno::current());
    }

    Ok(fd)
}

pub type OnTimer = fn(timer: &Timer, numTimeouts: u64);

pub struct Timer {
    callback: OnTimer,
    interval: u64,
    active: bool,

    fd: fd_t,
    events: IoFlag
}

impl Timer {
    pub fn new(callback: OnTimer, interval: u64) -> SysCallResult<Timer> {
        match create_timerfd(interval, false) {
            Ok(fd) => Ok(Timer {
                callback: callback,
                interval: interval,
                fd: fd,
                active: false,
                events: POLL_IN
            }),
            Err(errno) => Err(errno)
        }
    }

    pub fn single_shot(callback: OnTimer, interval: u64) -> SysCallResult<Timer> {
        match create_timerfd(interval, true) {
            Ok(fd) => Ok(Timer {
                callback: callback,
                interval: interval,
                fd: fd,
                active: false,
                events: POLL_IN
            }),
            Err(errno) => Err(errno)
        }
    }

    pub fn attach_to<'a>(&'a self, ev_loop: &mut EventLoop<'a>) {
       // ev_loop.poller.register(self.fd);

       // ev_loop.watchers.insert(self.fd, self);
    }

}

impl AsyncEvent for Timer {
    fn process(&self) {

        let mut num_timeouts: u64 = 0;
        loop {
            let res = unsafe {
               read(self.fd, mem::transmute(&num_timeouts), 8 as size_t)
            };
            if res == -1 {
                let err = Errno::current();
                match err.value() {
                    consts::EAGAIN => break,
                    _ => panic!(err)
                }
            }

            if res != 8 {
                panic!("Timer: failed to read the right number of bytes");
            }

            (self.callback)(self, num_timeouts);
            break;
        }

    }

    fn poll_fd(&self) -> fd_t { self.fd }

    fn stop(&mut self) { unsafe { close(self.fd) }; }

    fn flags(&self) -> IoFlag { self.events }
}
