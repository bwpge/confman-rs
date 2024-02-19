mod apply;
mod clean;
mod fetch;
mod info;
mod init;
mod reset;
mod status;
mod version;

use anyhow::Result;
use confman::Config;

use crate::cli::Cli;

/// Dispatch a command to the appropriate handler.
pub fn exec(args: &Cli) -> Result<()> {
    let config = Config::load(args.config_path.as_ref())?;
    dbg!(&config);

    if args.version {
        return version::exec(&config, args);
    }

    use crate::cli::Commands::*;
    let f = match &args.command {
        Some(c) => match c {
            Init(_) => init::exec,
            Fetch => fetch::exec,
            Apply(_) => apply::exec,
            Info => info::exec,
            Clean => clean::exec,
            Reset(_) => reset::exec,
            Status => status::exec,
            Version => version::exec,
        },
        None => {
            return Ok(());
        }
    };

    f(&config, args)
}
