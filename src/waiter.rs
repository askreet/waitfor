use std::thread::sleep;
use std::time::Duration;
use crate::cli::InputOptions;

pub struct Waiter {
    next_delay_ms: u64,
    rate: f32,
}

impl Waiter {
    pub fn next_delay(&self) -> u64 {
        self.next_delay_ms
    }

    pub fn wait(&mut self) -> () {
        sleep(Duration::from_millis(self.next_delay_ms));
        self.next_delay_ms = (self.next_delay_ms as f32 * self.rate).floor() as u64;
    }
}

pub fn create_waiter(opt: &InputOptions) -> Waiter {
    if opt.backoff {
        Waiter { next_delay_ms: opt.delay, rate: opt.rate }
    } else {
        Waiter { next_delay_ms: opt.delay, rate: 1.0 }
    }
}
