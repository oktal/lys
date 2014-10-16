use io::backend::epoll;
use utils::BoundedQueue;
use libc::c_void;
use std::collections::TreeMap;
use std::mem;
use native::io::file::fd_t;

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

    pub watchers: TreeMap<fd_t, &'a AsyncEvent + 'a>,
    pub events_queue: BoundedQueue<&'a AsyncEvent + 'a>
}

impl<'a> EventLoop<'a> {
    #[cfg(target_os = "linux")]
    pub fn default() -> EventLoop<'a> {
        let poller = epoll::Epoll::new(1 << 16).unwrap();

        EventLoop {
            poller: poller,
            watchers: TreeMap::new(),
            events_queue: BoundedQueue::new(1 << 8)
        }
    }

    pub fn run(&mut self) {
        loop {
            while (!self.events_queue.is_empty()) {
                let event = self.events_queue.pop().ok().unwrap();
                let fd = event.poll_fd();

                self.poller.register(fd);
                self.watchers.insert(fd,  event);
            }

            let mut events: [epoll::EpollEvent, ..256]
               = unsafe { mem::uninitialized() };

            let readyCount = self.poller.poll(events, consts::POLL_TIMEOUT).unwrap();
            for event in events.iter().take(readyCount) {
                if event.events.contains(epoll::EPOLLIN) {
                    let fd = event.data;
                    match self.watchers.find(&fd) {
                         Some(&asyncEvent) => asyncEvent.process(),
                         None => ()
                    }
                }
            }
        }
    }

    pub fn add_event(&mut self, event: &'a AsyncEvent) {
        match self.events_queue.push(event) {
            Err(Full) => fail!("The event queue is full"),
            Ok(_) => ()
        }
    }
}

mod consts {
    pub static POLL_TIMEOUT: uint = 1000;
}
