use std::path::PathBuf;

use chrono::{NaiveDate, NaiveTime};

use crate::entries;

pub fn summary(log_path: &PathBuf, date: &Option<String>, virtual_midnight: NaiveTime) {
    let date: NaiveDate = match date {
        Some(date) => {
            if let Ok(d) = fuzzydate::parse(date) {
                d.date()
            } else {
                println!("Couldn't parse date \"{}\"", date);
                return;
            }
        }
        None => chrono::Local::now().date_naive(),
    };

    match entries::read_all_date(log_path, date, virtual_midnight) {
        Ok(c) => {
            println!("--{}--", date);
            for entry in c.entries.iter() {
                println!("{}", entry);
            }
        }
        Err(e) => println!("{}", e),
    }
}
