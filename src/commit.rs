use crate::*;
use anyhow::{bail, Context, Result};
use log::*;
use std::env::current_dir;

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

    // get the current workspace.
    let workspace =
        Workspace::new(&root_path).with_context(|| "Commit: Could not load workspace!")?;
    let database = Database::new(&db_path).with_context(|| "Commit: Could not load database")?;
    let refs = Refs::new(git_path).with_context(|| "Commit: Could not load refs")?;

    // get commit message
    let message = Message::from_commit(&commit)
        .with_context(|| "Commit: Failed to construct commit message")?;
    debug!("{}", message);

    // collect entries for the tree
    let mut entries: Vec<Entry> = Vec::new();

    // iterate through all files in the workspace
    // files with identical oid are skipped, this prevents unnecessary writes
    // and makes diffing efficient
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

        let stat = workspace
            .stat_file(path)
            .with_context(|| "Commit: Could not get filestat")?;
        let entry = Entry::new(path.to_path_buf(), blob.get_oid().unwrap(), stat);
        entries.push(entry);
    }

    // store root tree
    let root = &mut Tree::build(entries).with_context(|| "Commit: Could not build root tree")?;
    // root.traverse(&|tree: &mut Tree| {
    //     database
    //         .store(tree)
    //         .with_context(|| "Commit: Database failed to store the new tree")
    //         .expect("Commit: Traversal of root tree failed");
    // });

    // get parent commit
    let parent = refs
        .read_head()
        .with_context(|| "Commit: Could not get parent")?;

    // generate commit
    let commit = &mut database::Commit::new(
        parent,
        root.get_oid()
            .with_context(|| "Commit: Tree should have oid set")?,
        message,
    );

    // store commit
    database
        .store(commit)
        .with_context(|| "Commit: Failed to store commit")?;

    // update ref to new HEAD
    refs.update_head(
        commit
            .get_oid()
            .with_context(|| "Commit: Is stored, should have oid set")?,
    )
    .with_context(|| "Commit: Updating HEAD unsuccessful")?;

    info!("Commit: OK");
    Ok(())
}
