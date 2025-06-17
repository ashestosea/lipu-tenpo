use crate::app::{App, AppResult, InputMode};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};

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
                KeyCode::Home => app.move_to_today(),
                KeyCode::Char('h') => app.move_to_today(),
                _ => {}
            },
            _ => match key_evt.code {
                KeyCode::Enter => {
                    app.commit_current_log();
                    app.refresh();
                }
                KeyCode::Esc => {
                    app.refresh();
                }
                KeyCode::Up => {
                    app.search_back();
                }
                KeyCode::Down => {
                    app.search_forward();
                }
                KeyCode::Right => {
                    if app.search_cursor >= 0 && app.input.cursor() == app.input.value().len() {
                        app.accept_history();
                    } else {
                        app.handle_event(&CrosstermEvent::Key(KeyEvent {
                            code: key_evt.code,
                            modifiers: key_evt.modifiers,
                            kind: key_evt.kind,
                            state: key_evt.state,
                        }));
                    }
                }
                KeyCode::Tab => {
                    app.accept_history();
                }
                KeyCode::PageUp => {
                    app.scroll_log_up();
                }
                KeyCode::PageDown => {
                    app.scroll_log_down();
                }
                KeyCode::Backspace => {
                    app.cancel_search();
                    app.handle_backspace_into_time();
                    app.handle_event(&CrosstermEvent::Key(KeyEvent {
                        code: key_evt.code,
                        modifiers: key_evt.modifiers,
                        kind: key_evt.kind,
                        state: key_evt.state,
                    }));
                }
                _ => {
                    app.handle_event(&CrosstermEvent::Key(KeyEvent {
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
