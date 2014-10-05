use std::mem;
use events::event_loop::EventLoop;
use events::errno::{SysCallResult, Errno};
use events::libc::{c_int, c_void, time_t, size_t, timespec, read, CLOCK_MONOTONIC};

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

bitflags!(
    flags TimerFdFlag: c_int {
        static TFD_CLOEXEC   = 0o2000000,
        static TFD_NONBLOCK  = 0o0004000
    }
)


pub trait TimerCallback {
   fn call(&self, numTimeouts: u64);
}

pub struct Timer {
    callback: Box<TimerCallback>,
    interval: u64,

    fd: i32
}

impl Timer {
    pub fn new(callback: Box<TimerCallback>, interval: u64) -> SysCallResult<Timer> {
        let fd = unsafe { timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK.bits()) };
        if fd < 0 {
            return Err(Errno::value());
        }

        let mut now = timespec { tv_sec: 0, tv_nsec: 0};

        let ct = unsafe { clock_gettime(CLOCK_MONOTONIC, &mut now as *mut timespec) };
        if ct < 0 {
            return Err(Errno::value());
        }

        let new_value = TimerSpec {
            it_value: timespec {
                tv_sec: interval as time_t,
                tv_nsec: 0
            },
            it_interval: timespec {
                 tv_sec: interval as time_t,
                 tv_nsec: 0
             }
        };

        let st = unsafe {
            timerfd_settime(fd, 0, &new_value as *const TimerSpec, 0 as *mut TimerSpec)
        };

        if st < 0 {
             return Err(Errno::value());
        }

        Ok(Timer { callback: callback, interval: interval, fd: fd })
    }

    pub fn attach_to(&self, evLoop: EventLoop) {
        unsafe {
            evLoop.poller.register(self.fd, mem::transmute(self))
        }
    }

    pub fn process(&mut self) {
        let mut numTimeouts: u64 = 0;
        loop {
            let res = unsafe {
               read(self.fd, mem::transmute(numTimeouts), 8 as size_t)
            };
            if res == -1 {
                continue;
            }

            if res != 8 {
                fail!("Timer: failed to read the right number of bytes");
            }

            self.callback.call(numTimeouts);
            break;
        }

    }
}
