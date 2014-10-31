pub use self::poller::Poller;
pub use self::epoll::Epoll;
pub use self::epoll::EpollEventKind;

use io::IoFlag;

trait ToIoFlags {
    fn to_io_flags(&self) -> IoFlag;
}

trait FromIoFlags {
    fn from_io_flags(flags: IoFlag) -> Self;
}

pub mod poller;
pub mod epoll;
