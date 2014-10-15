pub use self::event_loop::EventLoop;
pub use self::errno::{SysCallResult, Errno};
pub use self::timer::Timer;
pub use self::notify::Notify;

pub trait AsyncEvent {
    fn process(&self);
}

mod event_loop;
mod backend;
mod errno;
mod timer;
mod notify;
