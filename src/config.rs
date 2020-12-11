/// implements JSON read-write
use crate::time::Duration;
use serde::{Deserialize, Serialize};
use std::time::{Duration as StdDuration, Instant as StdInstant};

#[derive(Serialize, Deserialize)]
pub struct Pomodoro {
    work_duration: u16,        // duration in minutes
    short_break_duration: u16, // duration in minutes
    long_break_duration: u16,  // duration in minutes
    cycles_before_long_break: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Scenario {
    scenario: Vec<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct Buff {
    work_intervals: Vec<u16>,
    rest_intervals: Vec<u16>,
}

impl Buff {
    pub fn new() -> Self {
        Self {
            work_intervals: vec![],
            rest_intervals: vec![],
        }
    }

    pub fn add_work_interval(&mut self, interval_len: u16) {
        self.work_intervals.push(interval_len);
    }

    pub fn add_rest_interval(&mut self, interval_len: u16) {
        self.rest_intervals.push(interval_len);
    }
}

#[derive(Serialize, Deserialize)]
pub struct HistoryEntry {
    work_interval: Duration,
    rest_interval: Duration,
}

#[derive(Serialize, Deserialize, Default)]
pub struct History {
    intervals: Vec<HistoryEntry>,
}

#[test]
fn test_history() {
    let mut h = History { intervals: vec![] };
    let start = StdInstant::now();
    let end = start.elapsed();
    let t = Duration::from_secs(4500);
    h.work_intervals.push(end.into());
    h.rest_intervals.push(t);

    println!("{}", serde_json::to_string(&h).unwrap());
}
