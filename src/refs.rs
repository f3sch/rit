use anyhow::{bail, Context, Result};
use log::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::Lockfile;

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
        let mut lockfile = Lockfile::new(self.head_path())
            .with_context(|| "Refs: Failed to create lockfile while updating HEAD")?;

        if lockfile
            .hold_for_update()
            .with_context(|| "Refs: Lockfile creation went wrong")?
        {
            bail!("Could not acquire lock on file: {:?}", self.head_path());
        }

        lockfile
            .write(oid)
            .with_context(|| "Refs: lockfile write failed while updating HEAD")?;
        lockfile
            .write("\n".to_string())
            .with_context(|| "Refs: lockfile write failed while updating HEAD")?;
        lockfile
            .commit()
            .with_context(|| "Refs: Could not write changes to HEAD")?;

        Ok(())
    }

    /// Read the HEAD file if it exists and return the content.
    pub fn read_head(&self) -> Result<Option<String>> {
        trace!("Reading HEAD");

        // does file even exist
        if !self.head_path().exists() {
            debug!("HEAD does not exists (commit maybe parentless)");
            return Ok(None);
        }
        debug!("HEAD already exists, reading it now");

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
        debug!("HEAD contains: {s}");

        Ok(Some(s))
    }

    /// Get the path to HEAD.
    fn head_path(&self) -> PathBuf {
        self.pathname.join("HEAD")
    }
}
