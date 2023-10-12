use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q' | 'Q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        KeyCode::Enter | KeyCode::Pause | KeyCode::Char(' ') => app.toggle_playing(),

        KeyCode::Right => app.forward(),
        KeyCode::Left => app.backward(),
        KeyCode::Char('r' | 'R') => app.replay(),

        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
