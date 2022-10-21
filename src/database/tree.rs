use crate::*;
use anyhow::{Context, Result};
use log::*;
use std::collections::BTreeMap;
use std::path::Path;

const TREE_MODE: &str = "004000";

/// This is a generic `Tree` type.
/// It can either be a `Entry` or a `Tree` itself.
#[derive(Clone)]
pub enum TreeEntry {
    Entry(Entry),
    Tree(Tree),
}

impl TreeEntry {
    pub fn get_mode(&self) -> String {
        match self {
            TreeEntry::Entry(e) => e.get_mode(),
            _ => TREE_MODE.to_owned(),
        }
    }

    pub fn get_oid(&self) -> String {
        match self {
            TreeEntry::Entry(e) => e.get_oid(),
            TreeEntry::Tree(t) => t.get_oid().expect("Tree should have an oid!"),
        }
    }

    pub fn is_tree(&self) -> bool {
        match self {
            TreeEntry::Entry(e) => e.get_mode() == TREE_MODE,
            _ => false,
        }
    }
}

/// A `Tree` is represented as a binary tree holding on its leafs
/// `TreeEntry` types.
#[derive(Clone)]
pub struct Tree {
    entries: BTreeMap<String, TreeEntry>,
}

impl Tree {
    /// Create a new `Tree` from an `Entry` vector.
    pub fn new() -> Self {
        trace!("Creating a new tree");
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Build a `Tree` from a list of `Entry`.
    pub fn build(entries: Vec<Entry>) -> Result<Tree> {
        trace!("Building a tree from entries");
        // Sort BTree by their names.
        // This results in a `Tree` only changing its oid if something meaningful
        // changes in its entries.
        let mut s_entries = entries.to_vec();
        s_entries.sort();

        // Create root tree which will be returned
        let mut root = Tree::new();
        for entry in s_entries.iter() {
            // for each entry find parent path
            let mut ppath: Vec<String> = Path::new(&entry.get_name())
                .iter()
                .map(|c| c.to_str().unwrap().to_string())
                .collect();
            let name = ppath
                .pop()
                .with_context(|| "Tree: File path of entry has zero componentes")?;
            root.add_entry(&ppath, name, entry.clone());
        }

        Ok(root)
    }

    /// Add an `Entry` to a `Tree`.
    fn add_entry(&mut self, ppath: &[String], name: String, entry: Entry) {
        trace!(
            "Adding entry {} from {:?} with name {} to tree",
            entry.get_name(),
            ppath,
            name
        );

        if ppath.is_empty() {
            self.entries.insert(name, TreeEntry::Entry(entry));
        } else if let Some(TreeEntry::Tree(t)) = self.entries.get_mut(&ppath[0]) {
            t.add_entry(&ppath[1..], name, entry);
        } else {
            let mut t = Tree::new();
            t.add_entry(&ppath[1..], name, entry);
            self.entries.insert(ppath[0].clone(), TreeEntry::Tree(t));
        }
    }

    /// Traverse this `Tree` and apply a function to each `Entry`.
    pub fn traverse<F>(&self, f: &F)
    where
        F: Fn(&Tree) -> (),
    {
        trace!("Traversing root tree");
        // Do a postorder traversal(visit all children first, then
        // process `self`
        for (_name, entry) in self.entries.clone() {
            if let TreeEntry::Tree(tree) = entry {
                tree.traverse(f);
            }
        }

        f(self);
    }
}

impl Object for Tree {
    fn get_type(&self) -> Types {
        trace!("Getting type, should be tree");
        Types::Tree
    }

    fn get_data(&self) -> Vec<u8> {
        trace!("Getting data of tree");
        let mut data = Vec::new();
        for (name, entry) in self.entries.iter() {
            let mut pre = format!("{} {}", entry.get_mode(), name).as_bytes().to_vec();
            pre.push(b"\x00"[0]);
            let oid = hex::decode(entry.get_oid()).expect("Tree: Could not decode hex");
            pre.extend(oid);
            data.extend(pre);
        }

        debug!("Data size is {}", data.len());
        data
    }

    fn set_oid(&mut self, hash: String) {
        trace!("Setting oid of tree");
        unimplemented!();
    }

    fn get_oid(&self) -> Option<String> {
        trace!("Getting oid of tree");
        unimplemented!();
    }
}
