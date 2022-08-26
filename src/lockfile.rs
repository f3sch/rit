use anyhow::{bail, Context, Result};
use log::*;
use std::fs::{rename, File};
use std::io::Write;
use std::path::*;

/// A `Lockfile` should prevent simultaneous access to a file.
pub struct Lockfile {
    /// The file that is locked.
    file_path: PathBuf,

    /// The file-lock.
    lock_path: PathBuf,

    /// Is `file_path` locked.
    lock: Option<File>,
}

impl Lockfile {
    /// Create a new `Lockfile`.
    pub fn new(path: PathBuf) -> Result<Self> {
        trace!("Creating new Lockfile");
        let file_path = path;
        let mut lock_path = file_path.to_owned();
        lock_path.set_extension("lock");
        Ok(Self {
            file_path,
            lock_path,
            lock: None,
        })
    }

    /// Try acquiring the lock.
    /// false: we must not wait
    /// true: we must wait
    pub fn hold_for_update(&mut self) -> Result<bool> {
        trace!("Trying to acquire lock");
        // we hold the lock
        if self.lock.is_some() {
            debug!("Lock is held by us");
            return Ok(false);
        }

        // is it locked by somebody else
        if self.lock_path.exists() {
            debug!("Lock is held by else");
            return Ok(true);
        }

        // create it
        self.lock = Some(
            File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(&self.lock_path)
                .with_context(|| "Lockfile: Failed to create lockfile")?,
        );
        debug!("Lock created");

        Ok(false)
    }

    /// Write the data to the lockfile.
    pub fn write(&mut self, s: String) -> Result<()> {
        trace!("Trying to write to lockfile");
        self.raise_on_stale_lock()
            .with_context(|| "Lockfile: While trying write did not have lock")?;

        self.lock
            .as_ref()
            .unwrap()
            .write(s.as_bytes())
            .with_context(|| "Lockfile: Could not write to lockfile")?;
        debug!("Write successful");

        Ok(())
    }

    /// Commit changes to the file.
    pub fn commit(&mut self) -> Result<()> {
        trace!("Committing changes for lockfile");
        self.raise_on_stale_lock()
            .with_context(|| "Lockfile: While trying to commit did not have lock")?;

        drop(self.lock.take());
        rename(&self.lock_path, &self.file_path)
            .with_context(|| "Lockfile: Could not rename lockfile")?;
        debug!("Commit successful");

        Ok(())
    }

    /// Raise an error if we do not have the lock
    fn raise_on_stale_lock(&self) -> Result<()> {
        trace!("Checking if lockfile is stale");
        // is there a lock?
        if self.lock.is_none() {
            bail!("Not holding lock on file {:?}", self.lock_path);
        }
        debug!("Lockfile not stale");

        Ok(())
    }
}
