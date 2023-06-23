use std::{
    error::{self, Error},
    path::PathBuf,
};

use chrono::{NaiveDate, NaiveDateTime};
use tui_input::Input;

use crate::{
    config::{self, Config},
    entries::{self, EntryGroup, EntryRaw},
    files,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum InputMode {
    Editing,
    Logging,
}

pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Current value of the input box
    pub input: Input,
    /// Current input mode
    pub input_mode: InputMode,
    /// Effective date
    pub current_date: NaiveDate,
    /// Current Entries
    pub current_entries: EntryGroup,
    pub config: Config,
    log_path: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(log_path: String, config_path: String) -> Self {
        Self {
            running: true,
            input: Input::default(),
            input_mode: InputMode::Logging,
            current_date: chrono::Local::now().date_naive(),
            current_entries: Default::default(),
            config: config::read_config(config_path),
            log_path,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn move_next_day(&mut self) -> Result<NaiveDate, Box<dyn Error>> {
        if let Some(date) = self.current_date.succ_opt() {
            self.current_date = date;
            self.load_entries()?;
            Ok(date)
        } else {
            Err("Can't move forward a day. We're at the end of time!".into())
        }
    }

    pub fn move_prev_day(&mut self) -> Result<NaiveDate, Box<dyn Error>> {
        if let Some(date) = self.current_date.pred_opt() {
            self.current_date = date;
            self.load_entries()?;
            Ok(date)
        } else {
            Err("Can't move back a day. We're at the beginning of time!".into())
        }
    }

    pub fn load_entries(&mut self) -> Result<(), Box<dyn Error>> {
        match entries::read_all_date(
            &self.log_path(),
            self.current_date,
            self.config.virtual_midnight,
        ) {
            Ok(c) => {
                self.current_entries = c;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    // Construct a new Entry, save it to disk, and add it to the current list
    pub fn add_entry(&self, input_str: String) -> Result<(), Box<dyn Error>> {
        let time = chrono::Local::now().naive_local().time();
        let entry = EntryRaw::from_string(input_str, NaiveDateTime::new(self.current_date, time));
        entries::write(self, entry)
    }

    pub fn log_path(&self) -> PathBuf {
        if self.log_path.is_empty() {
            files::log_path()
        } else {
            PathBuf::from(&self.log_path)
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
