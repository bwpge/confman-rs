use anyhow::Result;
use confman::Config;

use crate::cli::Cli;

pub fn exec(_config: &Config, args: &Cli) -> Result<()> {
    println!("{}", args.get_version_string());
    Ok(())
}
