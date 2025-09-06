mod register;
mod report;
mod start;
mod status;
mod stop;

use clap::Command;

fn main() {
    let matches = Command::new("tt")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Time tracking app")
        .author("Jan Henning Thorsen <jan.henning@thorsenlabs.com>")
        .subcommand(start::subcommand())
        .subcommand(stop::subcommand())
        .subcommand(status::subcommand())
        .subcommand(report::subcommand())
        .subcommand(register::subcommand())
        .get_matches();

    let exit_code = match matches.subcommand() {
        Some(("start", args)) => start::run(args),
        Some(("stop", args)) => stop::run(args),
        Some(("report", args)) => report::run(args),
        Some(("register", args)) => register::run(args),
        _ => status::run(&matches),
    };

    match exit_code {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
