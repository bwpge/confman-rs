use std::path::PathBuf;

use clap::{Parser, Subcommand};

static NAME: &str = env!("CARGO_BIN_NAME");

static VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

static AFTER_HELP: &str = "Use -h for short descriptions and --help for more details";

#[derive(Parser, Debug)]
#[command(
    name = NAME,
    author,
    bin_name = NAME,
    version = VERSION,
    about,
    after_help = AFTER_HELP,
    disable_help_flag = true,
    disable_version_flag = true,
)]
#[command(
    help_template = "{bin} {version}\n{author-with-newline}{about-section}\n{all-args}{after-help}"
)]
pub struct Cli {
    /// Print help information
    #[arg(short, long, action = clap::ArgAction::Help)]
    pub help: Option<bool>,
    /// Print version information
    #[arg(short = 'V', long, action = clap::ArgAction::Version)]
    pub version: Option<bool>,
    /// Override the configuration file path
    #[arg(
        long = "config",
        value_name = "PATH",
        help_heading = "Global Options",
        global = true
    )]
    pub config_path: Option<PathBuf>,
    /// Suppress all output
    #[arg(
        short,
        long,
        help_heading = "Global Options",
        global = true,
        conflicts_with = "verbose"
    )]
    pub quiet: bool,
    /// Use verbose output (specify multiple times for more)
    #[arg(short, long, help_heading = "Global Options", action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    Init,
}
