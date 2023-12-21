use std::{
    error::Error,
    fmt::Display,
    ops::{Add, Sub},
    path::PathBuf,
};

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use csv::ByteRecord;
use ratatui::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct EntryRaw {
    #[serde(with = "naive_date_time")]
    pub end: NaiveDateTime,
    pub project: String,
    pub activity: String,
    #[serde(with = "string_vector")]
    pub tags: Vec<String>,
}

impl From<&Entry> for EntryRaw {
    fn from(value: &Entry) -> Self {
        Self {
            end: value.end,
            project: value.project.clone(),
            activity: value.activity.clone(),
            tags: value.tags.clone(),
        }
    }
}

impl From<Entry> for EntryRaw {
    fn from(value: Entry) -> Self {
        Self {
            end: value.end,
            project: value.project,
            activity: value.activity,
            tags: value.tags,
        }
    }
}

impl PartialOrd for EntryRaw {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.end.cmp(&other.end))
    }
}

impl Ord for EntryRaw {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end.cmp(&other.end)
    }
}

impl EntryRaw {
    pub fn from_string(value: String, fallback_datetime: NaiveDateTime) -> EntryRaw {
        let (dt, rest) = split_time_and_entry(value, fallback_datetime);
        let datetime = dt.unwrap_or(fallback_datetime);

        let (first, tags) = rest.split_once('+').unwrap_or((rest.as_str(), ""));
        let (project, activity) = first.split_once(':').unwrap_or(("", first));

        EntryRaw {
            end: datetime,
            project: String::from(project),
            activity: String::from(activity),
            tags: tags
                .split('+')
                .map(|f| -> String { String::from(f) })
                .collect(),
        }
    }

    fn effective_date(&self, virtual_midnight: NaiveTime) -> NaiveDate {
        match self.end.time() < virtual_midnight {
            true => self.end.date().pred_opt().unwrap(),
            false => self.end.date(),
        }
    }
}

pub fn split_time_and_entry(
    value: String,
    datetime: NaiveDateTime,
) -> (Option<NaiveDateTime>, String) {
    let date = datetime.date();

    if let Some((time, rest)) = value.split_once(' ') {
        if let Ok(time) = NaiveTime::parse_from_str(time, "%H:%M") {
            return (Some(NaiveDateTime::new(date, time)), rest.to_string());
        } else if let Some(time) = time.strip_prefix('-') {
            if let Ok(time) = NaiveTime::parse_from_str(time, "%H:%M") {
                let neg_dur = Duration::minutes(time.minute() as i64);
                return (Some(datetime.sub(neg_dur)), rest.to_string());
            } else if let Ok(time) = time.parse::<i64>() {
                let neg_dir = Duration::minutes(time);
                return (Some(datetime.sub(neg_dir)), rest.to_string());
            }
        }
    }

    (None, value)
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct EntryTitle(String);

impl Display for EntryTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&Entry> for EntryTitle {
    fn from(value: &Entry) -> Self {
        Self(value.display_sans_time())
    }
}

impl From<&EntryTitle> for String {
    fn from(value: &EntryTitle) -> Self {
        value.0.clone()
    }
}
impl From<String> for EntryTitle {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl<'a> From<&'a str> for EntryTitle {
    fn from(s: &'a str) -> Self {
        EntryTitle(s.to_string())
    }
}

impl From<EntryTitle> for String {
    fn from(a: EntryTitle) -> Self {
        a.0
    }
}

impl<'a> From<&'a EntryTitle> for &'a str {
    fn from(a: &'a EntryTitle) -> Self {
        &a.0
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    /// The project under which the activity was performed
    /// Can be an empty string
    pub project: String,
    /// The activity performed.
    /// Can't be an empty string
    pub activity: String,
    pub tags: Vec<String>,
}

impl From<&Entry> for Line<'_> {
    fn from(val: &Entry) -> Self {
        if val.is_on_task() {
            Line::styled(format!("{}", val), Style::new())
        } else {
            Line::styled(
                format!("{}", val),
                Style::default().add_modifier(Modifier::DIM),
            )
        }
    }
}

impl From<&Entry> for Text<'_> {
    fn from(val: &Entry) -> Self {
        if val.is_on_task() {
            Text::raw(format!("{}", val))
        } else {
            Text::styled(
                format!("{}", val),
                Style::default().add_modifier(Modifier::DIM),
            )
        }
    }
}

impl From<&Entry> for String {
    fn from(value: &Entry) -> String {
        let duration = value.duration();
        let duration_display = if duration.is_zero() {
            String::new()
        } else {
            format!("{}h {}m", duration.num_hours(), duration.num_minutes() % 60)
        };

        format!(
            "{:<8} {:<6} {}",
            duration_display,
            value.end.format("%H:%M"),
            value.display_sans_time()
        )
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = self.duration();
        let duration_display = if duration.is_zero() {
            String::new()
        } else {
            format!("{}h {}m", duration.num_hours(), duration.num_minutes() % 60)
        };

        write!(
            f,
            "{:<8} {:<6} {}",
            duration_display,
            self.end.format("%H:%M"),
            self.display_sans_time()
        )
    }
}

