use crate::{cli::Commit, *};
use anyhow::Context;
use anyhow::Result;
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

///  This represents a commit message.
pub struct Message {
    /// Author
    author: Author,

    /// Commit message.
    message: String,
}

impl Message {
    /// Get author's name.
    pub fn get_name(&self) -> String {
        self.author.name.clone()
    }

    /// Get author's email.
    pub fn get_email(&self) -> String {
        self.author.email.clone()
    }

    /// Get commit Message.
    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    /// Get author.
    pub fn get_author(&self) -> Author {
        self.author.clone()
    }

    /// Construct a `Message` from a `Commit`.
    pub fn from_commit(commit: &Commit) -> Result<Self> {
        let name = commit
            .get_author()
            .with_context(|| "Message: Could not get author's name")?;
        let email = commit
            .get_email()
            .with_context(|| "Message: Could not get author's email")?;
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .with_context(|| "Message: Could not get the time")?
            .as_secs()
            .to_string();
        Ok(Self {
            author: Author::new(name, email, time),
            message: commit
                .get_message()
                .with_context(|| "Message: Could not get commit message")?,
        })
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Commit Message:")?;
        writeln!(f, "{}", self.author)?;
        writeln!(f, "---")?;
        writeln!(f, "{}", self.message)?;
        writeln!(f, "---")?;

        Ok(())
    }
}
