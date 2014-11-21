use libc::{c_int, c_void};
use std::mem;
use std::ptr;
use io::errno::{SysCallResult, Errno};
use io::{IoFlag, IoEvent, POLL_IN, POLL_OUT};
use io::fd_t;

use super::{Poller, ToIoFlags, FromIoFlags};

extern {
    pub fn epoll_create(size: c_int) -> c_int;
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *const EpollEvent) -> c_int;
    pub fn epoll_wait(epfd: c_int, events: *mut EpollEvent, max_events: c_int, timeout: c_int) -> c_int;
}

#[repr(C)]
pub enum EpollControl {
    Add = 1,
    Del = 2,
    Mod = 3
}

#[repr(C, packed)]
pub struct EpollEvent {
    pub events: EpollEventKind,
    pub data: c_int
}

bitflags!(
    flags EpollEventKind: u32 {
        const EPOLLIN = 0x001,
        const EPOLLPRI = 0x002,
        const EPOLLOUT = 0x004,
        const EPOLLRDNORM = 0x040,
        const EPOLLRDBAND = 0x080,
        const EPOLLWRNORM = 0x100,
        const EPOLLWRBAND = 0x200,
        const EPOLLMSG = 0x400,
        const EPOLLERR = 0x008,
        const EPOLLHUP = 0x010,
        const EPOLLRDHUP = 0x2000,
        const EPOLLWAKEUP = 1 << 29,
        const EPOLLONESHOT = 1 << 30,
        const EPOLLET = 1 << 31
    }
)

impl ToIoFlags for EpollEventKind {
    fn to_io_flags(&self) -> IoFlag {
        let mut io_flags = IoFlag::empty();
        if self.contains(EPOLLIN) {
            io_flags.toggle(POLL_IN);
        }

        if self.contains(EPOLLOUT) {
            io_flags.toggle(POLL_OUT);
        }

        return io_flags;
    }
}

impl FromIoFlags for EpollEventKind {
    fn from_io_flags(flags: IoFlag) -> EpollEventKind {
        let mut epoll_flags = EpollEventKind::empty();
        if flags.contains(POLL_IN) {
            epoll_flags.toggle(EPOLLIN);
        }

        if flags.contains(POLL_OUT) {
            epoll_flags.toggle(EPOLLOUT)
        }

        return epoll_flags;
    }
}

pub struct Epoll {
    efd: fd_t
}

impl Epoll {
    pub fn new(size: u64) -> SysCallResult<Epoll> {
        let fd = unsafe { epoll_create(1024) };

        if fd < 0 {
            return Err(Errno::current());
        }

        Ok(Epoll { efd: fd })
    }
}

impl Poller for Epoll {

    fn add_poll_list(&mut self, fd: fd_t, flags: IoFlag) -> SysCallResult<()> {
        let event = EpollEvent {
            events: FromIoFlags::from_io_flags(flags),
            data: fd as c_int
        };

        let res = unsafe {
            epoll_ctl(self.efd, EpollControl::Add as c_int, fd, &event as *const EpollEvent)
        };

        if res < 0 {
            return Err(Errno::current());
        }

        Ok( () )
    }

    fn modify_poll_list(&mut self, fd: fd_t, flags: IoFlag) -> SysCallResult<()> {
        let event = EpollEvent {
            events: FromIoFlags::from_io_flags(flags),
            data: fd as c_int
        };

        let res = unsafe {
            epoll_ctl(self.efd, EpollControl::Mod as c_int, fd, &event as *const EpollEvent)
        };

        if res < 0 {
            return Err(Errno::current());
        }

        Ok ( () )
    }

    fn remove_poll_list(&mut self, fd: fd_t) -> SysCallResult<()> {
        let res = unsafe {
            epoll_ctl(self.efd, EpollControl::Del as c_int, fd, ptr::null())
        };

        if res < 0 {
            return Err(Errno::current());
        }

        Ok( () )
    }

    fn poll(&self, timeout_ms: uint) -> SysCallResult<Vec<IoEvent>> {
        let mut epoll_events: [EpollEvent, ..256] = unsafe { mem::uninitialized() };

        let res = unsafe {
            epoll_wait(self.efd, epoll_events.as_mut_ptr(), epoll_events.len() as c_int,
                       timeout_ms as c_int)
        };

        if res < 0 {
            return Err(Errno::current());
        }

        // Here we are guaranteed that res will not be < 0. Thus, casting it to an uint is safe
        let mut io_events = Vec::with_capacity(res as uint);

        for epoll_event in epoll_events.iter().take(res as uint) {
            io_events.push(IoEvent {
                flags: epoll_event.events.to_io_flags(),
                data: epoll_event.data
            });
        }

        Ok(io_events)
    }

}
