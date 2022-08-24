use clap::Args;
use std::path::PathBuf;

/// cat-file command.
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct CatFile {
    /// Path to cat-file.
    #[clap(value_parser)]
    pub path: PathBuf,
}
