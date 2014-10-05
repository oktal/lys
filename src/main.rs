#![feature(globs)]

use events::timer;

mod events;

struct MyCallback;

impl timer::TimerCallback for MyCallback {

    fn call(&self, numTimeouts: u64) {
        println!("Timeout");
    }
}

fn main() {
    let evLoop = events::event_loop::EventLoop::default();

    let on_timer_event = box MyCallback;

    let timer = match timer::Timer::new(on_timer_event, 1) {
        Ok(timer) => timer,
        Err(errno) => fail!(errno)
    };

    timer.attach_to(evLoop);


    evLoop.run()
}
