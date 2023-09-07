use clap::{Parser, Subcommand};
use lipu_tenpo::app::App;
/// This example is taken from https://raw.githubusercontent.com/fdehau/tui-rs/master/examples/user_input.rs
use lipu_tenpo::event::{Event, EventHandler};
use lipu_tenpo::handler;
use lipu_tenpo::tui::Tui;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::process::exit;
use std::{error::Error, io};

#[derive(Parser)]
#[command(author, version, about, long_about = None, infer_subcommands(true))]
struct Cli {
    #[arg(short, long, value_name = "CONF_FILE")]
    config: Option<String>,

    #[arg(short, long, value_name = "LOG_FILE")]
    log: Option<String>,

    #[arg(long, help = "Print csv")]
    csv: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Log { date: Option<Vec<String>> },
    Add { entry: Option<Vec<String>> },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Create the application
    let mut app = App::new(cli.log.unwrap_or_default(), cli.config.unwrap_or_default());

    match cli.command {
        Some(Commands::Log { date }) => {
            lipu_tenpo::subcommands::log(&app, date, cli.csv);
            exit(0);
        }
        Some(Commands::Add { entry }) => {
            lipu_tenpo::subcommands::add(&app, entry);
            exit(0);
        }
        None => {}
    }

    app.refresh();

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
