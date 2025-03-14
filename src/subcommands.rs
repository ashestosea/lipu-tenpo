use std::{error::Error, io::Write};

use chrono::NaiveDate;

use crate::{
    app::App,
    entries::{self, EntryGroup},
};

pub fn log(app: &App, date: Option<Vec<String>>, csv_print: bool) -> Result<(), Box<dyn Error>> {
    let date = date
        .unwrap_or(vec![chrono::Local::now().date_naive().to_string()])
        .join(" ");

    let date_time = fuzzydate::parse(&date)?;
    let date = date_time.date();
    let entry_group =
        entries::read_all_date(&app.log_contents(), date, app.config.virtual_midnight)?;

    if csv_print {
        log_csv(entry_group)
    } else {
        log_pretty(date, entry_group)
    }
}

fn log_csv(entry_group: EntryGroup) -> Result<(), Box<dyn Error>> {
    for entry in entry_group.entries.iter() {
        std::io::stdout().write_all(
            format!(
                "\"{}\", \"{}\", \"{}\", \"{}\"\n",
                &entry.end.to_string(),
                &entry.project,
                &entry.activity,
                &entry.tags.join(",")
            )
            .as_bytes(),
        )?;
    }
    Ok(())
}

fn log_pretty(date: NaiveDate, entry_group: EntryGroup) -> Result<(), Box<dyn Error>> {
    std::io::stdout().write_all(format!("--{}--\n", date).as_bytes())?;
    for entry in entry_group.entries.iter() {
        std::io::stdout().write_all(format!("{}\n", entry).as_bytes())?;
    }
    std::io::stdout()
        .write_all(format!("On task: {}\n", entry_group.time_on_task_display()).as_bytes())?;
    std::io::stdout()
        .write_all(format!("Other: {}\n", entry_group.time_off_task_display()).as_bytes())?;
    Ok(())
}

pub fn add(app: &App, entry: Option<Vec<String>>) {
    if let Some(entry) = entry {
        app.add_log(entry.join(" "))
            .expect("Error adding log entry")
    }
}
