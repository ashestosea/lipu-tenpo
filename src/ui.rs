use chrono::Datelike;
use ratatui::{prelude::*, widgets::*};

use crate::app::{App, InputMode};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let entry_group = &app.current_entries;

    let root_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints(
            [
                Constraint::Max(1),
                Constraint::Min(2),
                Constraint::Max(2),
                Constraint::Max(3),
                Constraint::Max(1),
            ]
            .as_ref(),
        )
        .split(root_layout[0]);

    let date_area = main_layout[0];
    let log_area = main_layout[1];
    let summary_area = main_layout[2];
    let input_area = main_layout[3];
    let hotkeys_area = main_layout[4];

    // Date
    let current_date = app.current_date;
    let is_today = current_date == chrono::Local::now().naive_local().date();
    let is_today_str = if is_today { "@" } else { "" };
    let date_style = if is_today {
        Style::default().bg(Color::Magenta).fg(Color::Black)
    } else {
        Style::default().bg(Color::Gray).fg(Color::Black)
    };

    let title_block = Block::default()
        .borders(Borders::BOTTOM)
        .title(format!(
            "─{}─{}──{}",
            is_today_str,
            current_date.weekday(),
            &current_date.format("%Y─%m─%d")
        ))
        .style(date_style);
    frame.render_widget(title_block, date_area);

    // Log
    let mut log_items: Vec<String> = entry_group.entries.iter().map(String::from).collect();

    let show_scrollbar = log_items.len() >= log_area.height.into();
    let scrollbar_constraint = if show_scrollbar { 5 } else { 0 };

    let log_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Min(1), Constraint::Max(scrollbar_constraint)].as_ref())
        .split(log_area);

    let log_block = Block::default()
        .padding(Padding {
            left: 1,
            right: 1,
            bottom: 1,
            top: 0,
        })
        .borders(
            Borders::LEFT
                | if show_scrollbar {
                    Borders::NONE
                } else {
                    Borders::RIGHT
                },
        )
        .border_type(BorderType::Rounded);
    if let Some(mut time_since_last) = entry_group.time_since_last_display() {
        time_since_last.insert_str(0, "> ");
        log_items.push(time_since_last);
    }

    let para: Paragraph = Paragraph::new(log_items.join("\n"))
        .scroll((app.log_scroll as u16, 0))
        .block(log_block);
    frame.render_widget(para, log_layout[0]);

    let log_scrollbar = Scrollbar::default()
            // .orientation(ScrollbarOrientation::VerticalRight)
            // .begin_symbol(Some("↑"))
            // .end_symbol(Some("↓"))
            // .thumb_symbol("-")
            // .track_symbol(Some("|"))
            ;
    if show_scrollbar {
        frame.render_stateful_widget(log_scrollbar, log_layout[1], &mut app.scroll_state);
    }

    // Summary
    let summary_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(summary_area);

    let work_summary_block = Block::default()
        .borders(Borders::BOTTOM | Borders::LEFT)
        .border_type(BorderType::Rounded)
        .padding(Padding {
            left: 1,
            ..Default::default()
        });
    let work_summary = Paragraph::new(format!("On task: {}", entry_group.time_on_task_display()))
        .block(work_summary_block);
    frame.render_widget(work_summary, summary_layout[0]);

    let other_summary_block = Block::default()
        .borders(Borders::BOTTOM | Borders::RIGHT)
        .border_type(BorderType::Rounded)
        .padding(Padding {
            right: 1,
            ..Default::default()
        });
    let other_summary = Paragraph::new(format!("Other: {}", entry_group.time_off_task_display()))
        .alignment(Alignment::Right)
        .block(other_summary_block);
    frame.render_widget(other_summary, summary_layout[1]);

    // Input
    let width = input_area.width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    let input_block = Block::default()
        .padding(Padding::horizontal(1))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let input = Paragraph::new(app.current_log.as_str())
        .scroll((0, scroll as u16))
        .block(input_block);
    frame.render_widget(input, input_area);

    match app.input_mode {
        InputMode::Editing => {}
        InputMode::Logging => frame.set_cursor(
            input_area.x + 2 + ((app.input.visual_cursor()).max(scroll) - scroll) as u16,
            // input_area.x + 2 + app.current_log.len() as u16,
            input_area.y + 1,
        ),
    }

    // Hotkeys
    let hotkeys_block = Block::default().padding(Padding::horizontal(1)).dark_gray();
    let hotkeys_help =
        Paragraph::new("Ctrl+Left/Right: Prev/Next day, Ctrl+Home: Today").block(hotkeys_block);
    frame.render_widget(hotkeys_help, hotkeys_area);
}
