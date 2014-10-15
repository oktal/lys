extern crate lys;

use lys::io::{Timer, Notify, EventLoop};

fn on_timer_event(num_timeouts: u64) {
     println!("Timeout!");
}

fn on_notify() {
    println!("Notified!");
}

fn main() {
    let mut ev_loop = EventLoop::default();

    let timer = match Timer::new(on_timer_event, 2) {
        Ok(timer) => timer,
        Err(errno) => fail!(errno)
    };

    timer.attach_to(&mut ev_loop);

    let notify = match Notify::new(on_notify) {
        Ok(notify) => notify,
        Err(errno) => fail!(errno)
    };

    notify.attach_to(&mut ev_loop);

    match notify.notify() {
        Ok(_) => (),
        Err(errno) => fail!(errno)
    }

    ev_loop.run();

}
