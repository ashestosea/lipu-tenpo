use chrono::Datelike;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, InputMode};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/tui-rs-revival/ratatui/tree/master/examples

    let entry_group = &app.current_entries;

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
    let current_date = app.current_date;
    let is_today = current_date == chrono::Local::now().naive_local().date();
    let is_today_str = if is_today { "@" } else { "" };
    let date_style = if is_today {
        Style::default()
                .bg(Color::Magenta)
                .fg(Color::Black)
    } else {
        Style::default()
                .bg(Color::Gray)
                .fg(Color::Black)
    };

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(format!(
            "─{}─{}──{}",
            is_today_str,
            current_date.weekday(),
            &current_date.format("%Y─%m─%d")
        ))
        .style(date_style);
    frame.render_widget(block, main_layout[0]);

    // Log
    let block = Block::default().borders(Borders::NONE);
    let mut items: Vec<ListItem> = entry_group
        .entries
        .iter()
        .map(|f| -> ListItem { ListItem::new(f) })
        .collect();
    if let Some(mut time_since_last) = entry_group.time_since_last_display() {
        time_since_last.insert_str(0, "> ");
        items.push(ListItem::new(Text::raw(time_since_last)));
    }
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
    let work_summary =
        Paragraph::new(format!("On task: {}", entry_group.time_on_task_display())).block(block);
    frame.render_widget(work_summary, summary_layout[0]);

    let block = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Thick);
    let other_summary = Paragraph::new(format!("Other: {}", entry_group.time_off_task_display()))
        .alignment(Alignment::Right)
        .block(block);
    frame.render_widget(other_summary, summary_layout[1]);

    // Input
    let width = main_layout[3].width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    let block = Block::default().borders(Borders::TOP);
    let input = Paragraph::new(app.input.value())
        .scroll((0, scroll as u16))
        .block(block);
    frame.render_widget(input, main_layout[3]);

    match app.input_mode {
        InputMode::Editing => {}
        InputMode::Logging => frame.set_cursor(
            main_layout[3].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16,
            main_layout[3].y + 1,
        ),
    }
}
