// System libraries
use std::ffi::OsString;

// Crates
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

// CLI subcommands
pub mod init;
pub use init::Init;
pub mod commit;
pub use commit::Commit;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[clap(name = "rit")]
#[clap(about = "Inferior rust git (rit)", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(flatten)]
    pub verbose: Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(Init),
    Commit(Commit),

    #[clap(external_subcommand)]
    External(Vec<OsString>),
}
