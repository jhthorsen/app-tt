use crate::event::TimeEvent;
use crate::styling::{plain_table, print_table};
use crate::utils::to_naive_date_time;
use clap::{Arg, Command};

pub fn command() -> Command {
    let example_time = chrono::Local::now().naive_local().format("%Y-%m-%dT%H:%M");

    Command::new("register")
        .about("Register an event with start and stop time")
        .arg(
            Arg::new("start_time")
                .help(format!(
                    "The start time for tracking (e.g., '{}')",
                    example_time,
                ))
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("stop_time")
                .help(format!(
                    "The stop time for tracking (e.g., '{}')",
                    example_time
                ))
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("project")
                .help("Event project name")
                .required(true)
                .short('p')
                .long("project"),
        )
        .arg(
            Arg::new("tag")
                .help("Event tag(s)")
                .short('t')
                .long("tag"),
        )
        .arg(
            Arg::new("description")
                .help("Event description")
                .short('d')
                .long("description"),
        )
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let start = to_naive_date_time(args.get_one::<String>("start_time"), None)?;
    let stop = to_naive_date_time(args.get_one::<String>("stop_time"), Some(&start))?;

    let tags = if let Some(tag) = args.get_one::<String>("tag") {
        tag.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec![]
    };

    let event = TimeEvent {
        description: args
            .get_one::<String>("description")
            .cloned()
            .unwrap_or_default(),
        project: args
            .get_one::<String>("project")
            .cloned()
            .unwrap_or_default(),
        start,
        stop: Some(stop),
        tags,
        total_duration: None,
    };

    event.save()?;
    print_table(event.to_table("Saved"), plain_table(), [1, 1]);

    Ok(0)
}
