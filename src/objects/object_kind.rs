use crate::objects::blob::blob::Blob;
use crate::objects::object_base::GitObject;
use crate::objects::tree::tree::Tree;
use anyhow::Result;
use crate::objects::commit::commit::Commit;

#[derive(Debug)]
pub enum GitObjectKind {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl GitObjectKind {
    pub fn get_data(&self) -> Result<String> {
        let data = match self {
            GitObjectKind::Blob(blob) => blob.content.clone(),
            GitObjectKind::Tree(tree) => {
                let entries: Vec<String> = tree.entries.iter()
                    .map(|entry| format!("{}", entry.name))
                    .collect();
                entries.join("\n")
            },
            GitObjectKind::Commit(commit) => {
                todo!()
            }
        };
        Ok(data)
    }
}

impl GitObject for GitObjectKind {
    fn get_hash(&self) -> &str {
        match self {
            GitObjectKind::Blob(blob) => blob.get_hash(),
            GitObjectKind::Tree(tree) => tree.get_hash(),
            GitObjectKind::Commit(commit) => commit.get_hash(),
        }
    }

    fn get_header_prefix(&self) -> &'static str {
        match self {
            GitObjectKind::Blob(blob) => blob.get_header_prefix(),
            GitObjectKind::Tree(tree) => tree.get_header_prefix(),
            GitObjectKind::Commit(commit) => commit.get_header_prefix(),
        }
    }

    fn compute_size(&self) -> usize {
        match self {
            GitObjectKind::Blob(blob) => blob.compute_size(),
            GitObjectKind::Tree(tree) => tree.compute_size(),
            GitObjectKind::Commit(commit) => commit.compute_size(),
        }
    }

    fn compute_object_data(&self) -> Vec<u8> {
        match self {
            GitObjectKind::Blob(blob) => blob.compute_object_data(),
            GitObjectKind::Tree(tree) => tree.compute_object_data(),
            GitObjectKind::Commit(commit) => commit.compute_object_data(),
        }
    }

    fn from_object_file(_hash: &str, _content: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        panic!("Should not be called")
    }
}

