use events::backend::epoll;
use events::libc::c_void;
use events::timer;

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

pub struct EventLoop {
    pub poller: epoll::Epoll,
}

impl EventLoop {
    #[cfg(target_os = "linux")]
    pub fn default() -> EventLoop {
        let poller = epoll::Epoll::new(1 << 16);

        EventLoop { poller: poller }
    }

    pub fn run(&self) {
        loop {
            let mut events: [epoll::EpollEvent, ..1] =
            [ epoll::EpollEvent { events: epoll::EPOLLIN, data: 0 as *mut c_void }];

            let readyCount = self.poller.poll(events, 2000);
            if readyCount > 0 {
                for i in range(0, readyCount as uint) {
                    if events[i].events.contains(epoll::EPOLLIN) {
                        println!("Ready for POLLIN");

                        /* Does not compile 
                        unsafe {
                            let timer_ev = events[i].data as *const timer::Timer;
                            timer_ev.process();
                        }
                        */
                    }
                }
            }
        }
    }
}
