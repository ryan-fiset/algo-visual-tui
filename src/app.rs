use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Currently selected tab
    pub current_tab: usize,
    /// List of tabs
    pub tab_list: Vec<&'static str>,
    /// Data Length
    pub data_length: u32,
    /// Tick rate
    pub tick_rate: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_tab: 0,
            tab_list: vec!["Bubble Sort", "Bogo Sort", "Selection Sort"],
            data_length: 50,
            tick_rate: 10,
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

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        if self.current_tab != self.tab_list.len() - 1 {
            self.current_tab += 1;
        } else {
            self.current_tab = 0;
        }
    }

    pub fn prev_tab(&mut self) {
        if self.current_tab != 0 {
            self.current_tab -= 1;
        } else {
            self.current_tab = self.tab_list.len() - 1;
        }
    }
}
