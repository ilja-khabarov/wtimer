/// implements JSON read-write
use crate::time::Duration;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::Cell;

pub trait Config {
    fn next_work_interval(&self) -> Duration;
    fn next_rest_interval(&self) -> Duration;
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
    fn next_work_interval(&self) -> Duration {
        self.current_cycles_amount
            .set(self.current_cycles_amount.get() + 1);
        Duration::from_minutes(self.work_duration as u64)
    }

    fn next_rest_interval(&self) -> Duration {
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

impl HistoryEntry {
    pub fn new(work: Duration, rest: Duration) -> Self {
        Self {
            work_interval: work,
            rest_interval: rest,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct History {
    #[serde(with = "serde_with::rust::display_fromstr")]
    date: NaiveDate,
    intervals: Vec<HistoryEntry>,
}

impl Default for History {
    fn default() -> Self {
        let date = Local::today().naive_local();
        History {
            date,
            intervals: vec![],
        }
    }
}

impl History {
    pub fn len(&self) -> usize {
        self.intervals.len()
    }

    pub fn push(&mut self, interval: HistoryEntry) {
        self.intervals.push(interval);
    }

    pub fn is_today(&self) -> bool {
        self.date == Local::today().naive_local()
    }
}

#[test]
fn test_history() {
    let mut h = History {
        date: Local::today().naive_local(),
        intervals: vec![HistoryEntry {
            work_interval: Duration::from_minutes(25),
            rest_interval: Duration::from_minutes(5),
        }],
    };
    let start = StdInstant::now();
    let end = start.elapsed();
    let t = Duration::from_secs(4500);
    let s = serde_json::to_string_pretty(&h).unwrap();

    println!("----- History: {}", s);
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
    let s = serde_json::to_string_pretty(&p).unwrap();
    let pomostr = r#"{"work_duration":25,"short_break_duration":5,"long_break_duration":15,"cycles_before_long_break":3,"current_cycles_amount":0}"#;
    let deser_pomo: Pomodoro = serde_json::from_str(pomostr).unwrap();
    assert_eq!(p, deser_pomo);
    println!("{}", s);
}
