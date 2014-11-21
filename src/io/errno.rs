use std::result;
use std::fmt;
use std::os;
use self::consts::*;

// From http://www.virtsync.com/c-error-codes-include-errno

pub struct Errno(uint);

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

    pub fn value(&self) -> uint {
        let &Errno(val) = self;

        val
    }
}

pub mod consts {
     pub const EPERM        : uint = 1;
     pub const ENOENT       : uint = 2;
     pub const ESRCH        : uint = 3;
     pub const EINTR        : uint = 4;
     pub const EIO          : uint = 5;
     pub const ENXIO        : uint = 6;
     pub const E2BIG        : uint = 7;
     pub const ENOEXEC      : uint = 8;
     pub const EBADF        : uint = 9;
     pub const ECHILD       : uint = 10;
     pub const EAGAIN       : uint = 11;
     pub const ENOMEM       : uint = 12;
     pub const EACCESS      : uint = 13;
     pub const EFAULT       : uint = 14;
     pub const ENOTBLK      : uint = 15;
     pub const EBUSY        : uint = 16;
     pub const EEXIST       : uint = 17;
     pub const EXDEV        : uint = 18;
     pub const ENODEV       : uint = 19;
     pub const ENOTDIR      : uint = 20;
     pub const EISDIR       : uint = 21;
     pub const EINVAL       : uint = 22;
     pub const ENFILE       : uint = 23;
     pub const EMFILE       : uint = 24;
     pub const ENOTTY       : uint = 25;
     pub const ETXTBSY      : uint = 26;
     pub const EFBIG        : uint = 27;
     pub const ENOSPC       : uint = 28;
     pub const ESPIPE       : uint = 29;
     pub const EROFS        : uint = 30;
     pub const EMLINK       : uint = 31;
     pub const EPIPE        : uint = 32;
     pub const EDOM         : uint = 33;
     pub const ERANGE       : uint = 34;
     pub const EDEADLK      : uint = 35;
     pub const ENAMETOOLONG : uint = 36;
     pub const ENOLCK       : uint = 37;
     pub const ENOSYS       : uint = 38;
     pub const ENOTEMPTY    : uint = 39;
     pub const ELOOP        : uint = 40;
     pub const EWOULDBLOCK  : uint = EAGAIN;
     pub const ENOMSG       : uint = 42;
     pub const EIDRM        : uint = 43;
     pub const ECHRNG       : uint = 44;
     pub const EL2NSYNC     : uint = 45;
     pub const EL3HLT       : uint = 46;
     pub const EL3RST       : uint = 47;
     pub const ELNRNG       : uint = 48;
     pub const EUNATCH      : uint = 49;
     pub const ENOCSI       : uint = 50;
     pub const EL2HLT       : uint = 51;
     pub const EBADE        : uint = 52;
     pub const EBADR        : uint = 53;
     pub const EXFULL       : uint = 54;
     pub const ENOANO       : uint = 55;
     pub const EBADRQC      : uint = 56;
     pub const EBADSLT      : uint = 57;
     pub const EDEADLOCK    : uint = EDEADLK;
     pub const EBFONT       : uint = 59;
     pub const ENOSTR       : uint = 60;
     pub const ENODATA      : uint = 61;
     pub const ETIME        : uint = 62;
     pub const ENOSR        : uint = 63;
     pub const ENONET       : uint = 64;
     pub const ENOPKG       : uint = 65;
     pub const EREMOTE      : uint = 66;
     pub const ENOLINK      : uint = 67;
     pub const EADV         : uint = 68;
     pub const ESRMNT       : uint = 69;
     pub const ECOMM        : uint = 70;
     pub const EPROTO       : uint = 71;
     pub const EMULTIHOP    : uint = 72;
     pub const EDOTDOT      : uint = 73;
     pub const EINPROGRESS  : uint = 115;
 }
