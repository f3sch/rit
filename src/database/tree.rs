use crate::*;
use log::*;

/// It will be stored by the `Database`.
pub struct Tree {
    /// Type: `Tree`.
    type_: Types,

    /// Unique oid of `Tree`.
    oid: Option<String>,

    /// Entries vector.
    entries: Vec<Entry>,

    /// Data of `Tree`.
    data: Vec<u8>,
}

impl Tree {
    /// Create a new `Tree` from an `Entry` vector.
    pub fn new(entries: Vec<Entry>) -> Self {
        Self {
            type_: Types::Tree,
            oid: None,
            entries,
            data: Vec::new(),
        }
    }
}

impl Object for Tree {
    fn get_type(&self) -> Types {
        trace!("Getting type, should be tree");
        self.type_
    }

    fn get_data(&mut self) -> &Vec<u8> {
        trace!("Getting data of tree");
        // sort entries
        self.entries.sort();
        // get weird packing
        self.data = self
            .entries
            .iter()
            .flat_map(|e| {
                let mut pre = format!("{} {}", e.get_mode(), e.get_name())
                    .as_bytes()
                    .to_vec();
                pre.push(b"\x00"[0]);
                let oid = hex::decode(e.get_oid()).expect("Tree: Could not decode hex");
                pre.extend(oid);
                pre
            })
            .collect();

        debug!("Data size is {}", self.data.len());
        &self.data
    }

    fn set_oid(&mut self, hash: String) {
        trace!("Setting oid of tree");
        self.oid = Some(hash);
    }

    fn get_oid(&self) -> Option<String> {
        trace!("Getting oid of tree");
        self.oid.clone()
    }
}
