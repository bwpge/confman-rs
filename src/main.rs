mod cli;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();
    dbg!(args);
}
