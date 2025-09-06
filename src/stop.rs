use clap::Command;

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    Ok(0)
}

pub fn subcommand() -> Command {
    Command::new("stop").about("Stop the application")
}
