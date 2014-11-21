extern crate lys;

use lys::io::EventLoop;
use lys::io::tcp::TcpEndpoint;

struct SimpleTcpServer<'a> {
    ev_loop: EventLoop<'a>,
    endpoint: TcpEndpoint
}

fn main() {
}
