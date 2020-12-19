use crate::config::{Config, History, HistoryEntry, Pomodoro};
use crate::time::Duration;
use std::error::Error;
use std::fs;
use std::string::String;

use dialog::DialogBox;
use std::cell::Cell;
use std::thread;

//extern crate shellexpand;
use shellexpand;

pub struct Control {
    history: Cell<History>,
    pomodoro: Pomodoro,
}

const DEFAULT_PATH: &str = "~/.wtimer/";
const DEFAULT_HISTORY_PATH: &str = "~/.wtimer/data/history.json";
const DEFAULT_POMODORO_PATH: &str = "~/.wtimer/config/pomodoro.json";

impl Control {
    pub fn new() -> Self {
        Self::load(None, None).expect("Failed to find default files")
    }

    pub fn load(
        history_filename: Option<String>,
        pomodoro_filename: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let history_filename =
            history_filename.unwrap_or(shellexpand::tilde(DEFAULT_HISTORY_PATH).into());
        let pomodoro_filename =
            pomodoro_filename.unwrap_or(shellexpand::tilde(DEFAULT_POMODORO_PATH).into());

        let data = fs::read_to_string(history_filename).unwrap();
        let history = Cell::new(serde_json::from_str(&data)?);
        let data = fs::read_to_string(pomodoro_filename)?;
        let pomodoro = serde_json::from_str(&data)?;

        Ok(Self { history, pomodoro })
    }

    pub fn run(&self) {
        let control = Control::load(None, None).expect("Failed to load configuration");
        let work_duration = control.next_work_interval();
        let rest_duration = control.next_rest_interval();

        let start = std::time::Instant::now();
        //working
        thread::sleep(work_duration.into());
        dialog::Message::new("Done working. Have your rest.")
            .show()
            .expect("Failed to show dialog box");
        let real_work_dur = start.elapsed().into();

        // resting
        let start = std::time::Instant::now();
        thread::sleep(rest_duration.into());
        dialog::Message::new("Done resting. Back to work.")
            .show()
            .expect("Failed to show dialog box");
        let real_rest_dur = start.elapsed().into();

        let real_durations: HistoryEntry = HistoryEntry::new(real_work_dur, real_rest_dur);

        let mut new_history = self.history.take();
        new_history.push(real_durations);
        self.history.set(new_history);
        self.store_history(None).unwrap();
    }

    pub fn store_history(&self, filename: Option<String>) -> Result<(), Box<dyn Error>> {
        let filename = filename.unwrap_or(shellexpand::tilde(DEFAULT_HISTORY_PATH).into());
        let json = serde_json::to_string(&self.history.take())?;
        fs::write(filename, json)?; // full file rewrite is not a bug ATM
        Ok(())
    }

    pub fn next_work_interval(&self) -> Duration {
        self.pomodoro.next_work_interval()
    }

    pub fn next_rest_interval(&self) -> Duration {
        self.pomodoro.next_rest_interval()
    }
}

impl Default for Control {
    fn default() -> Self {
        unimplemented!()
    }
}
