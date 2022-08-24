use crate::Blob;
use anyhow::{bail, Result};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use log::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ring::digest::{self, digest, Digest};
use std::fs::{rename, File};
use std::io::prelude::*;
use std::{fs::create_dir, path::PathBuf};

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

    /// Store a `Blob` in the `Database`.
    pub fn store(&self, blob: &mut Blob) -> Result<()> {
        trace!("Storing blob.");
        let mut content = format!(
            "{} {}\0",
            blob.get_type().as_string(),
            blob.get_data().len(),
        )
        .as_bytes()
        .to_vec();
        content.append(&mut blob.get_data().to_vec());
        debug!("Content is: {:?}", content);

        // calculate hash
        let hash = digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, &content);
        debug!("Blob calculated hash: {:02x?}", hash.as_ref());
        self.write_object(hash, content)?;

        Ok(())
    }

    /// Write object to `Database`.
    fn write_object(&self, hash: Digest, content: Vec<u8>) -> Result<()> {
        trace!("Writing blob to database.");
        // construct hash as valid utf-8 hex string
        let name = hex::encode(hash.as_ref());
        // construct object path
        let object_path = self.db_path.join(&name[0..2]).join(&name[2..]);
        let dirname = object_path.parent().unwrap();
        let temp_name = dirname.join(Self::generate_temp_name());
        debug!("object_path is {:?}", object_path);
        debug!("dirname is {:?}", dirname);
        debug!("temp_name is {:?}", temp_name);

        // if dirname does not exist create it
        if !dirname.exists() {
            create_dir(dirname)?;
        }

        {
            // open temp_file, ensuring it does not exists already
            let mut file = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(&temp_name)?;
            let compressed = ZlibEncoder::new(content, Compression::fast()).finish()?;
            file.write_all(&compressed)?;
            debug!("Written compressed content to temp_file.");
        } // file is closed here.

        rename(temp_name, object_path)?;

        Ok(())
    }

    /// Generate temporay name.
    fn generate_temp_name() -> String {
        let rand: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();
        format!("tmp_obj_{}", rand)
    }
}
