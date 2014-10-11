extern crate lio;

use lio::{Timer,EventLoop};

fn on_timer_event(num_timeouts: u64) {
     println!("Timeout!");
}

fn main() {
    let mut ev_loop = EventLoop::default();

    let timer = match Timer::single_shot(on_timer_event, 2) {
        Ok(timer) => timer,
        Err(errno) => fail!(errno)
    };

    timer.attach_to(&mut ev_loop);

    ev_loop.run();

}
