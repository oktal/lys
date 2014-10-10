extern crate async;

use async::{Timer,EventLoop};

fn on_timer_event(num_timeouts: u64) {
     println!("Timeout!");
}

fn main() {
    let mut ev_loop = EventLoop::default();

    let timer = match Timer::single_shot(on_timer_event, 2) {
        Ok(timer) => timer,
        Err(errno) => fail!(errno)
    };
    
    // TODO: Figure out why we need to return the reference that we borrowed

    let ev_loop2 = timer.attach_to(&mut ev_loop);

    ev_loop2.run();

}
