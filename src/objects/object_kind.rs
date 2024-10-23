use crate::objects::blob::blob::Blob;
use crate::objects::object_base::GitObject;
use crate::objects::tree::tree::Tree;
use anyhow::Result;

pub enum GitObjectKind {
    Blob(Blob),
    Tree(Tree),
    // Commit(Commit),  // Si vous voulez ajouter d'autres types à l'avenir
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
            // GitObjectKind::Commit(commit) => &commit.base.hash,
        }
    }

    fn get_header_prefix(&self) -> &'static str {
        match self {
            GitObjectKind::Blob(blob) => blob.get_header_prefix(),
            GitObjectKind::Tree(tree) => tree.get_header_prefix(),
            // GitObjectKind::Commit(commit) => &commit.base.hash,
        }
    }

    fn compute_size(&self) -> usize {
        match self {
            GitObjectKind::Blob(blob) => blob.compute_size(),
            GitObjectKind::Tree(tree) => tree.compute_size(),
            // GitObjectKind::Commit(commit) => &commit.base.hash,
        }
    }

    fn compute_object_data(&self) -> Vec<u8> {
        match self {
            GitObjectKind::Blob(blob) => blob.compute_object_data(),
            GitObjectKind::Tree(tree) => tree.compute_object_data(),
        }
    }

    fn from_data(_hash: &str, _content: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        panic!("Should not be called")
    }
}

