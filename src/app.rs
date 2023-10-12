use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use rayon::prelude::*;
use std::{error, fs, path::Path};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Playing,
    Replay,
    Stopping,
    End,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub txts: Vec<String>,
    pub idx: usize,
    pub state: State,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            txts: vec![],
            idx: 1,
            state: State::Playing,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new<T: AsRef<Path>>(txt_dir: T) -> Self {
        let txt_dir = txt_dir.as_ref();
        let txts = fs::read_dir(txt_dir)
            .unwrap()
            .par_bridge()
            .map(|entry| {
                let path = entry.unwrap().path();
                fs::read_to_string(path).unwrap()
            })
            .collect();

        Self {
            txts,
            ..Self::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_txt(&mut self) -> &str {
        if self.state == State::Stopping {
            return &self.txts[self.idx];
        }

        // self.idx = self.idx.clamp(0, self.txts.len() - 1);
        if self.idx < self.txts.len() {
            self.idx += 1;
        } else {
            self.idx = self.txts.len() - 1;
            self.state = State::Stopping;
            execute!(std::io::stdout(), Clear(ClearType::All)).ok();
        }
        &self.txts[self.idx]
        // let path = format!("txts/image_{:0>4}.txt", self.idx);
        // fs::read_to_string(path).unwrap()
    }

    pub fn toggle_playing(&mut self) {
        self.state = match self.state {
            State::Stopping => State::Playing,
            _ => State::Stopping,
        }
    }

    pub fn forward(&mut self) {
        self.idx = self.idx.saturating_add(50);
    }

    pub fn backward(&mut self) {
        self.idx = self.idx.saturating_sub(50);
    }

    pub fn replay(&mut self) {
        self.idx = 0;
    }
}
