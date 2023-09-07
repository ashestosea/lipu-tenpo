use std::{
    collections::HashSet,
    error::{self, Error},
    path::PathBuf,
};

use chrono::{NaiveDate, NaiveDateTime};
use indicium::simple::{SearchIndex, SearchIndexBuilder, SearchType};
use tui_input::Input;

use crate::{
    config::{self, Config},
    entries::{self, EntryGroup, EntryRaw, EntryTitle},
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
    pub input_mode: InputMode,
    /// Effective date
    pub current_date: NaiveDate,
    pub current_entries: EntryGroup,
    pub entry_titles: Vec<EntryTitle>,
    pub search_index: SearchIndex<usize>,
    pub search_cursor: u32,
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
            entry_titles: Default::default(),
            search_index: Default::default(),
            search_cursor: 0,
            config: config::read_config(config_path),
            log_path,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn move_to_today(&mut self) {
        self.current_date = chrono::Local::now().naive_local().date();
        self.refresh();
    }

    pub fn move_next_day(&mut self) -> Result<NaiveDate, Box<dyn Error>> {
        if let Some(date) = self.current_date.succ_opt() {
            self.current_date = date;
            self.refresh();
            Ok(date)
        } else {
            Err("Can't move forward a day. We're at the end of time!".into())
        }
    }

    pub fn move_prev_day(&mut self) -> Result<NaiveDate, Box<dyn Error>> {
        if let Some(date) = self.current_date.pred_opt() {
            self.current_date = date;
            self.refresh();
            Ok(date)
        } else {
            Err("Can't move back a day. We're at the beginning of time!".into())
        }
    }

    pub fn get_current_date_entries(
        &mut self,
        log_contents: &String,
    ) -> Result<(), Box<dyn Error>> {
        match entries::read_all_date(
            log_contents,
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

    pub fn rebuild_search_index(&mut self, log_contents: &String) {
        let all_entries = entries::read_all_from_string(log_contents).unwrap();
        let mut entry_titles: Vec<EntryTitle> =
            all_entries.entries.iter().map(EntryTitle::from).collect();

        entry_titles.reverse();

        // Dedup while retaining order
        let mut uniques = HashSet::new();
        entry_titles.retain(|e| uniques.insert(e.clone()));

        let mut search_index = SearchIndexBuilder::default()
            .search_type(SearchType::Live)
            .build();

        for (count, e) in entry_titles.iter().enumerate() {
            search_index.insert(&count, &String::from(e));
        }

        self.entry_titles = entry_titles;
        self.search_index = search_index;
    }

    // Construct a new Entry, save it to disk, and add it to the current list
    pub fn add_entry(&self, input_str: String) -> Result<(), Box<dyn Error>> {
        let time = chrono::Local::now().naive_local().time();
        let entry = EntryRaw::from_string(input_str, NaiveDateTime::new(self.current_date, time));
        entries::write(self, entry)
    }

    pub fn search_back(&mut self) {
        self.search_cursor = self.search_cursor.saturating_add_signed(1);
    }

    pub fn search_forward(&mut self) {
        self.search_cursor = self.search_cursor.saturating_add_signed(-1);
    }

    pub fn log_path(&self) -> PathBuf {
        if self.log_path.is_empty() {
            files::log_path()
        } else {
            PathBuf::from(&self.log_path)
        }
    }

    pub fn log_contents(&self) -> String {
        std::fs::read_to_string(self.log_path()).unwrap()
    }

    pub fn refresh(&mut self) {
        self.input.reset();
        let log_contents = self.log_contents();
        self.get_current_date_entries(&log_contents);
        self.rebuild_search_index(&log_contents);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
