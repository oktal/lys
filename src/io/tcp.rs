use native::io::file::fd_t;
use native::io::net::sock_t;
use io::errno::{SysCallResult, Errno, consts};
use super::{AsyncOperation, Pollable, IoFlag, POLL_IN, POLL_OUT};

use libc;

use std::io::net::ip;
use native::io::net::htons;
use std::mem;
use std::ptr;
use std::c_str::CString;
use std::cell::Cell;

extern {
    fn getaddrinfo(node: *const libc::c_char, service: *const libc::c_char,
                   hints: *const libc::addrinfo, res: *mut *mut libc::addrinfo) -> libc::c_int;

    fn gai_strerror(errorcode: libc::c_int) -> *const libc::c_char;

    fn inet_pton(af: libc::c_int, src: *const libc::c_char, dst: *mut libc::c_void)
        -> libc::c_int;

    fn inet_ntop(af: libc::c_int, src: *const libc::c_void, dst: *mut libc::c_char,
                 size: libc::socklen_t) -> *const libc::c_char;

    fn getnameinfo(sa: *const libc::sockaddr, salen: libc::socklen_t,
                   host: *mut libc::c_char, hostlen: libc::size_t,
                   serv: *mut libc::c_char, servlen: libc::size_t,
                   flags: libc::c_int) -> libc::c_int;

}

bitflags!(
    flags NameInfoFlags: libc::c_int {
        const NI_DGRAM         = 0x0001,
        const NI_NAMEREQD      = 0x0002,
        const NI_NOFQDN        = 0x0004,
        const NI_NUMERICHOST   = 0x0008,
        const NI_NUMERICSCOPE  = 0x0010,
        const NI_NUMERICSERV   = 0x0020
    }
)

bitflags!(
    flags TimerFdFlag: libc::c_int {
        const SOCK_CLOEXEC   = 0o2000000,
        const SOCK_NONBLOCK  = 0o0004000
    }
)

pub type OnConnect = fn(tcp: &Tcp);
pub type OnNewConnection = fn(endpoint: &TcpEndpoint);

pub struct Tcp {
    callback: OnConnect,
    fd: sock_t,

    events: Cell<IoFlag>
}

//TODO: ipv6
fn ipaddr_to_inaddr(ipaddr: ip::IpAddr) -> libc::in_addr {
    let s_addr = match ipaddr {
        ip::Ipv4Addr(a, b, c, d) =>
            (a as u32 << 24) | (b  as u32 << 16) | (c as u32 << 8) | d as u32,
        _ => unimplemented!()
    };

    libc::in_addr { s_addr: s_addr }
}

fn create_socket() -> SysCallResult<sock_t> {
    unsafe {
        let fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM | SOCK_NONBLOCK.bits(), 0);

        if fd < 0 {
            return Err(Errno::current());
        }

        Ok(fd)
    }
}


impl Tcp {
    pub fn connect(host: &str, port: u16, callback: OnConnect) -> SysCallResult<Tcp> {

        let hint = libc::addrinfo {
            ai_flags: 0,
            ai_family: libc::AF_INET,
            ai_socktype: libc::SOCK_STREAM,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: ptr::null_mut(),
            ai_canonname: ptr::null_mut(),
            ai_next: ptr::null_mut()
        };

        let result: *mut libc::addrinfo = ptr::null_mut();
        let res = unsafe {
            let service = "http".to_c_str();
            getaddrinfo(host.to_c_str().as_ptr() as *const libc::c_char,
                        service.as_ptr() as *const libc::c_char,
                        &hint as *const libc::addrinfo, mem::transmute(&result))
        };

        if res < 0 {
            return Err(Errno::current());
        }

        let sock_fd = try!(create_socket());

        let res_connect = unsafe {
            libc::connect(sock_fd, (*result).ai_addr as *const libc::sockaddr,
                          (*result).ai_addrlen as libc::socklen_t)
        };

        if res_connect < 0 {
            let err = Errno::current();
            if err.value() != consts::EINPROGRESS {
                return Err(err);
            }
        }

        Ok(Tcp {
            callback: callback,
            fd: sock_fd,
            events: Cell::new(POLL_IN | POLL_OUT)
        })

    }

}

pub struct TcpEndpoint {
    fd: fd_t,
    events: IoFlag,

    on_connection: OnNewConnection
}

