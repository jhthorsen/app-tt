use crate::entries::find_last_tracked_entry;
use crate::styling::{plain_table, print_table};
use clap::Command;

pub fn command() -> Command {
    Command::new("status").about("Show the current time tracking status")
}

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let entry = find_last_tracked_entry()?;
    let status = if entry.stop.is_some() {
        "Stopped"
    } else {
        "Tracking"
    };

    print_table(entry.to_table(status), plain_table(), [1, 1]);

    Ok(0)
}
