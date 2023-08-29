use crate::app::{App, AppResult, InputMode};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

pub fn handle_key_events(app: &mut App, key_evt: KeyEvent) -> AppResult<()> {
    match app.input_mode {
        InputMode::Editing => match key_evt.code {
            KeyCode::Char('e') => {
                app.input_mode = InputMode::Logging;
            }
            KeyCode::Char('q') => {
                app.quit();
            }
            _ => {}
        },
        InputMode::Logging => match key_evt.modifiers {
            KeyModifiers::CONTROL => match key_evt.code {
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
                KeyCode::Char('c') => app.quit(),
                KeyCode::Char('q') => app.quit(),
                KeyCode::Char('h') => app.move_to_today(),
                _ => {}
            },
            _ => match key_evt.code {
                KeyCode::Enter => {
                    app.add_entry(app.input.value().into());
                    app.refresh();
                }
                KeyCode::Esc => {
                    app.refresh();
                }
                _ => {
                    app.input.handle_event(&CrosstermEvent::Key(KeyEvent {
                        code: key_evt.code,
                        modifiers: key_evt.modifiers,
                        kind: key_evt.kind,
                        state: key_evt.state,
                    }));
                }
            },
        },
    }

    Ok(())
}
