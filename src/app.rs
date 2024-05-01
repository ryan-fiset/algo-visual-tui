use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
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
}
