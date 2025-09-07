use crate::styling::DASH;
use serde::{Deserialize, Serialize};
use std::fs::DirEntry;
use std::str::FromStr;

const RFC3339_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

// Required, since the files on disk has very strange formats
#[derive(Deserialize, Serialize)]
struct FileEntry {
    #[serde(rename = "__CLASS__")]
    class: Option<String>,
    description: Option<String>,
    duration: Option<String>,
    project: String,
    seconds: Option<i64>,
    start: String,
    stop: Option<String>,
    tags: Vec<String>,
    user: Option<String>,
}

#[derive(Default)]
pub struct TrackedEntry {
    pub description: String,
    pub project: String,
    pub total_duration: Option<chrono::Duration>,
    pub start: chrono::NaiveDateTime,
    pub stop: Option<chrono::NaiveDateTime>,
    pub tags: Vec<String>,
}

impl From<FileEntry> for TrackedEntry {
    fn from(value: FileEntry) -> Self {
        let start = chrono::NaiveDateTime::parse_from_str(&value.start, RFC3339_FORMAT)
            .unwrap_or_else(|_| chrono::Local::now().naive_local());

        let stop = value
            .stop
            .and_then(|s| chrono::NaiveDateTime::parse_from_str(&s, RFC3339_FORMAT).ok());

        TrackedEntry {
            description: value.description.unwrap_or_default(),
            project: value.project,
            total_duration: None,
            start,
            stop,
            tags: value.tags,
        }
    }
}

impl TrackedEntry {
    fn as_file_entry(&self) -> FileEntry {
        let duration = self.duration();
        FileEntry {
            class: Some("App::TimeTracker::Data::Task".to_string()),
            description: Some(self.description.clone()),
            duration: Some(format!(
                "{:02}:{:02}:{:02}",
                duration.num_hours(),
                duration.num_minutes() % 60,
                duration.num_seconds() % 60
            )),
            project: self.project.clone(),
            seconds: Some(self.duration().num_seconds()),
            start: self.start.format(RFC3339_FORMAT).to_string(),
            stop: self.stop.map(|s| s.format(RFC3339_FORMAT).to_string()),
            tags: self.tags.clone(),
            user: std::env::var("USER").ok(),
        }
    }

    pub fn description(&self) -> &str {
        if self.description.is_empty() {
            DASH
        } else {
            &self.description
        }
    }

    pub fn duration(&self) -> chrono::Duration {
        self.stop
            .unwrap_or_else(|| chrono::Local::now().naive_local())
            - self.start
    }

    pub fn from_file(file: &DirEntry) -> Result<TrackedEntry, anyhow::Error> {
        let content = std::fs::read_to_string(file.path())?;
        let entry: FileEntry = serde_json::from_str(&content)?;
        Ok(entry.into())
    }

    pub fn matches_args(&self, args: &clap::ArgMatches) -> bool {
        if let Some(project) = args.get_one::<String>("project")
            && self.project != *project
        {
            return false;
        }

        if let Some(tag) = args.get_one::<String>("tag")
            && !self.tags.iter().any(|t| t == tag)
        {
            return false;
        }

        true
    }

    pub fn tags_as_string(&self) -> String {
        std::collections::HashSet::<&String>::from_iter(self.tags.iter())
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn file_entry_in_date_range(
    file_entry: &DirEntry,
    since: &chrono::NaiveDate,
    until: &chrono::NaiveDate,
) -> bool {
    fn to_int<T: FromStr>(
        file_name: &str,
        range: std::ops::Range<usize>,
    ) -> Result<T, <T as FromStr>::Err> {
        file_name.get(range).unwrap_or_default().parse::<T>()
    }

    let file_path = file_entry.path();
    let Some(file_name) = file_path
        .file_name()
        .map(|n| n.to_str().unwrap_or_default())
    else {
        return false;
    };

    let y: i32 = to_int(file_name, 0..4).unwrap_or_default();
    let m: u32 = to_int(file_name, 4..6).unwrap_or_default();
    let d: u32 = to_int(file_name, 6..8).unwrap_or_default();

    let Some(entry_date) = chrono::NaiveDate::from_ymd_opt(y, m, d) else {
        return false;
    };

    entry_date >= *since && entry_date <= *until
}

pub fn find_tracked_entries(
    since: &chrono::NaiveDate,
    until: &chrono::NaiveDate,
) -> Vec<TrackedEntry> {
    let mut all_entries = vec![];
    for year_dir in read_dir(tracker_dir()) {
        for month_dir in read_dir(year_dir.path()) {
            for file in read_dir(month_dir.path()) {
                if !file_entry_in_date_range(&file, since, until) {
                    continue;
                }

                let path = file.path();
                let ext = path.extension().and_then(|s| s.to_str());
                if ext != Some("trc") && ext != Some("json") {
                    continue;
                }

                if let Ok(entry) = TrackedEntry::from_file(&file) {
                    all_entries.push(entry);
                }
            }
        }
    }

    all_entries.sort_by_key(|a| a.start);
    all_entries
}

fn read_dir(path: impl AsRef<std::path::Path>) -> Vec<DirEntry> {
    std::fs::read_dir(path)
        .map(|rd| rd.filter_map(Result::ok).collect())
        .unwrap_or_default()
}

fn tracker_dir() -> String {
    let home = std::env::var("HOME").expect("Can't find ~/.TimeTracker, without  being set");
    format!("{}/.TimeTracker", home)
}
