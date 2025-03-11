use chrono::Datelike;
use ratatui::{prelude::*, widgets::*};

use crate::app::{App, InputMode};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame<'_>) {
    let entry_group = &app.current_entries;

    let root_layout = Layout::vertical([Constraint::Fill(1)]).margin(1);

    let [main_area] = root_layout.areas(frame.size());
    let main_layout = Layout::vertical([
        Constraint::Max(1),
        Constraint::Min(2),
        Constraint::Max(2),
        Constraint::Max(3),
        Constraint::Max(1),
    ])
    .horizontal_margin(1);

    let [date_area, log_area, summary_area, input_area, hotkeys_area] =
        main_layout.areas(main_area);

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
    let mut log_items: Vec<Line> = entry_group
        .entries
        .iter()
        .map(ratatui::text::Line::from)
        .collect();

    let show_scrollbar = log_items.len() >= log_area.height.into();
    let scrollbar_constraint = if show_scrollbar { 5 } else { 0 };

    let log_layout =
        Layout::horizontal([Constraint::Min(1), Constraint::Max(scrollbar_constraint)]);
    let [log_body_area, log_scrollbar_area] = log_layout.areas(log_area);

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
        log_items.push(Line::from(time_since_last));
    }

    // let para: Paragraph = Paragraph::new(log_items.join("\n"))
    let para: Paragraph = Paragraph::new(Text::from(log_items))
        .scroll((app.log_scroll as u16, 0))
        .block(log_block);
    frame.render_widget(para, log_body_area);

    let log_scrollbar = Scrollbar::default();
    if show_scrollbar {
        frame.render_stateful_widget(log_scrollbar, log_scrollbar_area, &mut app.scroll_state);
    }

    // Summary
    let summary_layout = Layout::horizontal(Constraint::from_percentages([50, 50]));
    let [work_summary_area, other_summary_area] = summary_layout.areas(summary_area);

    let work_summary_block = Block::default()
        .borders(Borders::BOTTOM | Borders::LEFT)
        .border_type(BorderType::Rounded)
        .padding(Padding {
            left: 1,
            ..Default::default()
        });
    let work_summary = Paragraph::new(format!("On task: {}", entry_group.time_on_task_display()))
        .block(work_summary_block);
    frame.render_widget(work_summary, work_summary_area);

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
    frame.render_widget(other_summary, other_summary_area);

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
        InputMode::Logging => frame.set_cursor_position(Position::new(
            input_area.x + 2 + ((app.input.visual_cursor()).max(scroll) - scroll) as u16,
            // input_area.x + 2 + app.current_log.len() as u16,
            input_area.y + 1,
        )),
    }

    // Hotkeys
    let hotkeys_block = Block::default().padding(Padding::horizontal(1)).dark_gray();
    let hotkeys_help =
        Paragraph::new("Ctrl+Left/Right: Prev/Next day, Ctrl+Home: Today").block(hotkeys_block);
    frame.render_widget(hotkeys_help, hotkeys_area);
}
