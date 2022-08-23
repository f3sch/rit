use anyhow::{bail, Result};
use log::*;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

/// Given a `path` return all the files recursively except `.`, `..`, `.git` and
/// those in `.gitignore`.
pub fn get_files(path: &PathBuf) -> Vec<PathBuf> {
    trace!("Getting all files in repository, path: {:?}", path);
    let mut vec = Vec::new();

    // Recursively descend `path` adding all files to `vec`.
    WalkDir::new(path)
        .into_iter()
        .filter_map(|v| v.ok()) // is file ok
        .filter(|e| !is_ignored(e)) // is ignored
        .for_each(|entry| {
            vec.push(entry.path().to_path_buf()); // add to vec
        });

    for entry in WalkDir::new(path) {
        println!("{:?}", entry.unwrap().path());
    }

    // Warning for empty repository.
    if vec.len() == 0 {
        warn!("No files in repository!");
    }

    vec
}

/// Check if the entry is ignored.
fn is_ignored(entry: &DirEntry) -> bool {
    // ignore directories
    if entry.file_type().is_dir() {
        return true;
    }

    let name = entry
        .file_name()
        .to_str()
        .expect("Could not convert entry name to str!");

    name == ".git"
}

/// Check if `path` is a rit repository.
pub fn is_repo(root_path: &PathBuf) -> Result<bool> {
    trace!("Checking if repository.");
    let git_path = root_path.join(".git");
    let db_path = git_path.join("objects");

    // Does root even exists?
    if !root_path.exists() {
        bail!("'{:?}' does not exists!", root_path);
    }
    // check if this is actually a repository.
    if !git_path.exists() {
        bail!("'{:?}' is not a rit repository!", git_path);
    }
    // is the database present
    if !db_path.exists() {
        bail!("'{:?}' is missing!!", db_path);
    }

    debug!("{:?} is a repository.", root_path);
    Ok(true)
}
