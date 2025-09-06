use clap::{Arg, Command};

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    Ok(0)
}

pub fn subcommand() -> clap::Command {
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
                .required(true)
                .short('d')
                .long("description"),
        )
}
