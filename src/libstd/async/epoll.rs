extern crate libc;

pub mod consts {

    pub mod os {

        #[cfg(target_os = "linux")]
        pub mod epoll {
            static EPOLL_CLOEXEC    : uint = 0x80000;
            static EPOLL_NONBLOCK   : uint = 0x800;

            static EPOLLIN          : uint = 0x001; // Available for read
            static EPOLLPRI         : uint = 0x002; // Urgent data for read operation
            static EPOLLOUT         : uint = 0x004; // Available for write
            static EPOLLERR         : uint = 0x008; // Error condition triggered for FD
            static EPOLLHUP         : uint = 0x010; // Hang up on FD

            static EPOLLRDNORM      : uint = 0x040;
            static EPOLLRDBAND      : uint = 0x080;
            static EPOLLWRNORM      : uint = 0x100;
            static EPOLLWRBAND      : uint = 0x200;
            static EPOLLMSG         : uint = 0x400;
            static EPOLLRDHUP       : uint = 0x2000; // Since Linux 2.6.17
                                                     // Stream socket peer closed connection.
            static EPOLLONESHOT     : uint = 0x20000000;
            static EPOLLET          : uint = 0x40000000;

            static EPOLL_CTL_ADD : uint = 1; // Add a file descriptor to the interface.
            static EPOLL_CTL_DEL : uint = 2; // Remove a file descriptor from the interface.
            static EPOLL_CTL_MOD : uint = 3; // Change file descriptor structure.
        }

        //#[cfg(target_os = "bsd")]
        pub mod kqueue {
        } 

    }

}  

pub mod async {

    struct AsyncChannel<T> {
        isConnected     :   bool,
        #[cfg[(target_os = "linux")]]
        channel         :   *const epoll_data_t,
        write_cb        :   int,
        read_cb         :   int,
    }

    impl AsyncChannel {

        pub fn new() -> AsyncChannel {
            AsyncChannel { isConnected:false }
        }
        
        pub fn connect() {
            if
        }
        
        pub fn read(onRead : proc() ) {
            // Make sure we're connected.

        }
        
        pub fn write(message : int) {

        }
    }

    pub mod epoll {
        use libc::c_int;    
        struct epoll_data_t<T> {
            ptr     :   *mut T,
            fd      :   int,
            u32_t   :   u32,
            u64_t   :   u64,
        }

        struct epoll_event {
            events  :   u32,
            data    :   epoll_data_t, 
        }

        pub fn create(size : uint) -> int {
            println!("Calling epoll_create syscall");
            unsafe {
                epoll_create(size as c_int) as int
            }
        }

        pub fn create1(flags : uint) -> int {
            println!("Calling epoll_create1 syscall"
            unsafe {
                epoll_create1(flags as c_int) as int
            }
        }

        pub fn control(flags : uint) -> int {
            println!("Calling epoll_create1 syscall"
            unsafe {
                epoll_create1(flags as c_int) as int
            }
        }

        extern {

            // Require kernel 2.6, glibc 2.3.2
            pub fn epoll_create(size : c_int) -> c_int;

            // Require kernel 2.6.27, glibc 2.9
            pub fn epoll_create1(flags : c_int) -> c_int;

            pub fn epoll_ctl(c_int epfd, c_int op, c_int fd, *const struct epoll_event) -> c_int;

        }
    }
}

fn main() {

    println!("{:d}",async::epoll::create(1));
}
