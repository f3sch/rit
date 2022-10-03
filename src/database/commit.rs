use crate::*;
use log::*;
use std::fmt::Display;

/// The `Commit` `Object` is another implementation of the established `Blob` and
/// `Tree` pattern.
pub struct Commit {
    /// The `Tree` id the `Commit` belongs to.
    oid: String,

    /// Type.
    type_: Types,

    /// Possible parent of `Commit`.
    parent: Option<String>,

    /// The `Message` attached to this commit.
    message: Message,

    /// Data field
    data: Vec<u8>,
}

impl Commit {
    /// Create a new `Commit`.
    pub fn new(parent: Option<String>, tree: String, message: Message) -> Self {
        trace!("Creating Commit");
        Self {
            oid: tree,
            type_: Types::Commit,
            parent,
            message,
            data: Vec::new(),
        }
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "{:?}", self.data)?;
        writeln!(f)?;

        Ok(())
    }
}

impl Object for Commit {
    fn get_type(&self) -> Types {
        trace!("Getting type, should be commit");
        self.type_
    }

    fn get_data(&mut self) -> &Vec<u8> {
        trace!("Getting data of commit");
        let mut s = String::new();
        s.push_str(&format!("tree {}\n", self.oid));
        if let Some(parent) = &self.parent {
            debug!("Parent of commit is: {parent}");
            s.push_str(&format!("parent {}\n", parent));
        }
        s.push_str(&format!("author {}\n", self.message.get_author()));
        s.push_str(&format!("committer {}\n", self.message.get_author()));
        s.push('\n');
        s.push_str(&format!("{}\n", self.message.get_message()));
        self.data.extend(s.as_bytes().to_vec());

        &self.data
    }

    fn set_oid(&mut self, hash: String) {
        trace!("Setting oid of blob");
        self.oid = hash;
    }

    fn get_oid(&self) -> Option<String> {
        trace!("Getting oid of blob");
        Some(self.oid.clone())
    }
}
