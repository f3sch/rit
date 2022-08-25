use std::cmp::Ord;
use std::path::PathBuf;

/// `Entry` serves to package up information that `Tree` needs to about its
/// contents:
///             1. the filename
///             2. object id
///             3. file mode (hardcoded for now)
#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub struct Entry {
    /// Name is the path to the entry
    name: String,

    /// Object id (hash)
    oid: String,
}

impl Entry {
    /// Create a new `Entry`.
    pub fn new(name: PathBuf, oid: String) -> Self {
        Self {
            name: name
                .into_os_string()
                .into_string()
                .expect("Entry: Could not convert path to string"),
            oid,
        }
    }

    /// Get the name of the `Entry`.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get the oid of the `Entry`.
    pub fn get_oid(&self) -> String {
        self.oid.clone()
    }
}
