use crate::*;

/// A `Blob` contains a file's contents.
/// It will be stored by the `Database`.
pub struct Blob {
    /// Data contained in the `Blob`.
    data: Vec<u8>,

    /// Type: `Blob`.
    type_: Types,

    /// Unique oid of `Blob`.
    oid: Option<String>,
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
}

impl Object for Blob {
    fn get_type(&self) -> Types {
        self.type_
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn set_oid(&mut self, hash: String) {
        self.oid = Some(hash);
    }
}
