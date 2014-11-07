pub use self::event_loop::EventLoop;
pub use self::errno::{SysCallResult, Errno};
pub use self::timer::Timer;
pub use self::notify::Notify;
pub use self::tcp::{Tcp, TcpEndpoint};

use native::io::file::fd_t;

use libc::c_int;

pub trait Pollable {
    fn poll_fd(&self) -> fd_t;

    fn poll_flags(&self) -> IoFlag;
}

pub trait AsyncOperation : Pollable {
    fn process(&self, flags: IoFlag) -> IoFlag;

    fn stop(&mut self);
}

bitflags!(
    flags IoFlag: c_int {
        const POLL_IN     = 1,
        const POLL_OUT    = 2,

        const POLL_STOP   = 3
    }
)

pub struct IoEvent {
    flags: IoFlag,

    data: c_int
}

mod event_loop;
mod backend;
mod errno;
mod timer;
mod notify;
mod tcp;
