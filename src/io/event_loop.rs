use io::backend::{Poller, epoll, EpollEventKind};
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


fn create_poller(backend_type: BackendType) -> Box<Poller + 'static> {
    match backend_type {
        Select => unimplemented!(),
        Poll => unimplemented!(),
        Epoll => box epoll::Epoll::new(1 << 16).unwrap(),
        Kqueue => unimplemented!(),
        IOCP => unimplemented!()
    }
}

pub struct EventLoop<'a> {
    pub poller: Box<Poller + 'static>,

    pub watchers: TreeMap<fd_t, &'a AsyncEvent + 'a>,
    pub events_queue: BoundedQueue<&'a AsyncEvent + 'a>
}

impl<'a> EventLoop<'a> {
    #[cfg(target_os = "linux")]
    pub fn default() -> EventLoop<'a> {

        EventLoop {
            poller: create_poller(Epoll),
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

                self.poller.add_poll_list(fd, flags);
                self.watchers.insert(fd,  event);
            }

            let poll_events = self.poller.poll(consts::POLL_TIMEOUT).unwrap();

            for io_event in poll_events.iter() {
                let fd = io_event.data;
                match self.watchers.find(&fd) {
                     Some(event) => {
                         let before_flags = event.flags();
                         event.process();
                         let after_flags = event.flags();
                         if after_flags != before_flags {
                             self.poller.modify_poll_list(fd, after_flags);
                         }
                     }
                     None => () 
                }
            }
        }
    }

    pub fn add_event(&mut self, event: &'a AsyncEvent) {
        match self.events_queue.push(event) {
            Err(Full) => panic!("The event queue is full"),
            Ok(_) => ()
        }
    }
}

mod consts {
    pub static POLL_TIMEOUT: uint = 1000;
}
