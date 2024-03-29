use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

static NAME: &str = env!("CARGO_BIN_NAME");

static AFTER_HELP: &str = "Use -h for short descriptions and --help for more details";

#[derive(Parser, Debug)]
#[command(
    name = NAME,
    author,
    bin_name = NAME,
    version,
    about,
    after_help = AFTER_HELP,
    arg_required_else_help = true,
    disable_help_flag = true,
    disable_version_flag = true,
    help_template = "{bin} {version}\n{author-with-newline}{about-section}\n{all-args}{after-help}",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// Override the configuration file path
    #[arg(long = "config", value_name = "PATH", global = true)]
    pub config_path: Option<PathBuf>,
    /// Suppress all output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,
    /// Use verbose output (specify multiple for more)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
    /// Show help information
    #[arg(short, long, action = clap::ArgAction::Help, global = true)]
    pub help: Option<bool>,
    /// Show version information
    #[arg(short = 'V', long)]
    pub version: bool,
}

impl Cli {
    pub fn get_version_string(&self) -> String {
        let mut value = vec![format!(
            "{NAME} {} ({} {})",
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_GIT_SHA_SHORT"),
            env!("VERGEN_GIT_COMMIT_DATE"),
        )];

        if self.verbose > 0 {
            value.push(format!("commit-hash: {}", env!("CARGO_PKG_GIT_SHA")));
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
        }

        value.join("\n")
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Run first time setup
    Init(InitArgs),
    /// Fetch missing modules
    Fetch,
    /// Apply configuration modules and profiles
    Apply(ApplyArgs),
    /// Show information about various items
    Info,
    /// Remove unnecessary files
    Clean,
    /// Reset configuration state or application data
    Reset(ResetArgs),
    /// Show current configuration status
    Status,
    /// Show version information
    #[command(hide = true)]
    Version,
}

#[derive(Debug, Args, Clone)]
pub struct InitArgs {
    /// Clone setup information from a git repository
    repo: Option<String>,
}

#[derive(Debug, Args, Clone)]
pub struct ApplyArgs {
    /// Apply the list of modules in the order they are given
    module: Vec<String>,
    /// Apply the given profile
    #[arg(short, long, conflicts_with = "module")]
    profile: Option<String>,
}

#[derive(Debug, Args, Clone)]
pub struct ResetArgs {
    /// Completely remove all data (useful before uninstalling)
    #[arg(long)]
    full: bool,
}
