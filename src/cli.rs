/// Description of the CLI interface to floki
use std::path;
use structopt::StructOpt;

/// Subcommands of the main floki command
#[derive(Debug, StructOpt)]
pub(crate) enum Subcommand {
    /// Run a command within the container
    #[structopt(name = "run")]
    Run { command: Vec<String> },

    /// Pull the image in the configuration file
    #[structopt(name = "pull")]
    Pull {},

    /// Generate shell cocmpletions to stdout.
    #[structopt(name = "completion")]
    Completion {
        /// The shell to generate completions for.  Choose from: bash, fish, zsh, powershell, elvish
        #[structopt(name = "SHELL", parse(try_from_str))]
        shell: structopt::clap::Shell,
    },

    /// enter attach container
    #[structopt(name = "attach")]
    Attach,
}

/// Main CLI interface
#[derive(Debug, StructOpt)]
#[structopt(name = "floki", about = "The interactive container launcher.")]
pub(crate) struct Cli {
    /// Use the specified config instead of searching the tree for a
    /// "floki.yaml" file.
    #[structopt(long = "config", short = "c")]
    pub(crate) config_file: Option<path::PathBuf>,

    /// Deprecated, and no longer has any effect.
    #[structopt(long = "local", short = "l", hidden = true)]
    pub(crate) local: bool,

    /// Logging verbosity level
    #[structopt(short = "v", parse(from_occurrences))]
    pub(crate) verbosity: u8,

    #[structopt(subcommand)]
    pub(crate) subcommand: Option<Subcommand>,
}
