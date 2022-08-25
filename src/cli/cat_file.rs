use clap::Args;
use std::path::PathBuf;

/// Print some pretty information of a rit object.
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct CatFile {
    /// Path to the rit object.
    #[clap(value_parser)]
    pub path: PathBuf,
}
