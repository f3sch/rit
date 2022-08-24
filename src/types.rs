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

pub trait Object {
    /// Return the type of the `Object`.
    fn get_type(&self) -> Types;

    /// Turn the data into its bytes.
    fn get_data(&self) -> &Vec<u8>;

    /// Set the `Object` id.
    fn set_oid(&mut self, hash: String);
}
