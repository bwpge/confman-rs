mod cli;
mod commands;

use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;

fn main() -> ExitCode {
    let args = Cli::parse();
    dbg!(&args);

    match crate::commands::exec(&args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            // TODO: pretty print error
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
