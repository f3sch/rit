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
