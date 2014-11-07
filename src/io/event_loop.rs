use io::backend::{Poller, epoll, EpollEventKind};
use utils::BoundedQueue;
use libc::c_void;
use std::collections::TreeMap;
use std::mem;
use native::io::file::fd_t;

use super::{AsyncOperation, Pollable, IoFlag, IoEvent};

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

    pub watchers: TreeMap<fd_t, &'a AsyncOperation + 'a>,
    pub events_queue: BoundedQueue<&'a AsyncOperation + 'a>
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
                let flags = event.poll_flags();

                self.poller.add_poll_list(fd, flags);
                self.watchers.insert(fd,  event);
            }

            let poll_events = self.poller.poll(consts::POLL_TIMEOUT).unwrap();

            for io_event in poll_events.iter() {
                self.process_event(io_event);
            }
        }
    }

    pub fn add_event(&mut self, event: &'a AsyncOperation) {
        match self.events_queue.push(event) {
            Err(Full) => panic!("The event queue is full"),
            Ok(_) => ()
        }
    }

    fn process_event(&mut self, event: &IoEvent) {
        let fd = event.data;
        match self.watchers.find(&fd) {
             Some(watcher) => {
                 let flags = event.flags;
                 let new_flags = watcher.process(flags);
                 if flags != new_flags {
                     self.poller.modify_poll_list(fd, new_flags);
                 }
             }
             None => () 
        }
    }

}

mod consts {
    pub static POLL_TIMEOUT: uint = 1000;
}
