use rand::prelude::*;
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
    pub data_length: u64,
    /// Tick rate
    pub tick_rate: u64,
    /// Random number generator
    pub rng: ThreadRng,
    /// Barchart data
    pub bar_data: Vec<(&'static str, u64)>,
}

impl Default for App {
    fn default() -> Self {
        let data_length: u64 = 100;

        let mut rng = rand::thread_rng();
        let data: Vec<u64> = (1..=data_length).collect();

        let mut bar_data = data.iter().map(|x| ("", *x)).collect::<Vec<(&str, u64)>>();
        bar_data.shuffle(&mut rng);

        Self {
            running: true,
            tab_list: vec!["Bubble Sort", "Bogo Sort", "Selection Sort"],
            data_length,
            tick_rate: 10,
            rng,
            bar_data,
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

    pub fn shuffle_bar_data(&mut self) {
        self.bar_data.shuffle(&mut self.rng);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
