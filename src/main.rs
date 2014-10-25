extern crate lys;

use lys::io::{AsyncEvent, Timer, Notify, EventLoop, Tcp};

use std::io::net::ip::{SocketAddr, Ipv4Addr};

fn on_timer_event(timer: &Timer, num_timeouts: u64) {
     let timer_event = timer as &AsyncEvent;
     println!("Timeout! -> {}", timer_event.poll_fd());
}

fn on_notify(notify: &Notify) {
    println!("Notified!");
}

fn main() {
    let mut ev_loop = EventLoop::default();

    let timer = match Timer::new(on_timer_event, 2) {
        Ok(timer) => timer,
        Err(errno) => fail!(errno)
    };

    ev_loop.add_event(&timer);

    let notify = match Notify::new(on_notify) {
        Ok(notify) => notify,
        Err(errno) => fail!(errno)
    };

    ev_loop.add_event(&notify);


    match notify.notify() {
        Ok(_) => (),
        Err(errno) => fail!(errno)
    }

    let tcp = Tcp::connect("google.com", 80).unwrap();

    ev_loop.add_event(&tcp);

    ev_loop.run();

}
