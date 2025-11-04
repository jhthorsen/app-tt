use crate::event::{TimeEvent, find_events, find_last_event};
use crate::utils::to_naive_date_time;
use clap::{Arg, ArgAction, Command};
use std::io::Write;
use tempfile::NamedTempFile;

pub fn command() -> Command {
    let last = find_last_event().unwrap_or_default();
    let example_since = last.start.format("%Y-%m-%dT00:00:00");
    let example_until = last.start.format("%Y-%m-%dT23:59:59");

    Command::new("edit")
        .about("Edit event(s)")
        .arg(
            Arg::new("since")
                .help("From what start time for event(s) to edit")
                .long("since")
                .default_value(example_since.to_string()),
        )
        .arg(
            Arg::new("until")
                .help("Until what start time for event(s) to edit")
                .long("until")
                .default_value(example_until.to_string()),
        )
        .arg(
            Arg::new("dry_run")
                .help("Only show what would be done")
                .long("dry-run")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
}

pub fn run(args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    let since = to_naive_date_time(args.get_one::<String>("since"), None)?;
    let until = to_naive_date_time(args.get_one::<String>("until"), None)?;
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    for event in find_events(&since.date(), &until.date()) {
        if event.start < since || event.start > until {
            continue;
        }

        if args.get_flag("dry_run") {
            println!("{} {}", editor, event.path().to_string_lossy());
            continue;
        }

        let mut editor_argv = editor.split_whitespace().collect::<Vec<&str>>();
        if editor_argv.is_empty() {
            continue;
        }

        let mut tmp = NamedTempFile::new()?;
        write!(tmp, "{}", event.serialize(true)?)?;
        println!("$ {} \"{}\"", editor, tmp.path().to_string_lossy());

        let _ = std::process::Command::new(editor_argv.remove(0))
            .args(editor_argv)
            .arg(tmp.path())
            .status();

        let content = std::fs::read_to_string(tmp.path())?;
        if TimeEvent::from_string(&content).is_ok() {
            println!("$ mv {:?} {:?}", tmp.path(), event.path());
            tmp.persist(event.path())?;
        } else {
            eprintln!("Unable to move {:?} to {:?}", tmp.path(), event.path());
        }
    }

    Ok(0)
}
