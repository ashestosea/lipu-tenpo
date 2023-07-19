use crate::{app::App, entries};

pub fn log(app: &App, date: Option<Vec<String>>) {
    let date = match date {
        Some(date_vec) => {
            let date_str = date_vec.join(" ");
            if let Ok(d) = fuzzydate::parse(&date_str) {
                d.date()
            } else {
                println!("Couldn't parse date \"{}\"", date_str);
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

pub fn add(app: &App, entry: Option<Vec<String>>) {
    if let Some(entry) = entry {
        app.add_entry(entry.join(" "));
    }
}
