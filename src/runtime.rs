/// runtime

use crate::config::Buff;

pub struct Runtime {
    buffer: Buff,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime{ buffer: Buff::new() }
    }

    pub fn add_work_interval(&mut self, interval_len: u16 ) {
        self.buffer.add_work_interval(interval_len);
    }

    pub fn add_rest_interval(&mut self, interval_len: u16 ) {
        self.buffer.add_rest_interval(interval_len);
    }
}