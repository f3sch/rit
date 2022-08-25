use crate::*;
use anyhow::{bail, Context, Result};
use log::*;
use std::{
    env::{current_dir, var},
    fmt::Display,
    fs::File,
    io::{self, Write},
};

/// The `Commit` `Object` is another implementation of the established `Blob` and
/// `Tree` pattern.
pub struct Commit {
    /// The `Tree` id the `Commit` belongs to.
    oid: String,

    /// Type.
    type_: Types,

    /// The `Message` attached to this commit.
    message: Message,

    /// Data field
    data: Vec<u8>,
}

impl Commit {
    /// Create a new `Commit`.
    pub fn new(tree: String, message: Message) -> Self {
        trace!("Creating Commit");
        Self {
            oid: tree,
            type_: Types::Commit,
            message,
            data: Vec::new(),
        }
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "{:?}", self.data)?;
        writeln!(f)?;

        Ok(())
    }
}

impl Object for Commit {
    fn get_type(&self) -> Types {
        trace!("Getting type, should be commit");
        self.type_
    }

    fn get_data(&mut self) -> &Vec<u8> {
        trace!("Getting data of commit");
        let mut s = String::new();
        s.push_str(&format!("tree {}\n", self.oid));
        s.push_str(&format!("author {}\n", self.message.get_author()));
        s.push_str(&format!("committer {}\n", self.message.get_author()));
        s.push('\n');
        s.push_str(&format!("{}\n", self.message.get_message()));
        self.data.extend(s.as_bytes().to_vec());

        &self.data
    }

    fn set_oid(&mut self, hash: String) {
        trace!("Setting oid of blob");
        self.oid = hash;
    }

    fn get_oid(&self) -> Option<String> {
        trace!("Getting oid of blob");
        Some(self.oid.clone())
    }
}

/// Create the directory structure of a repository.
pub fn make_commit(commit: cli::Commit) -> Result<()> {
    trace!("Committing to repository");
    debug!("Got arguments: {:?}", commit);

    // get current directory
    let root_path =
        current_dir().with_context(|| "Commit: Could not get the current working directory!")?;
    let git_path = root_path.join(".git");
    let db_path = git_path.join("objects");
    debug!("root_path is {:?}", root_path);
    debug!("git_path is {:?}", git_path);
    debug!("db_path is {:?}", db_path);

    // check if this is actually a repository.
    if !is_repo(&root_path)? {
        bail!("Commit: {:?} is not a rit repository!", root_path);
    }

    // Get the current workspace.
    let workspace =
        Workspace::new(&root_path).with_context(|| "Commit: Could not load workspace!")?;
    let database = Database::new(&db_path).with_context(|| "Commit: Could not load database")?;

    // collect entries for the tree
    let mut entries: Vec<Entry> = Vec::new();

    for path in workspace.get_list_files() {
        debug!("Committing {:?} to database.", path);
        let data = workspace
            .read_file(path)
            .with_context(|| "Commit: Could not read file in workspace")?;
        let blob = &mut Blob::new(data);

        // store blob and set its oid
        database
            .store(blob)
            .with_context(|| "Commit: Failed storing blob")?;

        let entry = Entry::new(path.to_path_buf(), blob.get_oid().unwrap());
        entries.push(entry);
    }

    let tree = &mut Tree::new(entries);
    database
        .store(tree)
        .with_context(|| "Commit: Database failed to store the new tree")?;

    // Get commit message
    let message = Message::from_commit(&commit)
        .with_context(|| "Commit: Failed to construct commit message")?;
    debug!("{}", message);
    let commit = &mut Commit::new(
        tree.get_oid()
            .with_context(|| "Commit: Tree should have oid set")?,
        message,
    );
    database
        .store(commit)
        .with_context(|| "Commit: Failed to store commit")?;

    // store HEAD
    let mut f = File::options()
        .write(true)
        .create(true)
        .open(git_path.join("HEAD"))
        .with_context(|| "Commit: Cannot create HEAD")?;
    f.write(
        commit
            .get_oid()
            .with_context(|| "Commit: Is stored, should have oid set")?
            .as_bytes(),
    )
    .with_context(|| "Commit: Writing HEAD unsuccessful")?;

    info!("Commit: OK");
    Ok(())
}

/// Return the author of this `Commit`.
/// Looked at in the following order:
///     1.  author flag
///     2.  TODO local config
///     3.  environment variable `GIT_AUTHOR_NAME`
///     4.  TODO global config
pub fn get_author(commit: &cli::Commit) -> Result<String> {
    trace!("Getting author's name");
    // check commit command
    if commit.author.is_some() {
        debug!("Got from command");
        return Ok(commit.author.as_ref().unwrap().clone());
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
pub fn get_email(commit: &cli::Commit) -> Result<String> {
    trace!("Getting author's email");
    // check commit command
    if commit.email.is_some() {
        debug!("Got from command");
        return Ok(commit.email.as_ref().unwrap().clone());
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
pub fn get_message(commit: &cli::Commit) -> Result<String> {
    trace!("Getting commit message");

    // from command
    if commit.message.is_some() {
        debug!("Got from command");
        return Ok(commit.message.as_ref().unwrap().clone());
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
