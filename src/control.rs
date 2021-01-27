use crate::config::{Config, History, HistoryEntry, Pomodoro};
use crate::time::Duration;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::string::String;

use dialog::DialogBox;
use std::cell::Cell;
use std::thread;

use shellexpand;

pub struct Control {
    history: Cell<History>,
    pomodoro: Pomodoro,
}

//const DEFAULT_PATH: &str = "~/.wtimer/";
const DEFAULT_HISTORY_DIR: &str = "~/.wtimer/data";
const DEFAULT_POMODORO_DIR: &str = "~/.wtimer/config";
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
        let pomodoro = Self::load_pomodoro(pomodoro_filename);
        let history = Cell::new(Self::load_history(history_filename));

        Ok(Self { history, pomodoro })
    }

    pub fn run() {
        let control = Control::load(None, None).expect("Failed to load configuration");

        loop {
            let work_duration = control.next_work_interval();
            let rest_duration = control.next_rest_interval();
            println!(
                "Work: {}, Rest: {}",
                work_duration.as_mins(),
                rest_duration.as_mins(),
            );
            let mut new_history = control.history.take();
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

            new_history.push(real_durations);
            control.history.set(new_history);
            control.store_history(None).unwrap();

            if let Some(answer) = dialog::Input::new("Continue pomodoring?")
                .show()
                .expect("Failed to show dialog box")
            {
                match &answer as &str {
                    "y" => (),
                    _ => break,
                }
            } else {
                break;
            }
        }
    }

    fn load_pomodoro(filename: Option<String>) -> Pomodoro {
        let config_filename =
            filename.unwrap_or(shellexpand::tilde(DEFAULT_POMODORO_PATH).to_string());
        let config_path = Path::new(&config_filename);
        if config_path.exists() {
            let data = fs::read_to_string(config_path).unwrap();
            if let Ok(config) = serde_json::from_str::<Pomodoro>(&data) {
                return config;
            }
        }
        let config_dir_filename = shellexpand::tilde(DEFAULT_POMODORO_DIR).to_string();
        std::fs::create_dir_all(config_dir_filename).expect("Failed to create config dirs");
        Pomodoro::default()
    }

    fn load_history(filename: Option<String>) -> History {
        let history_filename = filename.unwrap_or(shellexpand::tilde(DEFAULT_HISTORY_PATH).into());
        let history_path = Path::new(&history_filename);
        if history_path.exists() {
            let data = fs::read_to_string(history_path).unwrap();
            if let Ok(history) = serde_json::from_str::<History>(&data) {
                if history.is_today() {
                    return history;
                }
            }
        }
        let history_dir = shellexpand::tilde(DEFAULT_HISTORY_DIR).to_string();
        std::fs::create_dir_all(history_dir).expect("Failed to create history dirs");
        History::default()
    }

    fn store_history(&self, filename: Option<String>) -> Result<(), Box<dyn Error>> {
        let filename = filename.unwrap_or(shellexpand::tilde(DEFAULT_HISTORY_PATH).into());
        let json = serde_json::to_string_pretty(&self.history.take())?;
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
