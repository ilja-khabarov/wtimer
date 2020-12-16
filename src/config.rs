/// implements JSON read-write
use crate::time::Duration;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::time::{Duration as StdDuration, Instant as StdInstant};

pub trait Config {
    fn get_next_work_interval(&self) -> Duration;
    fn get_next_rest_interval(&self) -> Duration;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pomodoro {
    work_duration: u16,        // duration in minutes
    short_break_duration: u16, // duration in minutes
    long_break_duration: u16,  // duration in minutes
    cycles_before_long_break: u8,
    current_cycles_amount: Cell<u8>,
}

impl Default for Pomodoro {
    fn default() -> Self {
        Self {
            work_duration: 25,       // duration in minutes
            short_break_duration: 5, // duration in minutes
            long_break_duration: 15, // duration in minutes
            cycles_before_long_break: 3,
            current_cycles_amount: Cell::new(0),
        }
    }
}

impl Config for Pomodoro {
    fn get_next_work_interval(&self) -> Duration {
        self.current_cycles_amount
            .set(self.current_cycles_amount.get() + 1);
        Duration::from_minutes(self.work_duration as u64)
    }

    fn get_next_rest_interval(&self) -> Duration {
        if self.current_cycles_amount.get() == self.cycles_before_long_break {
            self.current_cycles_amount.set(0);
            Duration::from_minutes(self.long_break_duration as u64)
        } else {
            Duration::from_minutes(self.short_break_duration as u64)
        }
    }
}

/// unsupported
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

impl History {
    pub fn len(&self) -> usize {
        self.intervals.len()
    }
}

#[test]
#[ignore]
fn test_history() {
    let mut h = History { intervals: vec![] };
    let start = StdInstant::now();
    let end = start.elapsed();
    let t = Duration::from_secs(4500);
    //h.work_intervals.push(end.into());
    //h.rest_intervals.push(t);

    //println!("{}", serde_json::to_string(&h).unwrap());
}

#[test]
fn test_pomodoro() {
    let p = Pomodoro {
        work_duration: 25,       // duration in minutes
        short_break_duration: 5, // duration in minutes
        long_break_duration: 15, // duration in minutes
        cycles_before_long_break: 3,
        current_cycles_amount: Cell::new(0),
    };
    let s = serde_json::to_string(&p).unwrap();
    let pomostr = r#"{"work_duration":25,"short_break_duration":5,"long_break_duration":15,"cycles_before_long_break":3,"current_cycles_amount":0}"#;
    let deser_pomo: Pomodoro = serde_json::from_str(pomostr).unwrap();
    assert_eq!(p, deser_pomo);
    println!("{}", s);
}
