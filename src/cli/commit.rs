use clap::Args;

/// Commit file from staging area to ...
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Commit {
    /// Set the author of this commit.
    #[clap(short, long, value_parser)]
    pub author: Option<String>,

    /// Set the author's email of this commit.
    #[clap(short, long, value_parser)]
    pub email: Option<String>,

    /// Set the commit message.
    #[clap(short, long, value_parser)]
    pub message: Option<String>,
}
