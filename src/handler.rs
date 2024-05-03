use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Increase data length
        KeyCode::Char('k') | KeyCode::Char('K') => app.data_length += 1,
        // Decrease data length
        KeyCode::Char('j') | KeyCode::Char('J') => {
            if app.data_length > 3 {
                app.data_length -= 1;
            }
        }
        // Increase tick rate
        KeyCode::Char('l') | KeyCode::Char('L') => app.tick_rate += 1,
        // Decrease tick rate
        KeyCode::Char('h') | KeyCode::Char('H') => {
            if app.tick_rate > 1 {
                app.tick_rate -= 1;
            }
        }
        // Shuffle Data
        KeyCode::Char('r') | KeyCode::Char('R') => app.shuffle_bar_data(),
        _ => {}
    }
    Ok(())
}
