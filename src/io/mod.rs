pub use self::event_loop::EventLoop;
pub use self::errno::{SysCallResult, Errno};
pub use self::timer::Timer;
pub use self::notify::Notify;
pub use self::tcp::Tcp;

use native::io::file::fd_t;

use libc::c_int;

pub trait AsyncEvent {
    fn process(&self);

    fn poll_fd(&self) -> fd_t;

    fn stop(&mut self);
}

bitflags!(
    flags IoFlag: c_int {
        const POLL_IN     = 1,
        const POLL_OUT    = 2,

        const POLL_STOP   = 3
    }
)

mod event_loop;
mod backend;
mod errno;
mod timer;
mod notify;
mod tcp;
