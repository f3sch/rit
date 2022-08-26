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
            let strip = entry
                .path()
                .strip_prefix(path)
                .expect("Utility: Could not strip prefix")
                .to_path_buf();
            vec.push(strip); // add to `vec`
        });

    // Warning for empty repository.
    if vec.is_empty() {
        warn!("No files in repository!");
    }
    debug!("List of files: {:?}", vec);

    vec
}

/// Check if the entry is ignored.
fn is_ignored(entry: &DirEntry) -> bool {
    // ignore directories
    if entry.file_type().is_dir() {
        return true;
    }

    // ignore symlinks
    if entry.path_is_symlink() {
        return true;
    }

    let path = entry.path();
    for spath in path.iter() {
        if spath.to_str() == Some(".git") {
            return true;
        }
    }

    let name = entry
        .file_name()
        .to_str()
        .expect("Could not convert entry name to str!");
    debug!("Not ignored Filename is {}", name);

    false
}

/// Check if `path` is a rit repository.
pub fn is_repo(root_path: &PathBuf) -> Result<bool> {
    trace!("Checking if repository.");
    let git_path = root_path.join(".git");
    let db_path = git_path.join("objects");

    // Does root even exists?
    if !root_path.exists() {
        bail!("Utility: '{:?}' does not exists!", root_path);
    }
    // check if this is actually a repository.
    if !git_path.exists() {
        debug!("'{:?}' is not a rit repository!", git_path);
        return Ok(false);
    }
    // is the database present
    if !db_path.exists() {
        debug!("'{:?}' is missing!!", db_path);
        return Ok(false);
    }

    debug!("{:?} is a repository.", root_path);
    Ok(true)
}
