use crate::entries::{TrackedEntry, find_tracked_entries};
use crate::utils::{format_hour_minutes, is_same_date, to_naive_date};
use chrono::Datelike;
use clap::{Arg, Command};
use prettytable::{Cell, Row, Table, row};

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let since = to_naive_date(args.get_one::<String>("since"));
    let until = to_naive_date(args.get_one::<String>("until"));
    let tracked_entries = find_tracked_entries(&since, &until);

    let arg_group = args.get_one::<String>("group").expect("Default missing");
    let mut grouped_entry = TrackedEntry {
        ..TrackedEntry::default()
    };

    let mut report = Table::new();
    report.set_format(crate::styling::table_styling());
    report.set_titles(row!["Date", "Project", "Start", "Stop", "Duration", "Tags"]);

    let mut total_entries = 0;
    let mut total_duration = chrono::Duration::zero();

    let mut tracked_entries = tracked_entries
        .iter()
        .filter(|e| e.matches_args(args))
        .peekable();

    while let Some(entry) = tracked_entries.next() {
        let duration = entry.duration();
        total_duration += duration;
        total_entries += 1;

        let same_date_tomorrow = if let Some(next_entry) = tracked_entries.peek() {
            is_same_date(&entry.start, &next_entry.start)
        } else {
            false
        };

        // Group entries by day
        let (entry, duration) = if arg_group == "day" {
            if grouped_entry.total_duration.is_none() {
                grouped_entry = TrackedEntry {
                    total_duration: Some(duration),
                    project: entry.project.clone(),
                    start: entry.start,
                    stop: entry.stop,
                    tags: entry.tags.clone(),
                };
            } else if is_same_date(&grouped_entry.start, &entry.start) {
                grouped_entry.tags.extend(entry.tags.clone());
                grouped_entry.total_duration =
                    Some(grouped_entry.total_duration.unwrap() + duration);
            }

            if same_date_tomorrow {
                continue;
            }

            (&grouped_entry, grouped_entry.total_duration.unwrap())
        } else {
            (entry, duration)
        };

        report.add_row(Row::new(vec![
            Cell::new(&entry.start.format("%Y-%m-%d").to_string()),
            Cell::new(&entry.project),
            Cell::new(&entry.start.format("%H:%M").to_string()),
            Cell::new(&entry.stop().format("%H:%M").to_string()),
            Cell::new(&format_hour_minutes(&duration)).style_spec("r"),
            Cell::new(&entry.tags_as_string()),
        ]));

        if !same_date_tomorrow {
            grouped_entry.total_duration = None;
        }
    }

    println!();
    report.printstd();

    let mut summary = Table::new();
    summary.set_format(crate::styling::summary_styling());
    summary.add_row(row!["Total entries:", total_entries.to_string()]);
    summary.add_row(row!["Total time:", &format_hour_minutes(&total_duration)]);

    println!();
    summary.printstd();

    println!();
    Ok(0)
}

pub fn subcommand() -> Command {
    let now = chrono::Local::now();
    let first_of_month = now.with_day(1).expect("Invalid day");

    Command::new("report")
        .about("Show time spent")
        .arg(
            Arg::new("project")
                .help("Project name")
                .short('p')
                .long("project"),
        )
        .arg(
            Arg::new("tag")
                .help("Tag(s) to filter by")
                .short('t')
                .long("tag"),
        )
        .arg(
            Arg::new("since")
                .help("From what time")
                .long("since")
                .default_value(first_of_month.format("%Y-%m-%d").to_string()),
        )
        .arg(
            Arg::new("until")
                .help("Until what time")
                .long("until")
                .default_value("now"),
        )
        .arg(
            Arg::new("group")
                .help("Group by day")
                .num_args(0..=1)
                .short('g')
                .long("group")
                .default_value("none")
                .default_missing_value("day"),
        )
}
