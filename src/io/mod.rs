pub use self::event_loop::EventLoop;
pub use self::errno::{SysCallResult, Errno};
pub use self::timer::Timer;
pub use self::notify::Notify;

use libc::c_int;

pub type fd_t = c_int;
pub type sock_t = c_int;

pub trait Pollable {
    fn poll_fd(&self) -> fd_t;

    fn poll_flags(&self) -> IoFlag;
}

pub enum IoEvent {
    Notify,
    Timer(u64),
    TcpConnection,
    In(Vec<u8>)
}

pub trait AsyncIoProvider : Pollable {
    fn handle_event<'a>(&self, ev_loop: &'a mut EventLoop<'a>,
                        data: &EventData, handler: &IoEventHandler);
}

pub trait IoEventHandler {
    fn handle_event<'a>(&self, ev_loop: &'a mut EventLoop<'a>, io_event: IoEvent);
}

bitflags!(
    #[deriving(Copy)]
    flags IoFlag: c_int {
        const POLL_IN     = 1,
        const POLL_OUT    = 2,

        const POLL_STOP   = 3
    }
)

pub struct EventData {
    flags: IoFlag,
    data: c_int
}

impl EventData {
    pub fn is_readable(&self) -> bool {
        self.flags.contains(POLL_IN)
    }

    pub fn is_writable(&self) -> bool {
        self.flags.contains(POLL_OUT)
    }
}

mod event_loop;
mod backend;
mod errno;
mod timer;
mod notify;
pub mod tcp;
