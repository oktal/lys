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
    pub fn value() -> Errno {
         Errno(os::errno())
     }
}

mod consts {
     pub static EPERM   : int = 1;
     pub static ENOENT  : int = 2;
     pub static ESRCH   : int = 3;
     pub static EINTR   : int = 4;
     pub static EIO     : int = 5;
     pub static ENXIO   : int = 6;
     pub static E2BIG   : int = 7;
     pub static ENOEXEC : int = 8;
     pub static EBADF   : int = 9;
     pub static ECHILD  : int = 10;
     pub static EAGAIN  : int = 11;
     pub static ENOMEM  : int = 12;
     pub static EACCESS : int = 13;
     pub static EFAULT  : int = 14;
     pub static ENOTBLK : int = 15;
     pub static EBUSY   : int = 16;
     pub static EEXIST  : int = 17;
     pub static EXDEV   : int = 18;
     pub static ENODEV  : int = 19;
     pub static ENOTDIR : int = 20;
     pub static EISDIR  : int = 21;
     pub static EINVAL  : int = 22;
     pub static ENFILE  : int = 23;
     pub static EMFILE  : int = 24;
     pub static ENOTTY  : int = 25;
     pub static ETXTBSY : int = 26;
     pub static EFBIG   : int = 27;
     pub static ENOSPC  : int = 28;
     pub static ESPIPE  : int = 29;
     pub static EROFS   : int = 30;
     pub static EMLINK  : int = 31;
     pub static EPIPE   : int = 32;
     pub static EDOM    : int = 33;
     pub static ERANGE  : int = 34;
 }
