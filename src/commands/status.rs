use crate::event::find_last_event;
use crate::styling::{plain_table, print_table};
use clap::Command;

pub fn command() -> Command {
    Command::new("status").about("Show the current time tracking status (default)")
}

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let last = find_last_event()?;
    let status = if last.stop.is_some() {
        "Stopped"
    } else {
        "Tracking"
    };

    print_table(last.to_table(status), plain_table(), [1, 1]);

    Ok(0)
}
