use crate::entries::find_last_tracked_entry;
use crate::styling::{DASH, plain_table, print_table};
use crate::utils::{format_date, format_duration};
use clap::Command;
use prettytable::{Table, row};

pub fn command() -> Command {
    Command::new("status").about("Show the current time tracking status")
}

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let entry = find_last_tracked_entry()?;
    let mut summary = Table::new();
    let (status, stop) = if let Some(d) = entry.stop {
        ("Stopped", format_date(&d, "full"))
    } else {
        ("Tracking", DASH.to_string())
    };

    summary.add_row(row!["Status", status]);
    summary.add_row(row!["Project", &entry.project]);
    summary.add_row(row!["Start", format_date(&entry.start, "full")]);
    summary.add_row(row!["Stop", stop]);
    summary.add_row(row!["Duration", &format_duration(&entry.duration())]);
    summary.add_row(row!["Tags", &entry.tags_as_string()]);
    summary.add_row(row!["Description", &entry.description()]);
    print_table(summary, plain_table(), [1, 1]);

    Ok(0)
}
