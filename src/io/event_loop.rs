use io::backend::{epoll, EpollEventKind};
use utils::BoundedQueue;
use libc::c_void;
use std::collections::TreeMap;
use std::mem;
use native::io::file::fd_t;

use super::{AsyncEvent, IoFlag};

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
        use super::{POLL_IN, POLL_OUT};

        loop {
            // First we dequeue all the events and add them to the loop
            while (!self.events_queue.is_empty()) {
                let mut event = self.events_queue.pop().ok().unwrap();
                let fd = event.poll_fd();
                let flags = event.flags();

                let mut events: EpollEventKind = epoll::EPOLLIN;
                if flags.contains(POLL_OUT) {
                    events = events | epoll::EPOLLOUT;
                }

                println!("fd => {}, flags = {}", fd, events.bits());


                self.poller.register(fd, events);
                self.watchers.insert(fd,  event);
            }

            let mut events: [epoll::EpollEvent, ..256]
               = unsafe { mem::uninitialized() };

            let readyCount = self.poller.poll(events, consts::POLL_TIMEOUT).unwrap();
            for event in events.iter().take(readyCount) {
                println!("fd {} is ready", event.data);
                if event.events.contains(epoll::EPOLLIN) ||
                   event.events.contains(epoll::EPOLLOUT) {
                    let fd = event.data;
                    match self.watchers.find(&fd) {
                         Some(event) => event.process(),
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
