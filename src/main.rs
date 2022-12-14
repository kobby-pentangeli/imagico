mod args;
mod error;
mod png;
mod utils;

use args::{command, App, Command};
use clap::Parser;
use error::ProgramResult;

fn run(cmd: Command) -> ProgramResult<()> {
    match cmd {
        Command::Encode(args) => command::encode(args),
        Command::Decode(args) => command::decode(args),
        Command::Remove(args) => command::remove(args),
        Command::Print(args) => command::print_chunks(args),
    }
}

fn main() -> ProgramResult<()> {
    let app = App::parse();
    if let Some(name) = app.name.as_deref() {
        println!("{}", name);
    }
    if let Some(cmd) = app.command {
        run(cmd)?;
    }
    Ok(())
}
