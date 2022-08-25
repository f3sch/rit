use crate::*;
use log::*;

/// A `Blob` contains a file's contents.
/// It will be stored by the `Database`.
pub struct Blob {
    /// Type: `Blob`.
    type_: Types,

    /// Unique oid of `Blob`.
    oid: Option<String>,

    /// Data contained in the `Blob`.
    data: Vec<u8>,
}

impl Blob {
    /// Create a new `Blob` based on the bytes in a file.
    pub fn new(data: Vec<u8>) -> Self {
        trace!("Creating blob");

        Self {
            type_: Types::Blob,
            oid: None,
            data,
        }
    }
}

impl Object for Blob {
    fn get_type(&self) -> Types {
        trace!("Getting type, should be blob");
        self.type_
    }

    fn get_data(&mut self) -> &Vec<u8> {
        trace!("Getting data of blob");
        &self.data
    }

    fn set_oid(&mut self, hash: String) {
        trace!("Setting oid of blob");
        self.oid = Some(hash);
    }

    fn get_oid(&self) -> Option<String> {
        trace!("Getting oid of blob");
        self.oid.clone()
    }
}
