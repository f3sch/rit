use crate::*;
use anyhow::{bail, Context, Result};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use log::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ring::digest::{self, digest};
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
            bail!("Database: {:?} does not exist or is malformed!", db_path);
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
    pub fn store(&self, object: &mut dyn Object) -> Result<()> {
        trace!("Storing Object.");
        let type_ = object.get_type().as_string();
        let data = object.get_data();
        let len = data.len();
        let mut content = format!("{} {}", type_, len).as_bytes().to_vec();
        content.push(b"\x00"[0]); // null terminate
        content.extend(data);
        debug!("Content is: {:?}", content);

        // calculate hash
        // construct hash as valid utf-8 hex string
        let hash = hex::encode(digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, &content));
        // set hash for blob
        object.set_oid(hash.clone());
        debug!("Object calculated hash: {}", hash);
        self.write_object(hash, content)
            .with_context(|| "Database: Could not store blob")?;

        Ok(())
    }

    /// Write object to `Database`.
    fn write_object(&self, hash: String, content: Vec<u8>) -> Result<()> {
        trace!("Writing blob to database");
        // construct object path
        let object_path = self.db_path.join(&hash[0..2]).join(&hash[2..]);
        let dirname = object_path.parent().unwrap();
        let temp_name = dirname.join(Self::generate_temp_name());
        debug!("object_path is {:?}", object_path);
        debug!("dirname is {:?}", dirname);
        debug!("temp_name is {:?}", temp_name);

        // if dirname does not exist create it
        if !dirname.exists() {
            create_dir(dirname).with_context(|| "Database: Failed to create directory")?;
        }

        {
            // open temp_file, ensuring it does not exists already
            let mut file = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(&temp_name)
                .with_context(|| "Database: Could not create file")?;
            // create encoder
            let mut compressed = ZlibEncoder::new(Vec::new(), Compression::fast());
            // compress content
            compressed
                .write_all(&content)
                .with_context(|| "Database: Unable to encode content")?;
            // finish compression
            let compressed = compressed
                .finish()
                .with_context(|| "Database: Could not finish encoding")?;
            // write data to file
            file.write_all(&compressed)
                .with_context(|| "Database: Could not write compressed data to file")?;
            debug!("Written compressed content to temp_file.");
        } // file is closed here.

        debug!("Rename temp_file to object_path");
        rename(temp_name, object_path)
            .with_context(|| "Database: Renaming of temp_name to object_path failed")?;

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
