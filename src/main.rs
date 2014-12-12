extern crate lys;

use lys::io::{EventLoop, IoEventHandler, IoEvent};
use lys::io::{AsyncIoProvider, Timer, Notify};
use lys::io::tcp::TcpEndpoint;

use std::rc::Rc;

struct SimpleTcpServer<'a> {
    ev_loop: EventLoop<'a>,
    endpoint: Rc<Box<TcpEndpoint>>
}

struct MyIoHandler {
    endpoint: Rc<Box<TcpEndpoint>>
}

impl MyIoHandler {
    pub fn new(endpoint: Rc<Box<TcpEndpoint>>) -> MyIoHandler {
        MyIoHandler {
            endpoint: endpoint
        }
    }

    fn handle_tcp_connection<'a>(&self, ev_loop: &'a mut EventLoop<'a>) {
        println!("New connection !");

        for conn in self.endpoint.accept() {
            ev_loop.start_io(Rc::new(box conn as Box<AsyncIoProvider>));
        }
    }

    fn handle_tcp_data<'a>(&self, ev_loop: &'a mut EventLoop<'a>, data: Vec<u8>) {
        println!("Data !")
    }

}

impl IoEventHandler for MyIoHandler {
    fn handle_event<'a>(&self, ev_loop: &'a mut EventLoop<'a>, io_event: IoEvent) {
        match io_event {
            IoEvent::TcpConnection => self.handle_tcp_connection(ev_loop),
            IoEvent::In(data) => self.handle_tcp_data(ev_loop, data),
            _ => ()
        }
    }

}

impl<'a> SimpleTcpServer<'a> {
    pub fn new() -> SimpleTcpServer<'a> {
        let mut ev_loop = EventLoop::default();

        let endpoint = Rc::new(box TcpEndpoint::bind("0.0.0.0", 9090).unwrap());

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
        //self.ev_loop.start_io(self.endpoint);

        let handler = MyIoHandler::new(self.endpoint);
        self.ev_loop.run(&handler);

    }
}


fn main() {
    let mut server = SimpleTcpServer::new();
    server.listen();
    server.start();
}
