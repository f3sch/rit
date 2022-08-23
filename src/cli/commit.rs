use clap::Args;

/// Init command.
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Commit {}
