use anyhow::{bail, Result};
use log::*;
use std::path::PathBuf;

/// The `Database` is responsible for managing files in `.git/objects`.
/// It will receive `Blobs` and store them.
pub struct Database {
    /// `db_path` is the path to the database.
    db_path: PathBuf,
}

impl Database {
    /// Create/Load a `Database` from `db_path`.
    pub fn new(db_path: &PathBuf) -> Result<Self> {
        trace!("Loading database");

        // is this really the database we want?
        if !db_path.exists() {
            bail!("Database does not exist or is malformed!");
        }

        Ok(Self {
            db_path: db_path.to_owned(),
        })
    }

    /// Get `Database` path.
    pub fn get_db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
