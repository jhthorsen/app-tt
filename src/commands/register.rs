use clap::{Arg, Command};

pub fn command() -> Command {
    Command::new("register")
        .about("Register a missed entry")
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

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    Ok(0)
}
