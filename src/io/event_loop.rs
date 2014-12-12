use io::backend::{Poller, epoll, EpollEventKind};
use utils::BoundedQueue;
use libc::c_void;
use std::collections::TreeMap;
use std::rc::Rc;
use std::mem;

use super::{AsyncIoProvider, IoEventHandler, EventData, Pollable, IoFlag};
use super::fd_t;
use io::Notify;

pub enum Backend {
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


fn create_poller(backend_type: Backend) -> Box<Poller + 'static> {
    match backend_type {
        Backend::Select => unimplemented!(),
        Backend::Poll => unimplemented!(),
        Backend::Epoll => box epoll::Epoll::new(1 << 16).unwrap(),
        Backend::Kqueue => unimplemented!(),
        Backend::IOCP => unimplemented!()
    }
}

pub struct EventLoop<'a> {
    pub poller: Box<Poller + 'static>,

    pub watchers: TreeMap<fd_t, Rc<Box<AsyncIoProvider + 'a>>>,
    pub events_queue: BoundedQueue<Rc<Box<AsyncIoProvider + 'a>>>
}

impl<'a> EventLoop<'a> {
    #[cfg(target_os = "linux")]
    pub fn default() -> EventLoop<'a> {

        EventLoop {
            poller: create_poller(Backend::Epoll),
            watchers: TreeMap::new(),
            events_queue: BoundedQueue::new(1 << 8)
        }
    }

    pub fn run<H: IoEventHandler>(&mut self, handler: &H) {
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
                self.process_event(io_event, handler);
            }
        }
    }

    pub fn start_io(&mut self, event: Rc<Box<AsyncIoProvider + 'a>>)
    {
        match self.events_queue.push(event) {
            Err(Full) => panic!("The event queue is full"),
            Ok(_) => ()
        }
    }

    fn process_event<H: IoEventHandler>(&'a mut self, event: &EventData, handler: &H) {
        use super::{POLL_IN, POLL_OUT};

        let fd = event.data;
        let watcher = self.watchers.get(&fd).unwrap();
        let flags = event.flags;
        watcher.handle_event(self, event, handler);

        let new_flags = watcher.poll_flags();

        if flags != new_flags {
            self.poller.modify_poll_list(fd, new_flags);
        }
    }

}

mod consts {
    pub static POLL_TIMEOUT: uint = 1000;
}
