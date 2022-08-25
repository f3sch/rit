/// Custom types that can be stored in the `Database`.
#[derive(Clone, Copy)]
pub enum Types {
    Blob,
    Tree,
    Commit,
}

impl Types {
    pub fn as_string(&self) -> String {
        match self {
            Self::Blob => String::from("blob"),
            Self::Tree => String::from("tree"),
            Self::Commit => String::from("commit"),
        }
    }

    pub fn as_type(s: String) -> Types {
        if s == "blob" {
            return Types::Blob;
        }
        if s == "tree" {
            return Types::Tree;
        }
        if s == "commit" {
            return Types::Commit;
        }

        panic!("Type {} not recognized, corrupt db?", s);
    }
}

pub trait Object {
    /// Return the type of the `Object`.
    fn get_type(&self) -> Types;

    /// Turn the data into its bytes.
    fn get_data(&mut self) -> &Vec<u8>;

    /// Set the `Object` id.
    fn set_oid(&mut self, hash: String);

    /// Get the `Object` id.
    fn get_oid(&self) -> Option<String>;
}
