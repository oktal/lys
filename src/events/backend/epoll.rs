use libc::{c_int, c_void};
use std::mem;

extern {
    pub fn epoll_create(size: c_int) -> c_int;
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *const EpollEvent) -> c_int;
    pub fn epoll_wait(epfd: c_int, events: *mut EpollEvent, max_events: c_int, timeout: c_int) -> c_int;
}

#[repr(C)]
pub enum EpollControl {
    EpollCtlAdd = 1,
    EpollCtlDel = 2,
    EpollCtlMod = 3
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

pub struct Epoll {
    efd: i32
}

impl Epoll {
    pub fn new(size: u64) -> Epoll {
        let fd = unsafe { epoll_create(1024) };

        if fd < 0 {
            fail!("Failed to epoll_create");
        }

        Epoll { efd: fd }
    }

    pub fn register(&self, fd: i32) {
        let kind = EPOLLIN | EPOLLOUT;
        let event = EpollEvent {
            events: kind,
            data: fd as c_int
        };

        let res = unsafe {
            epoll_ctl(self.efd, EpollCtlAdd as c_int, fd, &event as *const EpollEvent)
        };

        if res < 0 {
            fail!("Failed to epoll_ctl");
        }
    }

    pub fn poll(&self, events: &mut [EpollEvent], timeout_ms: uint) -> i32 {
        let res = unsafe {
            epoll_wait(self.efd, events.as_mut_ptr(), events.len() as c_int,
                       timeout_ms as c_int)
        };

        res
    }

}
