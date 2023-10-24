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
                // KeyCode::Left => {
                //     app.input = app.input.clone().with_cursor(app.input.cursor() - 1);
                // }
                // KeyCode::Right => {
                //     app.input = app.input.clone().with_cursor(app.input.cursor() + 1);
                // }
                KeyCode::PageUp => {
                    app.scroll_log_up();
                }
                KeyCode::PageDown => {
                    app.scroll_log_down();
                }
                // KeyCode::Backspace => {
                //     // app.current_log.pop();
                //     app.current_log.remove(app.input.cursor());
                //     // app.input = tui_input::Input::default();
                //     app.input = app.input.clone().with_value(app.current_log.clone()).with_cursor(app.input.visual_cursor());
                // }
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
