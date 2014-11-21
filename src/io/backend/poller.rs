use io::{IoFlag, IoEvent};
use io::errno::SysCallResult;
use io::fd_t;
use libc;

pub trait Poller {
    
    fn add_poll_list(&mut self, fd: fd_t, flags: IoFlag) -> SysCallResult<()>;
    fn modify_poll_list(&mut self, fd: fd_t, flags: IoFlag) -> SysCallResult<()>;
    fn remove_poll_list(&mut self, fd: fd_t) -> SysCallResult<()>;

    fn poll(&self, timeout_ms: uint) -> SysCallResult<Vec<IoEvent>>;
}
