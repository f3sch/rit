use crate::{cli::Commit, *};
use anyhow::{bail, Context, Result};
use log::*;
use std::env::{current_dir, var};

/// Create the directory structure of a repository.
pub fn make_commit(commit: Commit) -> Result<()> {
    trace!("Committing to repository");
    debug!("Got arguments: {:?}", commit);

    // get current directory
    let root_path =
        current_dir().with_context(|| "Commit: Could not get the current working directory!")?;
    let git_path = root_path.join(".git");
    let db_path = git_path.join("objects");
    debug!("root_path is {:?}", root_path);
    debug!("git_path is {:?}", git_path);
    debug!("db_path is {:?}", db_path);

    // check if this is actually a repository.
    if !is_repo(&root_path)? {
        bail!("Commit: {:?} is not a rit repository!", root_path);
    }

    // Get the current workspace.
    let workspace =
        Workspace::new(&root_path).with_context(|| "Commit: Could not load workspace!")?;
    let database = Database::new(&db_path).with_context(|| "Commit: Could not load database")?;

    // Get commit message input
    let author_name =
        get_author(&commit).with_context(|| "Commit: Failed to load author's name")?;
    let author_email =
        get_email(&commit).with_context(|| "Commit: Failed to load author's email")?;
    debug!("Author's name is {}", author_name);
    debug!("Author's email is {}", author_email);

    // collect entries for the tree
    let mut entries: Vec<Entry> = Vec::new();

    for path in workspace.get_list_files() {
        debug!("Committing {:?} to database.", path);
        let data = workspace
            .read_file(path)
            .with_context(|| "Commit: Could not read file in workspace")?;
        let blob = &mut Blob::new(data);

        // store blob and set its oid
        database
            .store(blob)
            .with_context(|| "Commit: Failed storing blob")?;

        let entry = Entry::new(path.to_path_buf(), blob.get_oid().unwrap());
        entries.push(entry);
    }

    let tree = &mut Tree::new(entries);
    database
        .store(tree)
        .with_context(|| "Commit: Database failed to store the new tree")?;

    Ok(())
}

/// Return the author of this `Commit`.
/// Looked at in the following order:
///     1.  author flag
///     2.  TODO local config
///     3.  environment variable `GIT_AUTHOR_NAME`
///     4.  TODO global config
fn get_author(commit: &Commit) -> Result<String> {
    trace!("Getting author's name");
    // check commit command
    if commit.author.is_some() {
        return Ok(commit.author.as_ref().unwrap().clone());
    }

    // check env var
    if let Ok(name) = var(GIT_AUTHOR_NAME) {
        return Ok(name);
    }

    // bail, there is no recovering here
    bail!("Commit: No author found to attribute commit to!");
}

/// Return the author's email of this `Commit`.
/// Looked at in the following order:
///     1.  email flag
///     2.  TODO local config
///     3.  environment variable `GIT_AUTHOR_EMAIL`
///     4.  TODO global config
fn get_email(commit: &Commit) -> Result<String> {
    trace!("Getting author's email");
    // check commit command
    if commit.email.is_some() {
        return Ok(commit.email.as_ref().unwrap().clone());
    }

    // check env var
    if let Ok(email) = var(GIT_AUTHOR_EMAIL) {
        return Ok(email);
    }

    // bail, there is no recovering here
    bail!("Commit: No email found to attribute commit to!");
}
