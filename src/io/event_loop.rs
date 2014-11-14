use io::backend::{Poller, epoll, EpollEventKind};
use utils::BoundedQueue;
use libc::c_void;
use std::collections::TreeMap;
use std::mem;
use native::io::file::fd_t;

use super::{Async, Pollable, AsyncReadable, AsyncWritable, IoFlag, IoEvent};

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

    pub watchers: TreeMap<fd_t, &'a Async + 'a>,
    pub events_queue: BoundedQueue<&'a Async + 'a>
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

    pub fn add_event(&mut self, event: &'a Async) {
        match self.events_queue.push(event) {
            Err(Full) => panic!("The event queue is full"),
            Ok(_) => ()
        }
    }

    fn process_event(&mut self, event: &IoEvent) {
        use super::{POLL_IN, POLL_OUT};

        let fd = event.data;
        let watcher = self.watchers.find(&fd).unwrap();
        let flags = event.flags;
        if flags.contains(POLL_IN) {
            if !watcher.is_readable() {
                panic!("Recevied a POLL_IN on a non-readable event")
            }

            watcher.handle_read();
        }

        if flags.contains(POLL_OUT) {
            if !watcher.is_writable() {
                panic!("Received a POLL_OUT a non-writable event")
            }

            watcher.handle_write();
        }

        let new_flags = watcher.poll_flags();

        if flags != new_flags {
            self.poller.modify_poll_list(fd, new_flags);
        }
    }

}

mod consts {
    pub static POLL_TIMEOUT: uint = 1000;
}
