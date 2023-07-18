use clap::{Parser, Subcommand};
use lipu_tenpo::app::App;
/// This example is taken from https://raw.githubusercontent.com/fdehau/tui-rs/master/examples/user_input.rs
use lipu_tenpo::event::{Event, EventHandler};
use lipu_tenpo::handler;
use lipu_tenpo::tui::Tui;
use std::process::exit;
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Parser)]
#[command(author, version, about, long_about = None, infer_subcommands(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, value_name = "CONF_FILE")]
    config: Option<String>,

    #[arg(short, long, value_name = "LOG_FILE")]
    log: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Summary,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Create the application
    let mut app = App::new(cli.log.unwrap_or_default(), cli.config.unwrap_or_default());

    match &cli.command {
        Some(Commands::Summary) => {
            lipu_tenpo::subcommands::summary(
                &app.log_path(),
                app.current_date,
                app.config.virtual_midnight,
            );
            exit(0);
        },
        None => {}
    }

    app.load_entries()?;

    // Initialize the terminal user interface
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(5);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Run the app
    while app.running {
        // Render the UI
        tui.draw(&mut app)?;
        // Handle events
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Mouse(_) => {}
            Event::Key(key) => {
                handler::handle_key_events(&mut app, key);
            }
            Event::Resize(_, _) => {}
        }
    }

    // restore terminal
    tui.exit()?;

    Ok(())
}

// tui-input ui example
// fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(2)
//         .constraints(
//             [
//                 Constraint::Length(1),
//                 Constraint::Length(3),
//                 Constraint::Min(1),
//             ]
//             .as_ref(),
//         )
//         .split(f.size());

//     let (msg, style) = match app.input_mode {
//         InputMode::Editing => (
//             vec![
//                 Span::raw("Press "),
//                 Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to exit, "),
//                 Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to start editing."),
//             ],
//             Style::default().add_modifier(Modifier::RAPID_BLINK),
//         ),
//         InputMode::Logging => (
//             vec![
//                 Span::raw("Press "),
//                 Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to stop editing, "),
//                 Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" to record the message"),
//             ],
//             Style::default(),
//         ),
//     };
//     let mut text = Text::from(Line::from(msg));
//     text.patch_style(style);
//     let help_message = Paragraph::new(text);
//     f.render_widget(help_message, chunks[0]);

//     let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

//     let scroll = app.input.visual_scroll(width as usize);
//     let input = Paragraph::new(app.input.value())
//         .style(match app.input_mode {
//             InputMode::Editing => Style::default(),
//             InputMode::Logging => Style::default().fg(Color::Yellow),
//         })
//         .scroll((0, scroll as u16))
//         .block(Block::default().borders(Borders::ALL).title("Input"));
//     f.render_widget(input, chunks[1]);
//     match app.input_mode {
//         InputMode::Editing =>
//             // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
//             {}

//         InputMode::Logging => {
//             // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
//             f.set_cursor(
//                 // Put cursor past the end of the input text
//                 chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
//                 // Move one line down, from the border to the input line
//                 chunks[1].y + 1,
//             )
//         }
//     }

//     let messages: Vec<ListItem> = app
//         .messages
//         .iter()
//         .enumerate()
//         .map(|(i, m)| {
//             let content = vec![Line::from(Span::raw(format!("{}: {}", i, m)))];
//             ListItem::new(content)
//         })
//         .collect();
//     let messages =
//         List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
//     f.render_widget(messages, chunks[2]);
// }
