use clap::Args;
use std::path::PathBuf;

/// Init command.
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Init {
    /// Path to initialize rit repository.
    #[clap(value_parser)]
    pub path: Option<PathBuf>,
}
