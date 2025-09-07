use crate::entries::find_last_tracked_entry;
use crate::styling::{plain_table, print_table};
use crate::utils::{format_date, format_duration, min_duration, to_naive_date_time};
use clap::{Arg, Command};
use prettytable::{Table, row};

pub fn command() -> Command {
    Command::new("stop").about("Stop the application").arg(
        Arg::new("stop_time")
            .help("The stop time for tracking (e.g., '08:00')")
            .default_value("now")
            .index(1),
    )
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let mut entry = find_last_tracked_entry()?;
    let mut summary = Table::new();

    let mut status = "Stopped";
    if entry.stop.is_none() {
        entry.stop = Some(to_naive_date_time(
            args.get_one::<String>("stop_time"),
            None,
        )?);

        if entry.duration().num_seconds() < min_duration()? {
            status = "Discarded";
            entry.delete()?;
            entry.description = "Event duration is lower than TT_MIN_DURATION".to_string();
        } else {
            status = "Saved";
            entry.save()?;
        }
    }

    summary.add_row(row!["Status", status]);
    summary.add_row(row!["Project", &entry.project]);
    summary.add_row(row!["Start", format_date(&entry.start, "full")]);
    summary.add_row(row!["Stop", format_date(&entry.stop.unwrap(), "full")]);
    summary.add_row(row!["Duration", &format_duration(&entry.duration())]);
    summary.add_row(row!["Tags", &entry.tags_as_string()]);
    summary.add_row(row!["Description", &entry.description()]);
    summary.add_row(row!["File", &entry.path().to_string_lossy()]);
    print_table(summary, plain_table(), [1, 1]);

    Ok(0)
}
