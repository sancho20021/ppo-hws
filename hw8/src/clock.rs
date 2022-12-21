use std::time::{Instant, Duration};

pub trait Clock {
    fn now(&self) -> Instant;
}

pub struct InstantClock;

impl Clock for InstantClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

pub struct FrozenClock {
    time: Instant
}

impl FrozenClock {
    pub fn new(time: Instant) -> Self {
        FrozenClock { time }
    }

    pub fn set(&mut self, time: Instant) {
        self.time = time;
    }

    pub fn get(&self) -> Instant {
        self.time.clone()
    }

    pub fn skip_minutes(&mut self, minutes: u64) {
        self.set(self.get() + Duration::from_secs(60 * minutes));
    }
}

impl Clock for FrozenClock {
    fn now(&self) -> Instant {
        self.time
    }
}
