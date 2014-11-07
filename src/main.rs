extern crate lys;

use lys::io::{AsyncOperation, Pollable, Timer, Notify, EventLoop, Tcp};

use std::io::net::ip::{SocketAddr, Ipv4Addr};

fn on_timer_event(timer: &Timer, num_timeouts: u64) {
     println!("Timeout! -> {}", timer.poll_fd());
}

fn on_notify(notify: &Notify) {
    println!("Notified!");
}

fn on_connect(tcp: &Tcp) {
    println!("Connected!");
}

fn main() {
    let mut ev_loop = EventLoop::default();

    let timer = match Timer::new(on_timer_event, 2) {
        Ok(timer) => timer,
        Err(errno) => panic!(errno)
    };

    ev_loop.add_event(&timer);

    let notify = match Notify::new(on_notify) {
        Ok(notify) => notify,
        Err(errno) => panic!(errno)
    };

    ev_loop.add_event(&notify);


    match notify.notify() {
        Ok(_) => (),
        Err(errno) => panic!(errno)
    }

    let tcp = Tcp::connect("google.com", 80, on_connect).unwrap();

    ev_loop.add_event(&tcp);

    ev_loop.run();

}
