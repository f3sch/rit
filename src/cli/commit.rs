use crate::*;
use anyhow::{bail, Context, Result};
use clap::Args;
use log::*;
use std::{env::var, io};

/// Commit file from staging area to ...
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Commit {
    /// Set the author of this commit.
    #[clap(short, long, value_parser)]
    pub author: Option<String>,

    /// Set the author's email of this commit.
    #[clap(short, long, value_parser)]
    pub email: Option<String>,

    /// Set the commit message.
    #[clap(short, long, value_parser)]
    pub message: Option<String>,
}

impl Commit {
    /// Return the author of this `Commit`.
    /// Looked at in the following order:
    ///     1.  author flag
    ///     2.  TODO local config
    ///     3.  environment variable `GIT_AUTHOR_NAME`
    ///     4.  TODO global config
    pub fn get_author(&self) -> Result<String> {
        trace!("Getting author's name");
        // check commit command
        if self.author.is_some() {
            debug!("Got from command");
            return Ok(self.author.as_ref().unwrap().clone());
        }

        // check env var
        if let Ok(name) = var(GIT_AUTHOR_NAME) {
            debug!("Got from env");
            return Ok(name);
        }

        // bail, there is no recovering here
        bail!("Commit: No author found to attribute commit to!");
    }

    /// Return the author's email of this `Commit`.
    /// Looked at in the following order:
    ///     1.  email flag
    ///     2.  TODO local config
    ///     3.  environment variable `GIT_AUTHOR_EMAIL`
    ///     4.  TODO global config
    pub fn get_email(&self) -> Result<String> {
        trace!("Getting author's email");
        // check commit command
        if self.email.is_some() {
            debug!("Got from command");
            return Ok(self.email.as_ref().unwrap().clone());
        }

        // check env var
        if let Ok(email) = var(GIT_AUTHOR_EMAIL) {
            debug!("Got from env");
            return Ok(email);
        }

        // bail, there is no recovering here
        bail!("Commit: No email found to attribute commit to!");
    }

    /// Return the message of the commit, either by:
    ///     1. message flag
    ///     2. TODO via opening `GIT_MESSAGE`
    ///     3. reading from stdin
    pub fn get_message(&self) -> Result<String> {
        trace!("Getting commit message");

        // from command
        if self.message.is_some() {
            debug!("Got from command");
            return Ok(self.message.as_ref().unwrap().clone());
        }

        // read from stdin
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.lines().for_each(|line| {
            buffer.push_str(
                &line
                    .with_context(|| "Commit: Failed to read from stdin")
                    .unwrap(),
            );
            buffer.push('\n');
        });
        if !buffer.is_empty() {
            debug!("Got from stdin");
            return Ok(buffer);
        }

        bail!("Commit: No commit message!");
    }
}
