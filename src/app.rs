use std::error;
use chrono;
/// use crate::command::get_destop;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// the current time
    pub time: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            time: chrono::offset::Local::now().format("%H:%M:%S").to_string(),
            running: true,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.time = chrono::offset::Local::now().format("%H:%M:%S").to_string();
    }

    pub fn new_desktop() {
        
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