impl Entry {
    pub fn from_raw_previous(raw: &EntryRaw, previous_entry: &EntryRaw) -> Entry {
        Entry {
            start: previous_entry.end,
            end: raw.end,
            project: raw.project.clone(),
            activity: raw.activity.clone(),
            tags: raw.tags.clone(),
        }
    }

    /// Returns Entry with project, activity, and tags initiated from the EntryRaw.
    ///
    /// Start and end times are both set to the EntryRaw's end time.
    pub fn from_raw(raw: &EntryRaw) -> Entry {
        Entry {
            start: raw.end,
            end: raw.end,
            project: raw.project.clone(),
            activity: raw.activity.clone(),
            tags: raw.tags.clone(),
        }
    }

    pub fn in_project(mut self, project: String) -> Entry {
        self.project = project;
        self
    }

    pub fn with_activity(mut self, activity: String) -> Entry {
        self.activity = activity;
        self
    }

    pub fn add_tag(mut self, tag: String) -> Entry {
        self.tags.push(tag);
        self
    }

    pub fn add_tags(mut self, tags: &mut Vec<String>) -> Entry {
        self.tags.append(tags);
        self
    }

    pub fn with_start(mut self, datetime: NaiveDateTime) -> Entry {
        self.start = datetime;
        self
    }

    pub fn with_end(mut self, datetime: NaiveDateTime) -> Entry {
        self.end = datetime;
        self
    }

    pub fn append(previous_entry: &Entry) -> Entry {
        Entry {
            start: previous_entry.end,
            end: chrono::Local::now().naive_local(),
            activity: String::new(),
            project: String::new(),
            tags: Vec::new(),
        }
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    pub fn is_on_task(&self) -> bool {
        !self.activity.contains("**")
    }

    pub fn display_sans_time(&self) -> String {
        if self.project.is_empty() {
            self.activity.to_string()
        } else {
            format!("{}: {}", self.project, self.activity)
        }
    }
}

#[derive(Clone)]
pub struct EntryGroup {
    pub entries: Vec<Entry>,
    time_on_task: Duration,
    time_off_task: Duration,
}

impl Default for EntryGroup {
    fn default() -> Self {
        EntryGroup {
            entries: vec![],
            time_on_task: Duration::seconds(0),
            time_off_task: Duration::seconds(0),
        }
    }
}

impl EntryGroup {
    pub fn new(entries: Vec<Entry>) -> EntryGroup {
        let mut on_task = Duration::seconds(0);
        let mut off_task = Duration::seconds(0);

        for entry in &entries {
            if entry.is_on_task() {
                on_task = on_task.add(entry.duration());
            } else {
                off_task = off_task.add(entry.duration());
            }
        }

        EntryGroup {
            entries,
            time_on_task: on_task,
            time_off_task: off_task,
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.len() == 0
    }

    pub fn time_on_task_display(&self) -> String {
        format!(
            "{}h {}m",
            self.time_on_task.num_hours(),
            self.time_on_task.num_minutes() % 60
        )
    }

    pub fn time_off_task_display(&self) -> String {
        format!(
            "{}h {}m",
            self.time_off_task.num_hours(),
            self.time_off_task.num_minutes() % 60
        )
    }

    pub fn time_since_last_display(&self) -> Option<String> {
        if let Some(entry) = self.entries.last() {
            let diff = chrono::Local::now().naive_local().time() - entry.end.time();
            if diff > Duration::zero() {
                return Some(format!(
                    "{}h {}m",
                    diff.num_hours(),
                    diff.num_minutes() % 60
                ));
            }
        }

        None
    }
}

pub fn read_all_date(
    log_contents: &String,
    date: NaiveDate,
    virtual_midnight: NaiveTime,
) -> Result<EntryGroup, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .quoting(true)
        .trim(csv::Trim::All)
        .from_reader(log_contents.as_bytes());

    let read_results: Result<Vec<EntryRaw>, csv::Error> = reader
        .deserialize()
        .filter(|f: &Result<EntryRaw, csv::Error>| match f {
            Ok(ent) => ent.effective_date(virtual_midnight) == date,
            Err(_) => false,
        })
        .collect();

    let raw_entries = match read_results {
        Ok(x) => x,
        Err(error) => panic!("Read error {:?}", error),
    };

    let count = raw_entries.len();
    let mut entries = vec![Entry::default(); count];

    for i in 0..count {
        if i == 0 {
            entries[i] = Entry::from_raw(&raw_entries[i]);
        } else {
            entries[i] = Entry::from_raw_previous(&raw_entries[i], &raw_entries[i - 1]);
        }
    }

