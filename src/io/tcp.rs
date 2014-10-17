use native::io::file::fd_t;
use native::io::net::sock_t;
use io::errno::{SysCallResult, Errno, consts};
use super::{AsyncEvent, IoFlag, POLL_IN, POLL_OUT};

use libc;

use std::io::net::ip;
use std::mem;

bitflags!(
    flags TimerFdFlag: libc::c_int {
        const SOCK_CLOEXEC   = 0o2000000,
        const SOCK_NONBLOCK  = 0o0004000
    }
)

pub struct Tcp {
    fd: sock_t,

    events: IoFlag
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

fn create_socket(addr: ip::SocketAddr) -> SysCallResult<sock_t> {
    unsafe {
        let family = match addr.ip {
            ip::Ipv4Addr(..) => libc::AF_INET,
            ip::Ipv6Addr(..) => libc::AF_INET6
        };

        let fd = libc::socket(family, libc::SOCK_STREAM | SOCK_NONBLOCK.bits(), 0);

        if fd < 0 {
            return Err(Errno::current());
        }

        Ok(fd)
    }
}


impl Tcp {
    pub fn connect(addr: ip::SocketAddr) -> SysCallResult<Tcp> {
        let sock_fd = try!(create_socket(addr));

        let family = match addr.ip {
            ip::Ipv4Addr(..) => libc::AF_INET,
            ip::Ipv6Addr(..) => libc::AF_INET6
        };

        let s_addr = ipaddr_to_inaddr(addr.ip);

        let serv_addr = libc::sockaddr_in {
            sin_family: family as u16,
            sin_port: addr.port,
            sin_addr: s_addr,
            sin_zero: unsafe { mem::zeroed() }
        };

        let res = unsafe {
            libc::connect(sock_fd, mem::transmute(&serv_addr),
                          mem::size_of::<libc::sockaddr_in>() as libc::socklen_t)
        };

        if res < 0 {
            let err = Errno::current();

            return Err(err);
        }

        Ok(Tcp {
            fd: sock_fd,
            events: POLL_IN | POLL_OUT
        })

    }
}
