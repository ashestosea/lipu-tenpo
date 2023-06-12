use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use ratatui::{
    backend::{CrosstermBackend, Backend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget, List, ListItem},
    Terminal, Frame, text::Text,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        ui(f);
    })?;
    
    thread::sleep(Duration::from_secs(5));
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Length(1)
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
         .title("lipu tenpo")
         .borders(Borders::TOP);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
         .title("2023 June 10 Saturday")
         .borders(Borders::ALL);
    // f.render_widget(block, chunks[1]);
    let items = vec![ListItem::new("1")];
    let list: List = List::new(items).block(block);
    f.render_widget(list, chunks[2]);
    let block = Block::default()
         .title("Work: 7h 30m")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[3]);
    let text_field = Text::default();
    // f.render_widget(text_field, chunks[4]);
}
