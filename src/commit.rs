use crate::{cli::Commit, is_repo, Workspace};
use anyhow::{bail, Context, Result};
use log::*;
use std::env::current_dir;

/// Create the directory structure of a repository.
pub fn make_commit(commit: Commit) -> Result<()> {
    trace!("Committing to repository");
    debug!("Got arguments: {:?}", commit);

    // get current directory
    let root_path =
        current_dir().with_context(|| format!("Could not get the current working directory!"))?;
    let git_path = root_path.join(".git");
    let db_path = git_path.join("objects");
    debug!("root_path is {:?}", root_path);
    debug!("git_path is {:?}", git_path);
    debug!("db_path is {:?}", db_path);

    // check if this is actually a repository.
    if !is_repo(&root_path)? {
        bail!("{:?} is not a rit repository!", root_path);
    }

    // Get the current workspace.
    let workspace = Workspace::new(root_path)?;
    let list_files = workspace.get_list_files();
    debug!("File list {:?}", list_files);

    Ok(())
}
