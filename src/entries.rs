use std::{error::Error, path::PathBuf};

use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use tui::{
    style::{Modifier, Style},
    text::{Span, Text},
};

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

impl From<String> for Entry {
    fn from(value: String) -> Entry {
        let now =
            chrono::NaiveDateTime::from_timestamp_millis(chrono::Local::now().timestamp_millis())
                .unwrap();
        let (first, tags) = value.split_once('+').unwrap_or_else(|| (value.as_str(), ""));
        let (project, activity) = first.split_once(':').unwrap_or_else(|| ("", first));
        Entry {
            start: now,
            end: now,
            project: String::from(project),
            activity: String::from(activity),
            tags: tags.split("+").map(|f| -> String {String::from(f)}).collect(),
        }
    }
}

impl From<&Entry> for String {
    fn from(val: &Entry) -> String {
        let duration = val.duration();
        let duration_str = format!("{}h {}m", duration.num_hours(), duration.num_minutes() % 60);
        if duration.is_zero() {}
        if val.project.is_empty() {
            format!(
                "{:<8} {:<6} {}",
                duration_str,
                val.end.format("%H:%M"),
                val.activity
            )
        } else {
            format!(
                "{:<8} {:<6} {}: {}",
                duration_str,
                val.end.format("%H:%M"),
                val.project,
                val.activity
            )
        }
    }
}

impl From<&Entry> for Span<'_> {
    fn from(val: &Entry) -> Self {
        if val.is_on_task() {
            Span::raw(String::from(val))
        } else {
            Span::styled(
                String::from(val),
                Style::default().add_modifier(Modifier::DIM),
            )
        }
    }
}

impl From<&Entry> for Text<'_> {
    fn from(val: &Entry) -> Self {
        if val.is_on_task() {
            Text::raw(String::from(val))
        } else {
            Text::styled(
                String::from(val),
                Style::default().add_modifier(Modifier::DIM),
            )
        }
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
}

pub fn read_all_from(path: &PathBuf) -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .quoting(true)
        .trim(csv::Trim::All)
        .from_path(path)?;

    let read_results: Result<Vec<EntryRaw>, Box<dyn Error>> = reader
        .deserialize()
        .map(|e| -> Result<EntryRaw, Box<dyn Error>> { Ok(e?) })
        .collect();

    let raw_entries = match read_results {
        Ok(x) => x,
        Err(error) => panic!("Read error {:?}", error),
    };
    // let err = read_results.err();
    // let raw_entries = read_results.unwrap_or_default();
    let count = raw_entries.len();
    let mut entries = vec![Entry::default(); count];

    for i in 0..count {
        if i == 0 {
            entries[i] = Entry::from_raw(&raw_entries[i]);
        } else {
            entries[i] = Entry::from_raw_previous(&raw_entries[i], &raw_entries[i - 1]);
        }
    }

    Ok(entries)
}

pub fn write_to(
    path: &PathBuf,
    temp_path: &PathBuf,
    entries: &[Entry],
) -> Result<(), std::io::Error> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_path(temp_path)?;

    for entry in entries {
        writer.serialize(EntryRaw::from(entry))?;
    }

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
    use super::read_all_from;
    use std::path::PathBuf;

    #[test]
    fn test_read_bad_file() {
        assert!(
            read_all_from(&PathBuf::from("./non-existant-file-for-testing-lipu-tenpo")).is_err()
        );
    }

    #[test]
    fn test_read_good_file() {
        let result = read_all_from(&PathBuf::from("./test.csv"));
        let entries = result.unwrap_or_default();
        
        for e in &entries {
            println!("{}", String::from(e));
        }
        
        assert_eq!(entries.len(), 9);
        assert_eq!(entries[0].activity, "**arrive");
    }
}
