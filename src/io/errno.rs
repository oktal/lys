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
             EDEADLK       => "Resource deadlock would occur",
             ENAMETOOLONG  => "File name too long",
             ENOLCK        => "No record locks available",
             ENOSYS        => "Function not implemented",
             ENOTEMPTY     => "Directory not empty",
             ELOOP         => "Too many symbolic links encountered",
             ENOMSG        => "No message of desired type",
             EIDRM         => "Identifier removed",
             ECHRNG        => "Channel number out of range",
             EL2NSYNC      => "Level 2 not synchronized",
             EL3HLT        => "Level 3 halted",
             EL3RST        => "Level 3 reset",
             ELNRNG        => "Link number out of range",
             EUNATCH       => "Protocol driver not attached",
             ENOCSI        => "No CSI structure available",
             EL2HLT        => "Level 2 halted",
             EBADE         => "Invalid exchange",
             EBADR         => "Invalid request descriptor",
             EXFULL        => "Exchange full",
             ENOANO        => "No anode",
             EBADRQC       => "Invalid request code",
             EBADSLT       => "Invalid slot",
             EBFONT        => "Bad font file format",
             ENOSTR        => "Device not a stream",
             ENODATA       => "No data available",
             ETIME         => "Timer expired",
             ENOSR         => "Out of streams resources",
             ENONET        => "Machine is not on the network",
             ENOPKG        => "Package not installed",
             EREMOTE       => "Object is remote",
             ENOLINK       => "Link has been severed",
             EADV          => "Advertise error",
             ESRMNT        => "Srmount error",
             ECOMM         => "Communication error on send",
             EPROTO        => "Protocol error",
             EMULTIHOP     => "Multihop attempted",
             EDOTDOT       => "RFS specific error",
             EINPROGRESS   => "Operation now in progress",
             _             => "Unknown errno code"
         };

         write!(f, "{}: {}", val, desc)
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
     pub const EPERM        : int = 1;
     pub const ENOENT       : int = 2;
     pub const ESRCH        : int = 3;
     pub const EINTR        : int = 4;
     pub const EIO          : int = 5;
     pub const ENXIO        : int = 6;
     pub const E2BIG        : int = 7;
     pub const ENOEXEC      : int = 8;
     pub const EBADF        : int = 9;
     pub const ECHILD       : int = 10;
     pub const EAGAIN       : int = 11;
     pub const ENOMEM       : int = 12;
     pub const EACCESS      : int = 13;
     pub const EFAULT       : int = 14;
     pub const ENOTBLK      : int = 15;
     pub const EBUSY        : int = 16;
     pub const EEXIST       : int = 17;
     pub const EXDEV        : int = 18;
     pub const ENODEV       : int = 19;
     pub const ENOTDIR      : int = 20;
     pub const EISDIR       : int = 21;
     pub const EINVAL       : int = 22;
     pub const ENFILE       : int = 23;
     pub const EMFILE       : int = 24;
     pub const ENOTTY       : int = 25;
     pub const ETXTBSY      : int = 26;
     pub const EFBIG        : int = 27;
     pub const ENOSPC       : int = 28;
     pub const ESPIPE       : int = 29;
     pub const EROFS        : int = 30;
     pub const EMLINK       : int = 31;
     pub const EPIPE        : int = 32;
     pub const EDOM         : int = 33;
     pub const ERANGE       : int = 34;
     pub const EDEADLK      : int = 35;
     pub const ENAMETOOLONG : int = 36;
     pub const ENOLCK       : int = 37;
     pub const ENOSYS       : int = 38;
     pub const ENOTEMPTY    : int = 39;
     pub const ELOOP        : int = 40;
     pub const EWOULDBLOCK  : int = EAGAIN;
     pub const ENOMSG       : int = 42;
     pub const EIDRM        : int = 43;
     pub const ECHRNG       : int = 44;
     pub const EL2NSYNC     : int = 45;
     pub const EL3HLT       : int = 46;
     pub const EL3RST       : int = 47;
     pub const ELNRNG       : int = 48;
     pub const EUNATCH      : int = 49;
     pub const ENOCSI       : int = 50;
     pub const EL2HLT       : int = 51;
     pub const EBADE        : int = 52;
     pub const EBADR        : int = 53;
     pub const EXFULL       : int = 54;
     pub const ENOANO       : int = 55;
     pub const EBADRQC      : int = 56;
     pub const EBADSLT      : int = 57;
     pub const EDEADLOCK    : int = EDEADLK;
     pub const EBFONT       : int = 59;
     pub const ENOSTR       : int = 60;
     pub const ENODATA      : int = 61;
     pub const ETIME        : int = 62;
     pub const ENOSR        : int = 63;
     pub const ENONET       : int = 64;
     pub const ENOPKG       : int = 65;
     pub const EREMOTE      : int = 66;
     pub const ENOLINK      : int = 67;
     pub const EADV         : int = 68;
     pub const ESRMNT       : int = 69;
     pub const ECOMM        : int = 70;
     pub const EPROTO       : int = 71;
     pub const EMULTIHOP    : int = 72;
     pub const EDOTDOT      : int = 73;
     pub const EINPROGRESS  : int = 115;
 }
