use native::io::file::fd_t;
use native::io::net::sock_t;
use io::errno::{SysCallResult, Errno};
use super::{AsyncOperation, Pollable, IoFlag, POLL_IN, POLL_OUT};

use io::errno;

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
            getaddrinfo(host.to_c_str().as_ptr(),
                        service.as_ptr(),
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
            if err.value() != errno::consts::EINPROGRESS {
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
            libc::listen(self.fd, consts::LISTEN_BACKLOG as i32)
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
            let mut peer_addr = libc::sockaddr_in {
                sin_family: libc::AF_INET as libc::sa_family_t,
                sin_port: 0,
                sin_addr: unsafe { mem::zeroed() },
                sin_zero: unsafe { mem::zeroed() }
            };

            let mut peer_addr_len: libc::socklen_t = 0;

            let res = unsafe {
                libc::accept(self.fd, mem::transmute(&mut peer_addr), &mut peer_addr_len)
            };

            if res < 0 {
                let errno = Errno::current();
                match errno.value() {
                    errno::consts::EAGAIN => break,
                    _ => println!("Error when accepting connection")
                }
            }

            let name_info = unsafe {
                /* Note that we will pay the cost of extra memory allocations here since
                 * to_c_str() allocates a new buffer (creates a copy).
                 * We also pay the cost of an extra allocation when converting back the
                 * CString to String. Unforunately, this is the only way to get a *mut c_char
                 * out of a String at moment
                 */
                let mut host = String::with_capacity(consts::NI_MAXHOST);
                let mut serv = String::with_capacity(consts::NI_MAXSERV);

                let mut c_host = host.to_c_str();
                let mut c_serv = host.to_c_str();

                let res = getnameinfo(mem::transmute(&peer_addr), peer_addr_len,
                            c_host.as_mut_ptr(),
                            consts::NI_MAXHOST as libc::size_t,
                            c_serv.as_mut_ptr(),
                            consts::NI_MAXSERV as libc::size_t,
                            NI_NAMEREQD.bits());

                (res, c_host.as_str().unwrap().to_string(),
                      c_serv.as_str().unwrap().to_string())
            };

            let (_, host_name, serv_name) = name_info;

            let peer_info = unsafe {
                let mut peer_name = String::with_capacity(consts::INET_ADDRSTRLEN);
                let mut c_peer_name = peer_name.to_c_str();

                let res = inet_ntop(libc::AF_INET as libc::c_int,
                                    &peer_addr.sin_addr as *const libc::in_addr as *const libc::c_void,
                                    c_peer_name.as_mut_ptr(),
                                    consts::INET_ADDRSTRLEN as libc::socklen_t);

                (res, c_peer_name.as_str().unwrap().to_string())
            };

            let (_, peer_name) = peer_info;

            println!("peer_name -> {}", peer_name);
            println!("host_name -> {}", host_name);
            println!("serv_name -> {}", serv_name);

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

mod consts {
    pub const LISTEN_BACKLOG  : uint = 1 << 4;

    pub const NI_MAXHOST      : uint = 1025;
    pub const NI_MAXSERV      : uint = 32;

    pub const INET_ADDRSTRLEN : uint = 16;
}
