/// Fie responsible for time utils
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::time::Duration as StdDuration;

#[derive(Serialize, Deserialize)]
pub struct Duration {
    hrs: u64,
    mins: u64,
    secs: u64,
}

impl Duration {
    pub fn from_secs(secs: u64) -> Duration {
        let hrs = secs / 3600;
        let secs = secs % 3600;
        let mins = secs / 60;
        let secs = secs % 60;
        Duration { hrs, mins, secs }
    }
    pub fn into_secs(self) -> u64 {
        self.secs + self.mins * 60 + self.hrs * 3600
    }
}

impl From<StdDuration> for Duration {
    fn from(d: StdDuration) -> Duration {
        let secs = d.as_secs();
        Self::from_secs(secs)
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_secs(self.into_secs() + rhs.into_secs())
    }
}
