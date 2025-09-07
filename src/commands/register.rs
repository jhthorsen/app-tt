use crate::entries::TrackedEntry;
use crate::styling::{plain_table, print_table};
use crate::utils::{format_date, to_naive_date_time};
use clap::{Arg, Command};
use prettytable::{Table, row};

pub fn command() -> Command {
    let example_time = chrono::Local::now().naive_local().format("%Y-%m-%dT%H:%M");

    Command::new("register")
        .about("Register a missed entry")
        .arg(
            Arg::new("start_time")
                .help(format!(
                    "The start time for tracking (e.g., '{}')",
                    example_time,
                ))
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("stop_time")
                .help(format!(
                    "The stop time for tracking (e.g., '{}')",
                    example_time
                ))
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("project")
                .help("Project name")
                .required(true)
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
    let start = to_naive_date_time(args.get_one::<String>("start_time"), None)?;
    let stop = to_naive_date_time(args.get_one::<String>("stop_time"), Some(&start))?;

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
            .cloned()
            .unwrap_or_default(),
        start,
        stop: Some(stop),
        tags,
        total_duration: None,
    };

    entry.save()?;

    let mut summary = Table::new();
    summary.add_row(row!["Status", "Saved"]);
    summary.add_row(row!["Project", &entry.project]);
    summary.add_row(row!["Start", format_date(&entry.start, "full")]);
    summary.add_row(row!["Tags", &entry.tags_as_string()]);
    summary.add_row(row!["Description", &entry.description()]);
    summary.add_row(row!["File", &entry.path().to_string_lossy()]);
    print_table(summary, plain_table(), [1, 1]);

    Ok(0)
}
