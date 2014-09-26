static EPOLL_CLOEXEC : uint 0x80000;
static EPOLL_NONBLOCK = uint 0x800;

static EPOLLIN          : uint 0x001;
static EPOLLPRI         : uint 0x002;
static EPOLLOUT         : uint 0x004;
static EPOLLERR         : uint 0x008;
static EPOLLHUP         : uint 0x010;

static EPOLLRDNORM      : uint 0x040;
static EPOLLRDBAND      : uint 0x080;
static EPOLLWRNORM      : uint 0x100;
static EPOLLWRBAND      : uint 0x200;
static EPOLLMSG         : uint 0x400;
static EPOLLRDHUP       : uint 0x2000;
static EPOLLONESHOT     : uint 0x20000000;
static EPOLLET          : uint 0x40000000;

static EPOLL_CTL_ADD : uint 1; // Add a file descriptor to the interface.
static EPOLL_CTL_DEL : uint 2; // Remove a file descriptor from the interface.
static EPOLL_CTL_MOD : uint 3; // Change file descriptor structure.

impl epoll {

    
}  

