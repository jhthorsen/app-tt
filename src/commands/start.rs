use crate::event::{TimeEvent, find_last_event};
use crate::styling::{plain_table, print_table};
use crate::utils::{default_project, format_date, min_duration, to_naive_date_time};
use clap::{Arg, Command};

pub fn command() -> clap::Command {
    let now = chrono::Local::now().naive_local();

    Command::new("start")
        .about("Start tracking time")
        .arg(
            Arg::new("start_time")
                .help("The start time for tracking (e.g., '08:00')")
                .default_value(format_date(&now, "hm"))
                .index(1),
        )
        .arg(
            Arg::new("project")
                .help("Event project name")
                .short('p')
                .long("project"),
        )
        .arg(Arg::new("tag").help("Event tag(s)").short('t').long("tag"))
        .arg(
            Arg::new("description")
                .help("Event description")
                .short('d')
                .long("description"),
        )
        .arg(
            Arg::new("resume")
                .help("Resume if stopped")
                .long("resume")
                .num_args(0..=1)
                .default_missing_value("600")
                .value_parser(clap::value_parser!(i64)),
        )
        .arg(crate::quiet_arg())
}

fn not_too_old_to_resume(event: &TimeEvent, max_age: i64) -> bool {
    if let Some(stop) = event.stop {
        let now = chrono::Local::now().naive_local();
        let diff = now - stop;
        diff.num_seconds() <= max_age
    } else {
        true
    }
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let start = to_naive_date_time(args.get_one::<String>("start_time"), None)?;
    let mut last = find_last_event().unwrap_or_default();

    let mut status = "Started";
    let resume = args.get_one::<i64>("resume");
    let project = args
        .get_one::<String>("project")
        .unwrap_or(&default_project())
        .to_owned();

    let mut event = if let Some(max_age) = resume
        && last.project == project
        && last.stop.is_some()
        && not_too_old_to_resume(&last, *max_age)
    {
        status = "Resumed";
        last.stop = None;
        last
    } else if !last.project.is_empty() && last.stop.is_none() {
        status = "Tracking";
        last
    } else {
        // Stop current event if not already stopped
        if !last.project.is_empty() && last.stop.is_none() {
            last.stop = Some(start);
            if last.duration().num_seconds() < min_duration()? {
                last.delete()?;
            } else {
                last.save()?;
            }
        }

        TimeEvent {
            description: "".to_string(),
            project,
            start,
            stop: None,
            tags: vec![],
            total_duration: None,
        }
    };

    if let Some(description) = args.get_one::<String>("description") {
        event.description = description.clone();
    }

    if let Some(tag) = args.get_one::<String>("tag") {
        event.add_tags(tag.split(',').map(|s| s.trim().to_string()).collect());
    }

    event.save()?;

    if !args.get_flag("quiet") {
        print_table(event.to_table(status), plain_table(), [1, 1]);
    }

    Ok(0)
}