    Ok(EntryGroup::new(entries))
}

pub fn read_all(path: &PathBuf) -> Result<EntryGroup, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .quoting(true)
        .trim(csv::Trim::All)
        .from_path(path)?;

    let read_results: Result<Vec<EntryRaw>, csv::Error> = reader.deserialize().collect();

    let mut raw_entries = match read_results {
        Ok(x) => x,
        Err(error) => panic!("Read error {:?}", error),
    };

    raw_entries.sort();

    let count = raw_entries.len();
    let mut entries = vec![Entry::default(); count];

    for i in 0..count {
        if i == 0 {
            entries[i] = Entry::from_raw(&raw_entries[i]);
        } else {
            entries[i] = Entry::from_raw_previous(&raw_entries[i], &raw_entries[i - 1]);
        }
    }

    Ok(EntryGroup::new(entries))
}

pub fn read_all_from_string(log_contents: &String) -> Result<EntryGroup, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .quoting(true)
        .trim(csv::Trim::All)
        .from_reader(log_contents.as_bytes());

    let read_results: Result<Vec<EntryRaw>, csv::Error> = reader.deserialize().collect();

    let mut raw_entries = match read_results {
        Ok(x) => x,
        Err(error) => panic!("Read error {:?}", error),
    };

    raw_entries.sort();

    let count = raw_entries.len();
    let mut entries = vec![Entry::default(); count];

    for i in 0..count {
        if i == 0 {
            entries[i] = Entry::from_raw(&raw_entries[i]);
        } else {
            entries[i] = Entry::from_raw_previous(&raw_entries[i], &raw_entries[i - 1]);
        }
    }

    Ok(EntryGroup::new(entries))
}

pub fn write(app: &App, entry: EntryRaw) -> Result<(), Box<dyn Error>> {
    let log_path = app.log_path();
    let mut path_string = log_path.clone().into_os_string();
    path_string.push("-tmp");
    let temp_path: PathBuf = path_string.into();
    let mut entries_raw: Vec<EntryRaw> = read_all(&log_path)?
        .entries
        .iter()
        .map(EntryRaw::from)
        .collect();
    entries_raw.push(entry);
    entries_raw.sort();
    write_to(
        &log_path,
        &temp_path,
        &entries_raw,
        app.config.virtual_midnight,
    )?;
    Ok(())
}

pub fn write_to(
    path: &PathBuf,
    temp_path: &PathBuf,
    entries: &[EntryRaw],
    virtual_midnight: NaiveTime,
) -> Result<(), std::io::Error> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .quote_style(csv::QuoteStyle::Never)
        .flexible(true)
        .from_path(temp_path)?;

    for i in 0..entries.len() {
        // If this Entry is effectively the start of the day, add some line breaks before
        if i > 0
            && entries[i].effective_date(virtual_midnight)
                != entries[i - 1].effective_date(virtual_midnight)
        {
            writer.write_field("\n")?;
            writer.write_byte_record(&ByteRecord::new())?;
        }
        writer.serialize(&entries[i])?;
    }

    // for entry in entries {
    //     if entry.end.time().hour() > 2 {
    //         writer.write_byte_record(&ByteRecord::new())?;
    //     }
    //     writer.serialize(EntryRaw::from(entry))?;
    // }

    std::fs::rename(temp_path, path)
}

mod naive_date_time {
    use super::*;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(dt.format("%Y-%m-%d %H:%M").to_string().as_str())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let dt = Deserialize::deserialize(deserializer)?;
        Ok(NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M")
            .map_err(D::Error::custom)
            .unwrap())
    }
}

mod string_vector {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(vector: &[String], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&vector.join(", "))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() {
            Ok(Vec::<String>::new())
        } else {
            Ok(s.split(',').map(ToOwned::to_owned).collect::<Vec<String>>())
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use chrono::NaiveTime;

    use super::read_all;
    use super::read_all_date;
    use std::path::PathBuf;

    #[test]
    fn test_read_bad_file() {
        assert!(read_all(&PathBuf::from("./non-existant-file-for-testing-lipu-tenpo")).is_err());
    }

    #[test]
    fn test_read_good_file() {
        let result = read_all(&PathBuf::from("./test/test.csv"));
        let entries = result.unwrap_or_default().entries;

        // for e in &entries {
        //     println!("{}", String::from(e));
        // }

        assert_eq!(entries.len(), 9);
        assert_eq!(entries[0].activity, "**arrive");
    }

    #[test]
    fn test_date_read_good_file() {
        let log_contents = std::fs::read_to_string(&PathBuf::from("./test/test.csv")).unwrap();
        let result = read_all_date(
            &log_contents,
            NaiveDate::from_ymd_opt(2023, 6, 14).unwrap(),
            NaiveTime::from_hms_opt(2, 0, 0).unwrap(),
        );
        let entries = result.unwrap_or_default().entries;

        for e in &entries {
            println!("{}", e);
        }

        assert_eq!(entries.len(), 5);
        assert_eq!(entries[0].activity, "**arrive");
    }
}
