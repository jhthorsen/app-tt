use crate::entries::{TrackedEntry, find_last_tracked_entry};
use crate::styling::{plain_table, print_table};
use crate::utils::{default_project, format_date, min_duration, to_naive_date_time};
use clap::{Arg, Command};
use prettytable::{Table, row};

pub fn command() -> clap::Command {
    Command::new("start")
        .about("Start tracking time")
        .arg(
            Arg::new("start_time")
                .help("The start time for tracking (e.g., '08:00')")
                .default_value("now")
                .index(1),
        )
        .arg(
            Arg::new("project")
                .help("Project name")
                .short('p')
                .long("project"),
        )
        .arg(
            Arg::new("tag")
                .help("Tag(s) for the time entry")
                .short('t')
                .long("tag"),
        )
        .arg(
            Arg::new("description")
                .help("Time entry description")
                .short('d')
                .long("description"),
        )
        .arg(
            Arg::new("resume")
                .help("Resume if stopped")
                .long("resume")
                .num_args(0..=1)
                .default_missing_value("300")
                .value_parser(clap::value_parser!(i64)),
        )
}

fn not_too_old_to_resume(entry: &TrackedEntry, max_age: i64) -> bool {
    if let Some(stop) = entry.stop {
        let now = chrono::Local::now().naive_local();
        let diff = now - stop;
        diff.num_seconds() <= max_age
    } else {
        true
    }
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let start = to_naive_date_time(args.get_one::<String>("start_time"), None)?;

    // Stop current entry if not already stopped
    if !args.contains_id("resume")
        && let Ok(mut last) = find_last_tracked_entry()
        && last.stop.is_none()
    {
        last.stop = Some(start);
        if last.duration().num_seconds() < min_duration()? {
            last.delete()?;
        } else {
            last.save()?;
        }
    }

    let tags = if let Some(tag) = args.get_one::<String>("tag") {
        tag.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec![]
    };

    let mut status = "Started";

    let project = args
        .get_one::<String>("project")
        .unwrap_or(&default_project())
        .to_owned();

    let entry = if let Some(max_age) = args.get_one::<i64>("resume")
        && let Ok(mut last) = find_last_tracked_entry()
        && not_too_old_to_resume(&last, *max_age)
        && last.project == project
    {
        status = "Resumed";
        last.stop = None;
        last.tags.extend(tags);
        last
    } else {
        TrackedEntry {
            description: args
                .get_one::<String>("description")
                .cloned()
                .unwrap_or_default(),
            project,
            start,
            stop: None,
            tags,
            total_duration: None,
        }
    };

    entry.save()?;

    let mut summary = Table::new();
    summary.add_row(row!["Status", status]);
    summary.add_row(row!["Project", &entry.project]);
    summary.add_row(row!["Start", format_date(&entry.start, "full")]);
    summary.add_row(row!["Tags", &entry.tags_as_string()]);
    summary.add_row(row!["Description", &entry.description()]);
    summary.add_row(row!["File", &entry.path().to_string_lossy()]);
    print_table(summary, plain_table(), [1, 1]);

    Ok(0)
}
