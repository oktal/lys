pub use self::event_loop::EventLoop;
pub use self::errno::{SysCallResult, Errno};
pub use self::timer::Timer;
pub use self::notify::Notify;

use native::io::file::fd_t;

pub trait AsyncEvent {
    fn process(&self);

    fn poll_fd(&self) -> fd_t;
}

mod event_loop;
mod backend;
mod errno;
mod timer;
mod notify;
