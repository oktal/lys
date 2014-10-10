use backend::epoll;
use libc::c_void;
use std::collections::TreeMap;
use std::mem;

use super::AsyncEvent;

// TODO: Fix all the unsafe crap

pub enum BackendType {
    /// Use the select() system call as a backend for the event loop
    Select,

    /// Use the poll() system call
    Poll,

    /// Use the epoll() system call
    Epoll,

    /// Use the BSD kqueue() system call
    Kqueue,

    /// Use Windows IOCP
    IOCP
}

pub struct EventLoop<'a> {
    pub poller: epoll::Epoll,

    pub events: TreeMap<i32, &'a AsyncEvent + 'a>
}

impl<'a> EventLoop<'a> {
    #[cfg(target_os = "linux")]
    pub fn default() -> EventLoop<'a> {
        let poller = epoll::Epoll::new(1 << 16);

        EventLoop { poller: poller, events: TreeMap::new() }
    }

    pub fn run(&self) {
        loop {
            let mut events: [epoll::EpollEvent, ..256]
               = unsafe { mem::uninitialized() };

            let readyCount = self.poller.poll(events, 1000);
            if readyCount > 0 {
                for i in range(0, readyCount as uint) {
                    if events[i].events.contains(epoll::EPOLLIN) {
                        let fd = events[i].data;
                        match self.events.find(&fd) {
                             Some(&asyncEvent) => asyncEvent.process(),
                             None => ()
                        }
                    }
                }
            }
        }
    }
}
