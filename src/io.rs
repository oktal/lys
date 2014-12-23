use std::option::Option;
use std::bitflags;
use std::ptr;
use std::mem;
use std::collections::HashMap;

use libc;

pub type fd_t = libc::c_int;

extern {
    fn epoll_create(size: libc::c_int) -> libc::c_int;
    fn epoll_ctl(epfd: libc::c_int, op: libc::c_int, fd: libc::c_int, event: *const EpollEvent) -> libc::c_int;
    fn epoll_wait(epfd: libc::c_int, events: *mut EpollEvent, max_events: libc::c_int, timeout: libc::c_int) -> libc::c_int;

}

bitflags!(
    flags IoFlag: libc::c_int {
        const POLL_IN     = 0x1,
        const POLL_OUT    = 0x2,

        const POLL_STOP   = 0x3
    }
);

pub enum Event<'a> {
    TcpConnection,
    TcpIn(Vec<u8>),
}

pub trait EventHandler {
    fn handle_event(&self, event: Event);
}

pub trait Pollable {
    fn poll_fd(&self) -> fd_t;

    fn poll_flags(&self) -> IoFlag;
}

pub trait AsyncIoProvider : Pollable {
    fn process_event(&self, flags: IoFlag, handler: &EventHandler);
}

pub struct IoEvent {
    fd: fd_t,
    flags: IoFlag
}


pub trait Poller {
    fn add(&mut self, fd: fd_t, flags: IoFlag) -> libc::c_int;
    fn modify(&mut self, fd: fd_t, flags: IoFlag) -> libc::c_int;
    fn remove(&mut self, fd: fd_t) -> libc::c_int;

    fn poll(&self, timeout_ms: i32) -> Option<Vec<IoEvent>>;
}

pub trait FromIoFlags {
    fn from_io_flags(flags: IoFlag) -> Self;
}

pub trait ToIoFlags {
   fn to_io_flags(&self) -> IoFlag;
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
    pub data: libc::c_int
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
);

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

struct Epoll {
    fd: fd_t
}

impl Epoll {
     pub fn new(size: i32) -> Option<Epoll> {
        let fd = unsafe { epoll_create(size) };

        if fd < 0 {
            return None;
        }

        Some(Epoll { fd: fd })
    }
}

impl Poller for Epoll {
    fn add(&mut self, fd: fd_t, flags: IoFlag) -> libc::c_int {
        let event = EpollEvent {
            events: FromIoFlags::from_io_flags(flags),
            data: fd as libc::c_int
        };

        let res = unsafe {
            epoll_ctl(self.fd, EpollControl::Add as libc::c_int, fd, &event as *const EpollEvent)
        };

        res
    }

    fn modify(&mut self, fd: fd_t, flags: IoFlag) -> libc::c_int {
        let event = EpollEvent {
            events: FromIoFlags::from_io_flags(flags),
            data: 0 as libc::c_int
        };

        let res = unsafe {
            epoll_ctl(self.fd, EpollControl::Mod as libc::c_int, fd, &event as *const EpollEvent)
        };

        res
    }

    fn remove(&mut self, fd: fd_t) -> libc::c_int {
        let res = unsafe {
            epoll_ctl(self.fd, EpollControl::Del as libc::c_int, fd, ptr::null())
        };

        res
    }

    fn poll(&self, timeout_ms: i32) -> Option<Vec<IoEvent>> {
        let mut buf: [EpollEvent, ..256] = unsafe { mem::uninitialized() };

        let res = unsafe {
            epoll_wait(self.fd, buf.as_mut_ptr(), buf.len() as libc::c_int, timeout_ms)
        };

        if res < 0 {
            return None;
        }

        let count = res as uint;
        let mut events = Vec::with_capacity(count);
    
        for event in buf.iter().take(count) {
            events.push(IoEvent {
                fd: event.data,
                flags: event.events.to_io_flags()
            });
        }

        Some(events)
    }
}

pub mod tcp {
    use libc;
    use std::mem;

    use super::{
        IoFlag, fd_t, POLL_IN, Pollable, AsyncIoProvider,
        EventHandler, Event
    };

    extern {
        fn inet_pton(af: libc::c_int, src: *const libc::c_char, dst: *mut libc::c_void)
            -> libc::c_int;
        fn htons(hostshort: u16) -> u16;
    }

    bitflags!(
    flags SocketFlag: libc::c_int {
        const SOCK_CLOEXEC   = 0o2000000,
        const SOCK_NONBLOCK  = 0o0004000
    }
    );

