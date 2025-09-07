use crate::entries::TrackedEntry;
use crate::styling::{plain_table, print_table};
use crate::utils::{default_project, format_date, to_naive_date_time};
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
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    // TODO: Stop active entry, if any

    let tags = if let Some(tag) = args.get_one::<String>("tag") {
        tag.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec![]
    };

    let entry = TrackedEntry {
        description: args
            .get_one::<String>("description")
            .cloned()
            .unwrap_or_default(),
        project: args
            .get_one::<String>("project")
            .unwrap_or(&default_project())
            .to_owned(),
        start: to_naive_date_time(args.get_one::<String>("start_time"), None)?,
        stop: None,
        tags,
        total_duration: None,
    };

    entry.save()?;

    let mut summary = Table::new();
    summary.add_row(row!["Status", "Started"]);
    summary.add_row(row!["Project", &entry.project]);
    summary.add_row(row!["Start", format_date(&entry.start, "full")]);
    summary.add_row(row!["Tags", &entry.tags_as_string()]);
    summary.add_row(row!["File", &entry.path().to_string_lossy()]);
    summary.add_row(row!["Description", &entry.description()]);
    print_table(summary, plain_table(), [1, 1]);

    Ok(0)
}
