use crate::cli::CatFile;
use crate::Types;
use anyhow::{bail, Context, Result};
use flate2::read::ZlibDecoder;
use log::*;
use ring::digest::SHA1_OUTPUT_LEN;
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
        .with_context(|| "CatFile: Zlib decoding issues")?;
    debug!("Object: '{}'", s);

    match Types::as_type(String::from(&s[0..4])) {
        Types::Blob => {
            let null = s
                .find(|c: char| c == '\x00')
                .with_context(|| "CatFile: Blob does not contain null-byte")?;
            // Print blob
            println!("Type: Blob");
            println!("Size: {}", &s[5..null]);
            println!("Content:\n'''");
            println!("{}", &s[null..]);
            println!("'''");
        }
        Types::Tree => {
            let size_space = s
                .find(|c: char| c == ' ')
                .with_context(|| "CatFile: No first space separating `tree` and `len`")?;
            let size_null = s
                .find(|c: char| c == '\x00')
                .with_context(|| "CatFile: No first null-byte marking end of `len`")?;
            let mut prev = size_null;
            // Print tree
            println!("Type: Tree");
            println!("Size: {}", &s[size_space..size_null]);
            println!("Content:\n'''");
            let _ = &s[..]
                .match_indices(|c: char| c == '\x00')
                .for_each(|(i, _)| {
                    if size_null != i {
                        println!("{}", i);
                        println!("Mode and Name: {}", &s[prev..i]);
                        println!("Hash: {}", &s[(i + 1)..(i + SHA1_OUTPUT_LEN)]);
                        prev = i;
                    }
                });
            println!("'''");
        }
    }

    Ok(())
}
