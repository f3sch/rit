use std::fmt::Display;

use crate::{cli::Commit, *};
use anyhow::Context;
use anyhow::Result;

///  This represents a commit message.
pub struct Message {
    /// Author's name.
    name: String,

    /// Author's email.
    email: String,

    /// Commit message.
    message: String,
}

impl Message {
    /// Get author's name.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get author's email.
    pub fn get_email(&self) -> String {
        self.email.clone()
    }

    /// Get commit Message.
    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    /// Construct a message from a
    pub fn from_commit(commit: &Commit) -> Result<Self> {
        Ok(Self {
            name: get_author(commit).with_context(|| "Message: Could not get author's name")?,
            email: get_email(commit).with_context(|| "Message: Could not get author's email")?,
            message: get_message(commit)
                .with_context(|| "Message: Could not get commit message")?,
        })
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Commit Message:")?;
        writeln!(f, "Author: {} - Email: {}", self.name, self.email)?;
        writeln!(f, "---")?;
        writeln!(f, "{}", self.message)?;
        writeln!(f, "---")?;

        Ok(())
    }
}
