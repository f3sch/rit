use crate::cli::CatFile;
use anyhow::{bail, Context, Result};
use flate2::read::ZlibDecoder;
use log::*;
use std::fs::File;
use std::io::prelude::*;

/// Print out the contents of a rit-object.
pub fn print_object(cat_file: CatFile) -> Result<()> {
    trace!("Printing rit-object {:?}", cat_file.path);
    if !cat_file.path.exists() {
        bail!("Git Object {:?} does not exits!", cat_file.path);
    }

    let mut buffer = Vec::new();
    let mut file = File::options()
        .read(true)
        .write(false)
        .open(cat_file.path)
        .with_context(|| "CatFile: Either file {:?} exists or cannot be created")?;
    file.read_to_end(&mut buffer).with_context(|| {
        "CatFile: File could not be read in total or buffer could not be resized"
    })?;
    debug!("Read object: {:?}", buffer);
    let mut d = ZlibDecoder::new(&buffer[..]);
    let mut s = String::new();
    d.read_to_string(&mut s)
        .with_context(|| "CatFile: Zlib decodding issues")?;
    info!("Object: '{}'", s);

    Ok(())
}
