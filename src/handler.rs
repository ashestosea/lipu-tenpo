use crate::app::{App, AppResult, InputMode};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
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
        InputMode::Logging => match key.modifiers {
            KeyModifiers::CONTROL => match key.code {
                KeyCode::Left => match app.move_prev_day() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e)
                    }
                },
                KeyCode::Right => match app.move_next_day() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e)
                    }
                },
                _ => {}
            },
            _ => match key.code {
                KeyCode::Enter => {
                    app.add_entry(app.input.value().into());
                    app.input.reset();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Editing;
                }
                _ => {
                    app.input.handle_event(&CrosstermEvent::Key(KeyEvent {
                        code: key.code,
                        modifiers: key.modifiers,
                        kind: key.kind,
                        state: key.state,
                    }));
                }
            },
        },
    }

    Ok(())
}
