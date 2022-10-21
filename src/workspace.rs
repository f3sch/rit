use crate::get_files;
use anyhow::{bail, Context, Result};
use is_executable::IsExecutable;
use log::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// The workspace is responsible for the files in the working tree.
/// All the files that can be directly edited, rather than those stored in `.git`.
pub struct Workspace {
    /// Root of the repository.
    root_path: PathBuf,

    /// List of all files in repository, which are not `.`, `..`, `.git` and not
    /// in .gitignore.
    list_files: Vec<PathBuf>,
}

impl Workspace {
    /// Create a new `Workspace` at `root_path`.
    pub fn new(root_path: &PathBuf) -> Result<Self> {
        trace!("Creating the workspace.");
        let list_files = get_files(root_path);

        Ok(Self {
            root_path: root_path.to_owned(),
            list_files,
        })
    }

    /// Return the list of files in the repository.
    /// An empty vector represents an empty repository.
    pub fn get_list_files(&self) -> &Vec<PathBuf> {
        trace!("Returning list of files.");
        &self.list_files
    }

    /// Return the root of the repository.
    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_path
    }

    /// Read the content of a file as bytes.
    pub fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>> {
        trace!("Reading contents of file {:?}", path);
        let mut f = File::open(path).with_context(|| "Workspace: Could not open file")?;
        let mut buffer = Vec::new();

        // read the whole content of the file
        f.read_to_end(&mut buffer)
            .with_context(|| "Workspace: Could not open file")?;
        Ok(buffer)
    }

    /// Stat a given file path.
    pub fn stat_file(&self, path: &PathBuf) -> Result<FileStat> {
        trace!("Getting file stat for {:?}", path);

        // For safety check if file still exists
        if !path.exists() {
            bail!("Workspace: File does not exists!");
        }

        if path.is_dir() {
            // check this first since dirs are always executable
            return Ok(FileStat::Dir);
        } else if path.is_file() && path.is_executable() {
            return Ok(FileStat::Executable);
        } else if path.is_file() {
            return Ok(FileStat::File);
        }

        bail!("Workspace: Unknown fs type encounterd!");
    }
}

/// Enum to define custom file stat modes.
/// There are really only three we care about.
pub enum FileStat {
    Dir,
    File,
    Executable,
}
