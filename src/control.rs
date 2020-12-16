use crate::config::{Config, History, Pomodoro};
use crate::time::Duration;
use std::error::Error;
use std::fs;
use std::string::String;

use dialog::DialogBox;
use std::thread;

pub struct Control {
    history: History,
    pomodoro: Pomodoro,
}

const DEFAULT_PATH: &str = "~/.wtimer/";
const DEFAULT_HISTORY_PATH: &str = "~/.wtimer/history/history.json";
const DEFAULT_POMODORO_PATH: &str = "~/.wtimer/config/pomodoro.json";

impl Control {
    pub fn new() -> Self {
        Self::load(None, None).expect("Failed to find default files")
    }

    pub fn load(
        history_filename: Option<String>,
        pomodoro_filename: Option<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let history_filename = history_filename.unwrap_or(DEFAULT_HISTORY_PATH.to_owned());
        let pomodoro_filename = pomodoro_filename.unwrap_or(DEFAULT_POMODORO_PATH.to_owned());
        let data = fs::read_to_string(history_filename).unwrap_or_default();
        //let history = serde_json::from_str(&data)?;
        //let data = fs::read_to_string(pomodoro_filename)?;
        //let pomodoro = serde_json::from_str(&data)?;
        let pomodoro = Pomodoro::default();
        let history = History::default();

        Ok(Self { history, pomodoro })
    }

    pub fn run(&self) {
        let control = Control::load(None, None).expect("Failed to load configuration");
        let work_duration = control.get_next_work_interval();
        let rest_duration = control.get_next_rest_interval();

        // working
        let work_duration = Duration::from_secs(5);
        let rest_duration = Duration::from_secs(5);
        thread::sleep(work_duration.into());
        dialog::Message::new("Done working. Have your rest.")
            .show()
            .expect("Failed to show dialog box");
        // resting
        thread::sleep(rest_duration.into());
        dialog::Message::new("Done resting. Back to work.")
            .show()
            .expect("Failed to show dialog box");
    }

    pub fn store_history(&self, filename: Option<String>) -> Result<(), Box<dyn Error>> {
        let filename = filename.unwrap_or(DEFAULT_HISTORY_PATH.to_owned());
        let json = serde_json::to_string(&self.history)?;
        fs::write(filename, json)?;
        Ok(())
    }

    pub fn get_next_work_interval(&self) -> Duration {
        self.pomodoro.get_next_work_interval()
    }

    pub fn get_next_rest_interval(&self) -> Duration {
        self.pomodoro.get_next_work_interval()
    }
}

impl Default for Control {
    fn default() -> Self {
        unimplemented!()
    }
}
