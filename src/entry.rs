use std::path::PathBuf;

/// `Entry` serves to package up information that `Tree` needs to about its
/// contents:
///             1. the filename
///             2. object id
///             3. file mode (hardcoded for now)
pub struct Entry {
    /// Name is the path to the entry
    name: PathBuf,

    /// Object id (hash)
    oid: String,
}

impl Entry {
    /// Create a new `Entry`.
    pub fn new(name: PathBuf, oid: String) -> Self {
        Self { name, oid }
    }

    /// Get a reference to the name of the `Entry`.
    pub fn get_name(&self) -> &PathBuf {
        &self.name
    }

    /// Get the oid of the `Entry`.
    pub fn get_oid(&self) -> String {
        self.oid.clone()
    }
}
