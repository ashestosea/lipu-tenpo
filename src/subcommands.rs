use std::path::PathBuf;

use chrono::{NaiveDate, NaiveTime};

use crate::entries;

pub fn summary(log_path: &PathBuf, current_date: NaiveDate, virtual_midnight: NaiveTime) {
    match entries::read_all_date(log_path, current_date, virtual_midnight) {
        Ok(c) => {
            println!("--{}--", current_date);
            for entry in c.entries.iter() {
                println!("{}", entry);
            }
        }
        Err(e) => println!("{}", e),
    }
}
