/// Custom types that can be stored in the `Database`.
#[derive(Clone, Copy)]
pub enum Types {
    Blob,
}

impl Types {
    pub fn as_string(&self) -> String {
        match self {
            Self::Blob => String::from("blob"),
        }
    }
}
