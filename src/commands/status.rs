use crate::event::find_last_event;
use crate::styling::{plain_table, print_table};
use crate::utils::format_date;
use clap::Command;
use prettytable::row;

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

    let mut t = last.to_table(status);
    t.insert_row(
        3,
        row![
            "Now",
            format_date(&chrono::Local::now().naive_local(), "full")
        ],
    );

    print_table(t, plain_table(), [1, 1]);

    Ok(0)
}
