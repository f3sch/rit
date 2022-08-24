use anyhow::Result;
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
            create_repo(init)?;
        }
        cli::Commands::Commit(commit) => {
            make_commit(commit)?;
        }
        cli::Commands::CatFile(cat_file) => {
            print_object(cat_file)?;
        }
        cli::Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }
    Ok(())
}
