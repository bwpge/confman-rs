mod cli;

use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;

fn main() -> ExitCode {
    let args = Cli::parse();

    if args.version {
        let version = Cli::get_version_string(args.verbose > 0);
        println!("{version}");
        return ExitCode::SUCCESS;
    }

    dbg!(args);
    ExitCode::SUCCESS
}
