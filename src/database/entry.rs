use std::cmp::Ord;
use std::path::PathBuf;

use crate::FileStat;

const REGULAR_MODE: &str = "100644";
const EXECUTABLE_MODE: &str = "100744";

/// `Entry` serves to package up information that `Tree` needs to about its
/// contents:
///             1. the filename
///             2. object id
///             3. file mode
#[derive(PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct Entry {
    /// Name is the path to the entry
    name: String,

    /// Object id (hash)
    oid: String,

    /// File mode
    stat: &'static str,
}

impl Entry {
    /// Create a new `Entry`.
    pub fn new(name: PathBuf, oid: String, stat: FileStat) -> Self {
        Self {
            name: name
                .into_os_string()
                .into_string()
                .expect("Entry: Could not convert path to string"),
            oid,
            stat: match stat {
                FileStat::Dir => EXECUTABLE_MODE,
                FileStat::File => REGULAR_MODE,
                FileStat::Executable => EXECUTABLE_MODE,
            },
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

    /// Get File mode
    pub fn get_mode(&self) -> String {
        self.stat.to_owned()
    }
}
