use crate::cli::Init;
use anyhow::{bail, Context, Result};
use log::*;
use std::{
    fs::{canonicalize, create_dir_all},
    path::PathBuf,
};

/// Create the directory structure of a repository.
pub fn create_repo(init: Init) -> Result<()> {
    trace!("Creating directory structure");
    // Get the path.
    // If none is provided use `.` as default.
    let path = match init.path {
        Some(path) => path,
        None => PathBuf::from("."),
    };
    info!("Path is {:?}", path);

    // make path absolute
    let root_path = canonicalize(path).with_context(|| format!("Could not cannonicalize path!"))?;
    let git_path = root_path.join(".git");
    debug!("root_path is {:?}", root_path);
    debug!("git_path is {:?}", git_path);

    // check if this already is a repository.
    if git_path.exists() {
        bail!("'{:?}' is already a rit repository!", git_path);
    }

    // create basic structure
    for dir in ["objects", "refs"] {
        create_dir_all(git_path.join(dir))?;
        debug!("Created directory '{}' under git_path", dir);
    }

    info!("Initialized empty rit repository in {:?}", git_path);
    Ok(())
}
