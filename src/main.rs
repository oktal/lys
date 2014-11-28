extern crate lys;

use lys::io::{EventLoop, IoEventHandler, IoEvent};
use lys::io::{Timer, Notify};
use lys::io::tcp::TcpEndpoint;

struct SimpleTcpServer<'a> {
    ev_loop: EventLoop<'a>,
    endpoint: TcpEndpoint
}

struct MyIoHandler<'a> {
    endpoint: &'a TcpEndpoint
}

impl<'a> MyIoHandler<'a> {
    pub fn new(endpoint: &'a TcpEndpoint) -> MyIoHandler<'a> {
        MyIoHandler {
            endpoint: endpoint
        }
    }

    fn handle_tcp_connection(&self) {
        println!("New connection !");

        for conn in self.endpoint.accept() {
        }
    }

}

impl<'a> IoEventHandler for MyIoHandler<'a> {
    fn handle_event(&self, io_event: IoEvent) {
        match io_event {
            TcpConnection => self.handle_tcp_connection()
        }
    }

}

impl<'a> SimpleTcpServer<'a> {
    pub fn new() -> SimpleTcpServer<'a> {
        let mut ev_loop = EventLoop::default();

        let endpoint = TcpEndpoint::bind("0.0.0.0", 9090).unwrap();

        SimpleTcpServer {
            ev_loop: ev_loop,
            endpoint: endpoint
        }
    }

    pub fn listen(&mut self) {
        println!("Listening on 0.0.0.0:9090");
        self.endpoint.listen();
    }

    pub fn start(&mut self) {
        self.ev_loop.start_io(&self.endpoint);

        let handler = MyIoHandler::new(&self.endpoint);
        self.ev_loop.run(&handler);
    }
}


fn main() {
    let mut server = SimpleTcpServer::new();
    server.listen();
    server.start();
}
