use crate::app::{App, InputMode, AppResult};
use crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

pub fn handle_key_events(mut app: &mut App, key: KeyEvent) -> AppResult<()> {
    match app.input_mode {
        InputMode::Editing => match key.code {
            KeyCode::Char('e') => {
                app.input_mode = InputMode::Logging;
            }
            KeyCode::Char('q') => {
                app.quit();
            }
            _ => {}
        },
        InputMode::Logging => match key.code {
            KeyCode::Enter => {
                app.messages.push(app.input.value().into());
                app.input.reset();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Editing;
            }
            _ => {
                app.input.handle_event(&Event::Key(key));
            }
        },
    }
    
    Ok(())
}
