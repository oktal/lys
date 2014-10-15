use std::result;
use std::fmt;
use std::os;
use self::consts::*;

// From http://www.virtsync.com/c-error-codes-include-errno

pub struct Errno(int);

pub type SysCallResult<T> = result::Result<T, Errno>;

impl fmt::Show for Errno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Errno(val) = self;

        let desc = match val {
             EPERM         => "Operation not permitted",
             ENOENT        => "No such file or directory",
             ESRCH         => "No such process",
             EINTR         => "Interrupted system call",
             EIO           => "I/O error",
             ENXIO         => "No such device or address",
             E2BIG         => "Argument list too long",
             ENOEXEC       => "Exec format error",
             EBADF         => "Bad file number",
             ECHILD        => "No child processes",
             EAGAIN        => "Try again",
             ENOMEM        => "Out of memory",
             EACCESS       => "Permission denied",
             EFAULT        => "Bad address",
             ENOTBLK       => "Block device required",
             EBUSY         => "Device or resource busy",
             EEXIST        => "File exists",
             EXDEV         => "Cross-device link",
             ENODEV        => "No such device",
             ENOTDIR       => "Not a directory",
             EISDIR        => "Is a directory",
             EINVAL        => "Invalid argument",
             ENFILE        => "File table overflow",
             EMFILE        => "Too many open files",
             ENOTTY        => "Not a typewriter",
             ETXTBSY       => "Text file busy",
             EFBIG         => "File too large",
             ENOSPC        => "No space left on device",
             ESPIPE        => "Illegal seek",
             EROFS         => "Read-only file system",
             EMLINK        => "Too many links",
             EPIPE         => "Broken pipe",
             EDOM          => "Math argument out of domain of func",
             ERANGE        => "Math result not representable",
             _             => "Unknown errno code"
         };

         write!(f, "{}", desc)
    }
}

impl Errno {
    pub fn current() -> Errno {
         Errno(os::errno())
     }

    pub fn value(&self) -> int {
        let &Errno(val) = self;

        val
    }
}

pub mod consts {
     pub const EPERM   : int = 1;
     pub const ENOENT  : int = 2;
     pub const ESRCH   : int = 3;
     pub const EINTR   : int = 4;
     pub const EIO     : int = 5;
     pub const ENXIO   : int = 6;
     pub const E2BIG   : int = 7;
     pub const ENOEXEC : int = 8;
     pub const EBADF   : int = 9;
     pub const ECHILD  : int = 10;
     pub const EAGAIN  : int = 11;
     pub const ENOMEM  : int = 12;
     pub const EACCESS : int = 13;
     pub const EFAULT  : int = 14;
     pub const ENOTBLK : int = 15;
     pub const EBUSY   : int = 16;
     pub const EEXIST  : int = 17;
     pub const EXDEV   : int = 18;
     pub const ENODEV  : int = 19;
     pub const ENOTDIR : int = 20;
     pub const EISDIR  : int = 21;
     pub const EINVAL  : int = 22;
     pub const ENFILE  : int = 23;
     pub const EMFILE  : int = 24;
     pub const ENOTTY  : int = 25;
     pub const ETXTBSY : int = 26;
     pub const EFBIG   : int = 27;
     pub const ENOSPC  : int = 28;
     pub const ESPIPE  : int = 29;
     pub const EROFS   : int = 30;
     pub const EMLINK  : int = 31;
     pub const EPIPE   : int = 32;
     pub const EDOM    : int = 33;
     pub const ERANGE  : int = 34;
 }
