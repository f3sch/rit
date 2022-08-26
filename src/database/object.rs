use crate::*;

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
