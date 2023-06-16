use std::error;

use tui_input::Input;

use crate::entries::Entry;

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
    // Current Entries
    pub current_entries: Vec<Entry>,
}

impl Default for App {
    fn default() -> App {
        App {
            running: true,
            input: Input::default(),
            input_mode: InputMode::Editing,
            current_entries: Vec::new(),
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
    
    // Construct a new Entry, save it to disk, and add it to the current list
    pub fn add_entry(&self, input_str: String)
    {
        let entry: Entry = Entry::from(input_str);
        println!("{}", entry.project);
        println!("{}", entry.activity);
    }
    
    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
