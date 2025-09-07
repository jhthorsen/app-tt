use crate::entries::find_last_tracked_entry;
use crate::styling::{plain_table, print_table};
use crate::utils::{min_duration, to_naive_date_time};
use clap::{Arg, Command};

pub fn command() -> Command {
    Command::new("stop")
        .about("Stop the application")
        .arg(
            Arg::new("stop_time")
                .help("The stop time for tracking (e.g., '08:00')")
                .default_value("now")
                .index(1),
        )
        .arg(crate::quiet_arg())
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let mut entry = find_last_tracked_entry()?;

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

    if !args.get_flag("quiet") {
        print_table(entry.to_table(status), plain_table(), [1, 1]);
    }

    Ok(0)
}
