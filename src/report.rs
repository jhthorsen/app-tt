use clap::{Arg, Command};

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    Ok(0)
}

pub fn subcommand() -> Command {
    Command::new("report")
        .about("Show time spent")
        .arg(
            Arg::new("project")
                .help("Project name")
                .short('p')
                .long("project"),
        )
        .arg(
            Arg::new("tag")
                .help("Tag(s) to filter by")
                .short('t')
                .long("tag"),
        )
        .arg(Arg::new("since").help("From what time").long("since"))
        .arg(Arg::new("until").help("Until what time").long("until"))
}