impl TcpEndpoint {
    pub fn bind(host: &str, port: u16, on_connection: OnNewConnection)
        -> SysCallResult<TcpEndpoint> {

        let sock_fd = try!(create_socket());

        let mut addr = libc::sockaddr_in {
            sin_family: libc::AF_INET as libc::sa_family_t,
            sin_port: htons(port),
            sin_addr: unsafe { mem::zeroed() },
            sin_zero: unsafe { mem::zeroed() }
        };

        let res = unsafe {
            let c_host = host.to_c_str();
            inet_pton(libc::AF_INET, c_host.as_ptr() as *const libc::c_char,
                      mem::transmute(&addr.sin_addr))
        };

        if res < 0 {
            return Err(Errno::current());
        }

        let res_bind = unsafe {
            libc::bind(sock_fd, mem::transmute(&addr),
                       mem::size_of::<libc::sockaddr_in>() as libc::socklen_t)
        };

        if res_bind < 0 {
            return Err(Errno::current());
        }

        Ok(TcpEndpoint {
            fd: sock_fd,
            events: POLL_IN,
            on_connection: on_connection
        })
    }

    pub fn listen(&self) -> SysCallResult<()> {
        let res = unsafe {
            libc::listen(self.fd, default::LISTEN_BACKLOG as i32)
        };

        if res < 0 {
            return Err(Errno::current());
        }

        Ok( () )
    }

}

impl AsyncOperation for Tcp {
    fn process(&self, flags: IoFlag) -> IoFlag {
        (self.callback)(self);
        let mut events = self.events.get();
        if events.contains(POLL_OUT) {
            events.remove(POLL_OUT);
            self.events.set(events);
        }

        self.events.get()
    }

    fn stop(&mut self) { unsafe { libc::close(self.fd) }; }

}

impl Pollable for Tcp {
    fn poll_fd(&self) -> fd_t { self.fd }

    fn poll_flags(&self) -> IoFlag { self.events.get() }
}

impl AsyncOperation for TcpEndpoint {
    fn process(&self, flags: IoFlag) -> IoFlag {
        loop {
            let mut in_addr = libc::sockaddr {
                sa_family: libc::AF_INET as libc::sa_family_t,
                sa_data: unsafe { mem::zeroed() }
            };

            let mut in_len: libc::socklen_t = 0;

            let res = unsafe {
                libc::accept(self.fd, mem::transmute(&in_addr), mem::transmute(&in_len))
            };

            if res < 0 {
                let errno = Errno::current();
                match errno.value() {
                    consts::EAGAIN => break,
                    _ => println!("Error when accepting connection")
                }
            }

            let mut host = String::with_capacity(default::NI_MAXHOST);
            let mut serv = String::with_capacity(default::NI_MAXSERV);

            let res_ninfo = unsafe {
                let mut c_host = host.to_c_str();
                let mut c_serv = host.to_c_str();

                getnameinfo(mem::transmute(&in_addr), in_len,
                            c_host.as_mut_ptr() as *mut libc::c_char,
                            default::NI_MAXHOST as libc::size_t,
                            c_serv.as_mut_ptr() as *mut libc::c_char,
                            default::NI_MAXSERV as libc::size_t,
                            (NI_NUMERICHOST | NI_NUMERICSERV).bits())
            };

            let mut peer_name = String::with_capacity(default::NI_MAXHOST);

            let ntop_res = unsafe {
                let mut c_peer_name = peer_name.to_c_str();
                inet_ntop(libc::AF_INET as libc::c_int, mem::transmute(&in_addr),
                          c_peer_name.as_mut_ptr() as *mut libc::c_char, default::NI_MAXHOST as
                          libc::socklen_t)
            };

            if ntop_res == ptr::null() {
                println!("Hue");
            }

            println!("peer_name -> {}", peer_name);
            println!("host -> {}", host);
            println!("serv -> {}", serv);

            (self.on_connection)(self);
        }

        self.events
    }

    fn stop(&mut self) { }
}

impl Pollable for TcpEndpoint {
    fn poll_fd(&self) -> fd_t { self.fd }

    fn poll_flags(&self) -> IoFlag { self.events }
}

mod default {
    pub const LISTEN_BACKLOG: uint = 1 << 4;

    pub const NI_MAXHOST    : uint = 1025;
    pub const NI_MAXSERV    : uint = 32;
}
