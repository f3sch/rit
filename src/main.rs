use anyhow::{bail, Context, Result};
use clap::Parser;
use log::*;
use rit::*;

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .format_timestamp(None)
        .filter_level(args.verbose.log_level_filter())
        .init();
    info!("Started logging, commencing to execute subcommand!");
    debug!("Got {:?}", args);

    match args.command {
        cli::Commands::Init(init) => {
            create_repo(init).with_context(|| "Main: create_repo unsuccessful")?;
        }
        cli::Commands::Commit(commit) => {
            make_commit(commit).with_context(|| "Main: make_commit unsuccessful")?;
        }
        cli::Commands::CatFile(cat_file) => {
            print_object(cat_file).with_context(|| "Main: print_object unsuccessful")?;
        }
        cli::Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
            bail!("Main: No extra arguments are allowed!");
        }
    }
    Ok(())
}
