use crate::*;
use ring::digest::Digest;

/// A `Blob` contains a file's contents.
/// It will be stored by the `Database`.
pub struct Blob {
    /// Data contained in the `Blob`.
    data: Vec<u8>,

    /// Type: `Blob`.
    type_: Types,

    /// Unique oid of `Blob`.
    oid: Option<Digest>,
}

impl Blob {
    /// Create a new `Blob` based on the bytes in a file.
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            type_: Types::Blob,
            oid: None,
        }
    }

    /// Return a reference to the data of the blob.
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Return the type of
    pub fn get_type(&self) -> Types {
        self.type_
    }

    pub fn set_oid(&mut self, digest: Digest) {
        self.oid = Some(digest);
    }
}
