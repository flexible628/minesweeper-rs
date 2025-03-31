use std::time::Instant;

pub struct Counter {
    count: i32,
}

pub struct SecsCounter {
    now: Instant,
    secs: u64,
    is_idle: bool,
}

impl Counter {
    pub fn new(count: i32) -> Self {
        Self { count }
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }

    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }
}

impl Default for SecsCounter {
    fn default() -> Self {
        Self {
            now: Instant::now(),
            secs: 0,
            is_idle: true,
        }
    }
}

impl SecsCounter {
    pub fn get_secs(&mut self) -> Option<u64> {
        if self.is_idle {
            return None;
        }

        let elapsed = self.now.elapsed().as_secs() + 1;

        if self.secs < elapsed {
            self.secs = elapsed;
            Some(elapsed)
        } else {
            None
        }
    }

    pub fn start(&mut self) {
        self.now = Instant::now();
        self.secs = 0;
        self.is_idle = false;
    }

    pub fn stop(&mut self) {
        self.is_idle = true;
    }
}
