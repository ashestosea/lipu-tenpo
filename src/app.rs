use std::{
    error::{self, Error},
    path::PathBuf,
};

use chrono::NaiveDate;
use tui_input::Input;

use crate::entries::{self, Entry};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum InputMode {
    Editing,
    Logging,
}

/// Application.
#[derive(Debug)]
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
    pub current_entries: Vec<Entry>,
    pub log_path: PathBuf,
}

impl Default for App {
    fn default() -> App {
        App {
            running: true,
            input: Input::default(),
            input_mode: InputMode::Logging,
            current_date: chrono::Local::now().date_naive(),
            current_entries: Vec::new(),
            log_path: PathBuf::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
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

    pub fn load_entries(&mut self) -> Result<Vec<Entry>, Box<dyn Error>> {
        entries::read_all_date(&self.log_path, self.current_date)
    }

    // Construct a new Entry, save it to disk, and add it to the current list
    pub fn add_entry(&self, input_str: String) -> Result<(), Box<dyn Error>> {
        let entry: Entry = Entry::from(input_str);
        entries::write(self, entry)
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
