use crate::{entries::find_tracked_entries, utils::format_hour_minutes};
use chrono::Datelike;
use clap::Command;

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let now = chrono::Local::now().date_naive();
    let first_of_month = now.with_day(1).expect("Invalid day");

    let tracked_entries = find_tracked_entries(&first_of_month, &now);
    let Some(last) = tracked_entries.last() else {
        eprintln!("Unable to find the last tracked entry.");
        return Ok(1);
    };

    if last.stop.is_none() {
        println!(
            "Tracking {} for {} since {}.",
            last.project,
            last.stop.unwrap().format("%Y-%m-%d %H:%M"),
            format_hour_minutes(&last.duration()),
        );
    } else {
        println!(
            "Stopped tracking {} at {} after {}.",
            last.project,
            last.stop.unwrap().format("%Y-%m-%d %H:%M"),
            format_hour_minutes(&last.duration()),
        );
    }

    Ok(0)
}

pub fn subcommand() -> Command {
    Command::new("status").about("Show the current time tracking status")
}