    fn create_socket() -> Option<fd_t> {
        let fd = unsafe {
            libc::socket(libc::AF_INET, libc::SOCK_STREAM | SOCK_NONBLOCK.bits(), 0)
        };

        if fd < 0 {
            return None;
        }

        Some(fd as fd_t)
    }

    pub struct Endpoint {
        fd: fd_t,

        flags: IoFlag
    }

    impl Endpoint {
       pub fn bind(host: &str, port: u16) -> Option<Endpoint> {
           let sock_fd = create_socket().unwrap();
            let mut addr = libc::sockaddr_in {
                sin_family: libc::AF_INET as libc::sa_family_t,
                sin_port: unsafe { htons(port) },
                sin_addr: unsafe { mem::zeroed() },
                sin_zero: unsafe { mem::zeroed() }
            };

            let res = unsafe {
                let c_host = host.to_c_str();
                inet_pton(libc::AF_INET, c_host.as_ptr() as *const libc::c_char, mem::transmute(&addr.sin_addr))
            };

            if res < 0 {
                return None;
            }

            let res_bind = unsafe {
                libc::bind(sock_fd, mem::transmute(&addr),
                           mem::size_of::<libc::sockaddr_in>() as libc::socklen_t)
            };

            if res < 0 {
                return None;
            }

            Some(Endpoint {
                fd: sock_fd,
                flags: POLL_IN
            })
       }

       pub fn listen(&mut self) -> libc::c_int {
           let res = unsafe {
               libc::listen(self.fd, 128 as i32)
           };

           res
       }

       pub fn accept<'a>(&'a self) -> IncomingConnections<'a> {
           IncomingConnections { endpoint: self }
       }
    }

    impl Pollable for Endpoint {
        fn poll_fd(&self) -> fd_t { self.fd }

        fn poll_flags(&self) -> IoFlag { self.flags }
    }

    impl AsyncIoProvider for Endpoint {
        fn process_event(&self, flags: IoFlag, handler: &EventHandler) {
            handler.handle_event(super::Event::TcpConnection);
        }
    }

    pub struct Socket {
        fd: fd_t,
        flags: IoFlag
    }

    impl Pollable for Socket {
        fn poll_fd(&self) -> fd_t { self.fd }

        fn poll_flags(&self) -> IoFlag { self.flags }
    }

    impl AsyncIoProvider for Socket {
        fn process_event(&self, flags: IoFlag, handler: &EventHandler) {
            println!("Data has been received in the socket fd -> {}", self.fd);
        }
    }

    pub struct IncomingConnections<'a> {
        endpoint: &'a Endpoint
    }

    impl<'a> Iterator<Socket> for IncomingConnections<'a> {
        fn next(&mut self) -> Option<Socket> {
            let mut peer_addr = libc::sockaddr_in {
                sin_family: libc::AF_INET as libc::sa_family_t,
                sin_port: 0,
                sin_addr: unsafe { mem::uninitialized() },
                sin_zero: unsafe { mem::zeroed() }
            };

            let mut peer_addr_len = 0 as libc::socklen_t;
            
            let res = unsafe {
                libc::accept(self.endpoint.fd, mem::transmute(&mut peer_addr), &mut peer_addr_len)
            };

            if res < 0 {
                return None;
            }

            println!("Incoming connection has been accepted !");
            Some(Socket { fd: res, flags: POLL_IN })
        }
    }


}

pub struct EventLoop<'a> {
    providers: HashMap<fd_t, &'a (AsyncIoProvider + 'a)>,

    poller: Box<Poller + 'static>
}

impl<'a> EventLoop<'a> {
    pub fn new() -> EventLoop<'a> {
        let epoller = box Epoll::new(1024).unwrap();

        EventLoop {
            providers: HashMap::new(),
            poller: epoller
        }
    }

    pub fn join(&mut self, provider: &'a (AsyncIoProvider + 'a)) {
        let poll_fd = provider.poll_fd();
        let poll_flags = provider.poll_flags();
        println!("Starting polling on fd -> {}", poll_fd);

        self.poller.add(poll_fd, poll_flags);
        self.providers.insert(poll_fd, provider);
    }

    pub fn run<H: EventHandler>(&self, handler: &H) {
        loop {
            let events = self.poller.poll(0).unwrap();

            for event in events.iter() {
                self.process_event(event, handler);
            }
        }
    }

    fn process_event<H: EventHandler>(&self, event: &IoEvent, handler: &H) {
        let fd = event.fd;

        match self.providers.get(&fd) {
            Some(provider) => provider.process_event(event.flags, handler),
            None => panic!("Unknown provider for fd {}", fd)
        };

    }
}

