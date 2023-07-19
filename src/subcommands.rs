use chrono::NaiveDate;

use crate::{app::App, entries};

pub fn summary(app: &App, date: Option<String>) {
    let date: NaiveDate = match date {
        Some(date_val) => {
            if let Ok(d) = fuzzydate::parse(&date_val) {
                d.date()
            } else {
                println!("Couldn't parse date \"{}\"", date_val);
                return;
            }
        }
        None => chrono::Local::now().date_naive(),
    };

    match entries::read_all_date(&app.log_path(), date, app.config.virtual_midnight) {
        Ok(c) => {
            println!("--{}--", date);
            for entry in c.entries.iter() {
                println!("{}", entry);
            }
        }
        Err(e) => println!("{}", e),
    }
}

pub fn log(app: &App, entry: Option<String>) {
    app.add_entry(entry.unwrap_or_default());
}
