use clap::Command;

pub fn command() -> Command {
    Command::new("stop").about("Stop the application")
}

pub fn run(_args: &clap::ArgMatches) -> Result<i32, anyhow::Error> {
    Ok(0)
}
