use std::fmt::Display;

use chrono::{
    offset::{Offset, TimeZone},
    Local,
};
use log::{debug, trace};

/// The `Author` object packages up the name, the email and the time values
/// that form contents of the author and committer headers in a `Commit`.
#[derive(Clone, Debug)]
pub struct Author {
    /// Author's name.
    pub name: String,

    /// Author's email.
    pub email: String,

    /// Time format string.
    pub time: String,
}

impl Author {
    /// Create a new `Author`.
    /// This automatically calculates the timezone offset.
    pub fn new(name: String, email: String, time: String) -> Self {
        trace!("Creating a new author");
        // Calculate the timezone offset
        let offset = Local.timestamp(0, 0).offset().fix().local_minus_utc();
        let sign: String = if offset < 0 {
            String::from("-")
        } else {
            String::from("+")
        };
        let time = format!("{} {}{}", time, sign, offset);
        debug!("Author.time: {}", time);

        Self { name, email, time }
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}> {}", self.name, self.email, self.time)
    }
}
