use std::time::{Duration, Instant};

pub struct Timer {
    pub last_instant: Instant,
    pub delta: Duration,     // now - last instant
    pub period: Duration,    //how often we want the timer to go off or be ready
    pub countdown: Duration, //how much time until the timer goes off next
    pub ready: bool,         // timer is ready
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        // if negative then reset timer props ready and period
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
