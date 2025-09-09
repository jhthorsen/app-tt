use crate::event::{TimeEvent, find_events};
use crate::styling::{DASH, plain_table, print_table, regular_table};
use crate::utils::{format_date, format_duration, to_naive_date_time};
use chrono::Datelike;
use clap::{Arg, Command};
use prettytable::{Cell, Row, Table, row};

pub fn command() -> Command {
    let now = chrono::Local::now();
    let first_of_month = now.with_day(1).expect("Invalid day");

    Command::new("report")
        .about("Show time spent")
        .arg(
            Arg::new("project")
                .help("Event project name")
                .short('p')
                .long("project"),
        )
        .arg(
            Arg::new("tag")
                .help("Event tag(s) to filter by")
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
                .default_value(format_date(&now.naive_local(), "hm")),
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

fn is_same_date(a: &chrono::NaiveDateTime, b: &chrono::NaiveDateTime) -> bool {
    a.year() == b.year() && a.month() == b.month() && a.day() == b.day()
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let since = to_naive_date_time(args.get_one::<String>("since"), None)?;
    let until = to_naive_date_time(args.get_one::<String>("until"), None)?;

    let arg_group = args.get_one::<String>("group").expect("Default missing");
    let mut grouped_event = TimeEvent {
        ..TimeEvent::default()
    };

    let mut report = Table::new();
    report.set_titles(row!["Date", "Project", "Start", "Stop", "Duration", "Tags"]);

    let mut total_events = 0;
    let mut total_duration = chrono::Duration::zero();

    let events = find_events(&since.date(), &until.date());
    let mut events = events.iter().filter(|e| e.matches_args(args)).peekable();
    while let Some(event) = events.next() {
        let duration = event.duration();
        total_duration += duration;
        total_events += 1;

        let same_event_tomorrow = if let Some(next_event) = events.peek() {
            is_same_date(&event.start, &next_event.start) && event.project == next_event.project
        } else {
            false
        };

        // Group events by day
        let (event, duration) = if arg_group == "day" {
            if grouped_event.total_duration.is_none() {
                grouped_event = TimeEvent {
                    description: event.description.clone(),
                    total_duration: Some(duration),
                    project: event.project.clone(),
                    start: event.start,
                    stop: event.stop,
                    tags: event.tags.clone(),
                };
            } else if is_same_date(&grouped_event.start, &event.start)
                && grouped_event.project == event.project
            {
                grouped_event.tags.extend(event.tags.clone());
                grouped_event.total_duration =
                    Some(grouped_event.total_duration.unwrap() + duration);
            }

            if same_event_tomorrow {
                continue;
            }

            (&grouped_event, grouped_event.total_duration.unwrap())
        } else {
            (event, duration)
        };

        let stop = if let Some(d) = event.stop {
            format_date(&d, "hm")
        } else {
            DASH.to_string()
        };

        report.add_row(Row::new(vec![
            Cell::new(&format_date(&event.start, "ymd")),
            Cell::new(&event.project),
            Cell::new(&format_date(&event.start, "hm")),
            Cell::new(&stop),
            Cell::new(&format_duration(&duration)).style_spec("r"),
            Cell::new(&event.tags_as_string()),
        ]));

        if !same_event_tomorrow {
            grouped_event.total_duration = None;
        }
    }

    let mut summary = Table::new();
    summary.add_row(row!["Total events:", total_events.to_string()]);
    summary.add_row(row!["Total time:", &format_duration(&total_duration)]);

    print_table(report, regular_table(), [1, 1]);
    print_table(summary, plain_table(), [0, 1]);

    Ok(0)
}
