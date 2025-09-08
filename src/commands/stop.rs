use crate::event::find_last_event;
use crate::styling::{plain_table, print_table};
use crate::utils::{default_project, min_duration, to_naive_date_time};
use clap::{Arg, Command};

pub fn command() -> Command {
    Command::new("stop")
        .about("Stop tracking time")
        .arg(
            Arg::new("stop_time")
                .help("The stop time for tracking (e.g., '08:00')")
                .default_value("now")
                .index(1),
        )
        .arg(
            Arg::new("project")
                .help("Add a tag, unless same project as last event")
                .long("tag-unless-same-project")
                .num_args(0..=1)
                .default_missing_value(default_project()),
        )
        .arg(crate::quiet_arg())
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let mut last = find_last_event()?;
    let mut status = "Stopped";

    if last.stop.is_none() {
        if let Some(project) = args.get_one::<String>("project")
            && last.project != *project
        {
            status = "Added tag";
            last.add_tags(vec![project]);
            last.save()?;
        } else {
            last.stop = Some(to_naive_date_time(
                args.get_one::<String>("stop_time"),
                None,
            )?);

            if last.duration().num_seconds() < min_duration()? {
                status = "Discarded";
                last.delete()?;
                last.description = "Event duration is lower than TT_MIN_DURATION".to_string();
            } else {
                status = "Saved";
                last.save()?;
            }
        }
    }

    if !args.get_flag("quiet") {
        print_table(last.to_table(status), plain_table(), [1, 1]);
    }

    Ok(0)
}
