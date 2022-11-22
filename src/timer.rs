use std::time::{Instant, Duration};

pub struct GameTimer {
    pub ready: bool,
    duration_ms: Duration,
    _last_instant: Instant,
}

impl GameTimer {
    pub fn new(duration_ms: Duration) -> Self {
        Self {
            ready: false,
            duration_ms,
            _last_instant: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        if self._last_instant.elapsed() > self.duration_ms {
            self.ready = true;
            self._last_instant = Instant::now();
        }
    }

    pub fn reset(&mut self) {
        self.ready = false;
    }
}