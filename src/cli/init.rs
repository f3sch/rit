use clap::Args;
use std::path::PathBuf;

/// Initialize a rit repository.
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Init {
    /// Path where to initialize a rit repository.
    #[clap(value_parser)]
    pub path: Option<PathBuf>,
}
