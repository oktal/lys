extern crate lys;

use std::option::Option;

use lys::io::EventLoop;
use lys::io::tcp::{Tcp, TcpEndpoint, TcpSocket};

use std::io::net::ip::{SocketAddr, Ipv4Addr};

struct SimpleTcpServer<'a> {
    ev_loop: EventLoop<'a>,

    endpoint: TcpEndpoint
}

impl<'a> SimpleTcpServer<'a> {
    pub fn new() -> SimpleTcpServer<'a> {
        let ev_loop = EventLoop::default();

        SimpleTcpServer {
            ev_loop: ev_loop,
            endpoint: None
        }
        
    }

    pub fn bind(&mut self) {
        self.endpoint
            = Some(TcpEndpoint::bind("127.0.0.1", 9090, self.on_connection).unwrap());
        self.endpoint.listen();
        println!("Listening on 127.0.0.1:9090");
        self.ev_loop.add_event(&self.endpoint);
    }

    pub fn run(&mut self) {
        self.ev_loop.run();
    }

    fn on_connection(&self, sock: TcpSocket) {
        println!("Got a new connection!");
    }
}

fn main() {

}
