use std::path::PathBuf;

use clap::{Parser, Subcommand};

static NAME: &str = env!("CARGO_BIN_NAME");

static AFTER_HELP: &str = "Use -h for short descriptions and --help for more details";

#[derive(Parser, Debug)]
#[command(
    name = NAME,
    author,
    bin_name = NAME,
    about,
    after_help = AFTER_HELP,
    disable_help_flag = true,
    disable_version_flag = true,
    help_template = "{bin} {version}\n{author-with-newline}{about-section}\n{all-args}{after-help}",
)]
pub struct Cli {
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
    /// Print help information
    #[arg(short, long, action = clap::ArgAction::Help, help_heading = "Global Options", global = true)]
    pub help: Option<bool>,
    /// Print version information
    #[arg(short = 'V', long, help_heading = "Global Options")]
    pub version: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn get_version_string(verbose: bool) -> String {
        let mut value = vec![format!(
            "{NAME} {} ({} {})",
            env!("CARGO_PKG_VERSION"),
            env!("CONFMAN_GIT_SHA_SHORT"),
            env!("VERGEN_GIT_COMMIT_DATE"),
        )];

        if verbose {
            value.push(format!("commit-hash: {}", env!("CONFMAN_GIT_SHA")));
            value.push(format!("commit-date: {}", env!("VERGEN_GIT_COMMIT_DATE")));
            value.push(format!(
                "build-target: {}",
                env!("VERGEN_CARGO_TARGET_TRIPLE"),
            ));
            let profile = if env!("VERGEN_CARGO_DEBUG") == "true" {
                "debug"
            } else {
                "release"
            };
            value.push(format!(
                "build-type: {profile} (opt={})",
                env!("VERGEN_CARGO_OPT_LEVEL")
            ));
            value.push(format!("build-date: {}", env!("VERGEN_BUILD_DATE")));
        }

        value.join("\n")
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    Init,
}
