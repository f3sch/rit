use anyhow::{bail, Context, Result};
use log::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// `Refs` manages all files under ~.git/refs~.
/// It also can update HEAD.
pub struct Refs {
    /// Points to ~.git~.
    pathname: PathBuf,
}

impl Refs {
    /// Create a new `Refs` object, which manages the references in ~.git/refs~.
    pub fn new(pathname: PathBuf) -> Result<Self> {
        trace!("Creating new `Refs`");
        if !pathname.exists() {
            bail!("Refs: .git does not exists!");
        }

        Ok(Self { pathname })
    }

    /// Update HEAD to point to a new `Tree`.
    pub fn update_head(&self, oid: String) -> Result<()> {
        trace!("Updating HEAD to {}", oid);
        let mut f = File::options()
            .write(true)
            .read(false)
            .create(true)
            .open(self.head_path())
            .with_context(|| "Commit: Cannot create HEAD")?;
        f.write(oid.as_bytes())
            .with_context(|| "Commit: Writing HEAD unsuccessful")?;

        Ok(())
    }

    /// Read the HEAD file if it exists and return the content.
    pub fn read_head(&self) -> Result<Option<String>> {
        // does file even exist
        if !self.head_path().exists() {
            return Ok(None);
        }

        // read it
        let mut buffer = Vec::new();
        let mut f = File::options()
            .read(true)
            .write(false)
            .open(self.head_path())
            .with_context(|| "Refs: Could not open HEAD")?;
        f.read_to_end(&mut buffer)
            .with_context(|| "Refs: Unable to read HEAD")?;
        let s =
            String::from_utf8(buffer).with_context(|| "Refs: Encoding bytes into utf-8 failed")?;

        Ok(Some(s))
    }

    /// Get the path to HEAD.
    fn head_path(&self) -> PathBuf {
        self.pathname.join("HEAD")
    }
}
