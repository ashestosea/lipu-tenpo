use std::path::PathBuf;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::App, entries};

/// Renders the user interface widgets.
pub fn render<B: Backend>(_app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples

    let top_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Max(1),
                Constraint::Min(2),
                Constraint::Max(2),
                Constraint::Max(2),
            ]
            .as_ref(),
        )
        .split(top_layout[0]);

    let block = Block::default()
        .borders(Borders::NONE)
        // .border_style(Style::default())
        // .border_type(BorderType::Rounded)
        // .title("lipu-tenpo")
        .title_alignment(Alignment::Center);
    frame.render_widget(block, top_layout[0]);

    // Date
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(chrono::Local::now().format("%Y-%m-%d").to_string())
        .style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::DIM)
                .fg(Color::DarkGray),
        );
    frame.render_widget(block, main_layout[0]);

    // Log
    let block = Block::default().borders(Borders::NONE);
    let entries = entries::read_all_from(&PathBuf::from("./test.csv")).unwrap();
    let items: Vec<ListItem> = entries
        .iter()
        .map(|f| -> ListItem { ListItem::new(f) })
        .collect();
    let list: List = List::new(items)
        .block(block)
        .style(Style::default().bg(Color::DarkGray));
    frame.render_widget(list, main_layout[1]);

    // Summary
    let summary_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_layout[2]);

    let block = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Double);
    let work_summary = Paragraph::new("Work: 6h 30m").block(block);
    frame.render_widget(work_summary, summary_layout[0]);

    let block = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Thick);
    let other_summary = Paragraph::new("Other: 1h 15m")
        .alignment(Alignment::Right)
        .block(block);
    frame.render_widget(other_summary, summary_layout[1]);

    // Input
    let block = Block::default().borders(Borders::TOP);
    frame.render_widget(block, main_layout[3]);
}
