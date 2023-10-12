// #![allow(unused)]
use crate::app::App;
use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute,
};
use std::{io::Write, sync::OnceLock, thread::sleep, time::Duration};
use tui::{backend::Backend, Frame};

static H: OnceLock<u16> = OnceLock::new();
static _W: OnceLock<u16> = OnceLock::new();

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, _frame: &mut Frame<'_, B>) {
    let mut stdout = std::io::stdout();
    let (term_width, term_height) = crossterm::terminal::size().unwrap();

    let txt = app.next_txt();
    let height = *H.get_or_init(|| txt.lines().count() as u16);
    // let width = *W.get_or_init(|| txt.trim().chars().count() as u16 / height);

    // let padding_x = term_width.saturating_sub(width) / 2;
    let padding_y = term_height.saturating_sub(height) / 2;
    execute!(stdout, MoveTo(0, padding_y)).ok();

    for line in txt.lines() {
        write!(&mut stdout, "{0:^1$}", line, term_width as usize).ok();
        // write!(&mut stdout, "{}", line).ok();
        execute!(stdout, MoveToNextLine(1)).ok();
    }
    sleep(Duration::from_millis(30));
}
