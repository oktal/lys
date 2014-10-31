use native::io::file::fd_t;
use native::io::net::sock_t;
use io::errno::{SysCallResult, Errno, consts};
use super::{AsyncEvent, IoFlag, POLL_IN, POLL_OUT};

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
}

bitflags!(
    flags TimerFdFlag: libc::c_int {
        const SOCK_CLOEXEC   = 0o2000000,
        const SOCK_NONBLOCK  = 0o0004000
    }
)

pub type OnConnect = fn(tcp: &Tcp);

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

impl AsyncEvent for Tcp {
    fn process(&self) {
        (self.callback)(self);
        let mut events = self.events.get();
        if events.contains(POLL_OUT) {
            events.remove(POLL_OUT);
            self.events.set(events);
        }
    }

    fn poll_fd(&self) -> fd_t { self.fd }

    fn stop(&mut self) { unsafe { libc::close(self.fd) }; }

    fn flags(&self) -> IoFlag { self.events.get() }
}
