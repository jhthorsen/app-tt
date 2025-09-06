use serde::Deserialize;
use std::fs::DirEntry;
use std::str::FromStr;

#[derive(Debug, Default, Deserialize)]
pub struct TrackedEntry {
    pub description: Option<String>,
    pub project: String,
    #[serde(skip_deserializing)]
    pub total_duration: Option<chrono::Duration>,
    pub start: chrono::NaiveDateTime,
    pub stop: Option<chrono::NaiveDateTime>,
    pub tags: Vec<String>,
}

impl TrackedEntry {
    pub fn description(&self) -> &str {
        if let Some(d) = &self.description
            && !d.is_empty()
        {
            return d;
        }

        crate::styling::DASH
    }

    pub fn duration(&self) -> chrono::Duration {
        self.stop
            .unwrap_or_else(|| chrono::Local::now().naive_local())
            - self.start
    }

    pub fn from_file(file: &DirEntry) -> Result<TrackedEntry, anyhow::Error> {
        let content = std::fs::read_to_string(file.path())?;
        let entry: TrackedEntry = serde_json::from_str(&content)?;
        Ok(entry)
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
