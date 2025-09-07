mod commands;
mod event;
mod styling;
mod utils;

use clap::{Arg, ArgAction, Command};

fn quiet_arg() -> Arg {
    Arg::new("quiet")
        .help("As little output as possible")
        .long("quiet")
        .num_args(0)
        .action(ArgAction::SetTrue)
}

fn main() {
    let matches = Command::new("tt")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Time tracking app")
        .author("Jan Henning Thorsen <jan.henning@thorsenlabs.com>")
        .subcommand(commands::start::command())
        .subcommand(commands::stop::command())
        .subcommand(commands::status::command())
        .subcommand(commands::report::command())
        .subcommand(commands::register::command())
        .subcommand(commands::edit::command())
        .get_matches();

    let exit_code = match matches.subcommand() {
        Some(("start", args)) => commands::start::run(args),
        Some(("stop", args)) => commands::stop::run(args),
        Some(("report", args)) => commands::report::run(args),
        Some(("register", args)) => commands::register::run(args),
        Some(("edit", args)) => commands::edit::run(args),
        _ => commands::status::run(&matches),
    };

    match exit_code {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
