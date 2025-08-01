use std::{
    collections::HashSet,
    error::{self, Error},
    path::PathBuf,
};

use chrono::{NaiveDate, NaiveDateTime};
use crossterm::event::Event as CrosstermEvent;
use indicium::simple::{SearchIndex, SearchIndexBuilder, SearchType};
use ratatui::widgets::ScrollbarState;
use tui_input::{backend::crossterm::EventHandler, Input};

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
    pub input: Input,
    pub log_time: String,
    pub log_opening: String,
    pub log_input: String,
    pub log_closing: String,
    pub input_mode: InputMode,
    /// Effective date
    pub current_date: NaiveDate,
    pub current_entries: EntryGroup,
    pub entry_titles: Vec<EntryTitle>,
    pub search_index: SearchIndex<usize>,
    pub search_cursor: i32,
    pub log_scroll: usize,
    pub scroll_state: ScrollbarState,
    pub config: Config,
    log_path: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(log_path: String, config_path: String) -> Self {
        Self {
            running: true,
            input: Input::default(),
            log_time: Default::default(),
            log_opening: Default::default(),
            log_input: Default::default(),
            log_closing: Default::default(),
            input_mode: InputMode::Logging,
            current_date: chrono::Local::now().date_naive(),
            current_entries: Default::default(),
            entry_titles: Default::default(),
            search_index: Default::default(),
            search_cursor: -1,
            log_scroll: Default::default(),
            scroll_state: Default::default(),
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

    /// Process [`crossterm`] input events and reconstruct [`App`] current_log
    pub fn handle_event(&mut self, evt: &CrosstermEvent) {
        self.input.handle_event(evt);
        self.construct_current_log();
    }

    pub fn cancel_search(&mut self) {
        self.search_cursor = -1;
        self.construct_current_log();
    }

    /// Override input with current history search
    pub fn accept_history(&mut self) {
        self.input = Input::new(
            [
                self.log_opening.clone(),
                self.log_input.clone(),
                self.log_closing.clone(),
            ]
            .concat(),
        );
        self.search_cursor = -1;
        self.construct_current_log();
    }

    /// If current entry is empty, move input to editing time
    pub fn handle_backspace_into_time(&mut self) {
        if !self.log_time.is_empty() && self.log_input.is_empty() {
            self.log_input = self.log_time.clone();
            self.log_time.clear();
            self.input = Input::new(self.log_input.clone());
        }
    }

    /// Construct the current log text from input and the search index if applicable
    fn construct_current_log(&mut self) {
        let time = chrono::Local::now().naive_local().time();
        let (input_time, input_entry) = entries::split_time_and_entry(
            self.input.value().to_string(),
            NaiveDateTime::new(self.current_date, time),
        );

        self.log_input = input_entry.clone();

        if let Some(time_string) = input_time.map(|t| t.format("%H:%M").to_string()) {
            self.log_time = [time_string, String::from(" ")].concat();
            self.input = Input::new(input_entry.clone());
        }

        let match_results = if input_entry.is_empty() {
            self.search_index
                .search(self.search_index.dump_keyword().unwrap())
        } else {
            self.search_index.search(&input_entry)
        };

        if self.search_cursor < 0 || match_results.is_empty() {
            self.log_opening.clear();
            self.log_closing.clear();
        } else {
            let index = if self.search_cursor >= match_results.len() as i32 {
                self.search_cursor = (match_results.len() - 1) as i32;
                match_results.len() - 1
            } else {
                self.search_cursor as usize
            };

            let history: String = self.entry_titles.get(*match_results[index]).unwrap().into();
            let start = history.find(self.log_input.as_str()).unwrap_or_default();

            let (opening, mid) = history.split_at(start);
            let (_, closing) = mid.split_at(self.log_input.len());

            self.log_opening = String::from(opening);
            self.log_closing = String::from(closing);
        }
    }

    /// Construct a new Entry from [`App`] current_log, save it to disk, and add it to the current list
    pub fn commit_current_log(&self) -> Result<(), Box<dyn Error>> {
        let time = chrono::Local::now().naive_local().time();
        let entry = EntryRaw::from_string(
            [
                self.log_time.clone(),
                self.log_opening.clone(),
                self.log_input.clone(),
                self.log_closing.clone(),
            ]
            .concat(),
            NaiveDateTime::new(self.current_date, time),
        );
        entries::write(self, entry)
    }

    /// Construct a new [`EntryRaw`], save it to disk, and add it to the current list
    pub fn add_log(&self, input: String) -> Result<(), Box<dyn Error>> {
        let time = chrono::Local::now().naive_local().time();
        let entry = EntryRaw::from_string(input, NaiveDateTime::new(self.current_date, time));
        entries::write(self, entry)
    }

    /// Move the history search cursor back one entry and reconstruct [`App`] current_log
    pub fn search_back(&mut self) {
        self.search_cursor = self.search_cursor.saturating_add(1);
        self.construct_current_log();
    }

    /// Move the history search cursor forward one entry and reconstruct [`App`] current_log
    pub fn search_forward(&mut self) {
        let temp = self.search_cursor - 1;
        if temp < -1 {
            self.search_cursor = -1;
        } else {
            self.search_cursor = temp;
        }
        self.construct_current_log();
    }

    /// Get [`app::App`] log_path or the default log path from [`files::log_path()`]
    pub fn log_path(&self) -> PathBuf {
        if self.log_path.is_empty() {
            files::log_path()
        } else {
            PathBuf::from(&self.log_path)
        }
    }

    /// Get content of the log file as a [`String`]
    pub fn log_contents(&self) -> String {
        std::fs::read_to_string(self.log_path()).unwrap()
    }

    /// Reset input, reload entries from disk, & rebuild the search index
    pub fn refresh(&mut self) {
        self.search_cursor = -1;
        self.log_time.clear();
        self.log_opening.clear();
        self.log_input.clear();
        self.log_closing.clear();
        self.input.reset();
        self.get_current_date_entries(&self.log_contents());
        self.scroll_log(0);
        self.rebuild_search_index(&self.log_contents());
    }

    pub fn scroll_log_up(&mut self) {
        self.scroll_state.prev();
        self.log_scroll = self.log_scroll.saturating_sub(1);
    }

    pub fn scroll_log_down(&mut self) {
        self.scroll_state.next();
        self.log_scroll = self
            .log_scroll
            .saturating_add(1)
            .clamp(0, self.current_entries.len().saturating_sub(1));
    }

    pub fn scroll_log(&mut self, index: usize) {
        self.log_scroll = index;
        self.scroll_state = self.scroll_state.content_length(self.current_entries.len());
        self.scroll_state = self.scroll_state.position(index);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
