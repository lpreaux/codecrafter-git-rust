use crate::fs_reader::FSReader;
use crate::objects::blob::blob::Blob;
use crate::objects::object_base::GitObject;
use crate::objects::object_kind::GitObjectKind;
use crate::objects::tree::tree::{Tree, TreeEntry};
use anyhow::Result;
use std::path::Path;

pub struct GitObjectTransformer<'a> {
    fs_reader: &'a FSReader,
}

impl<'a> GitObjectTransformer<'a> {
    pub fn new(fs_reader: &'a FSReader) -> Self {
        GitObjectTransformer { fs_reader }
    }

    pub fn transform_fs_to_object(&self, root_path: &Path) -> Result<GitObjectKind> {
        if root_path.is_file() {
            let blob = self.transform_file_to_blob(root_path)?;
            Ok(GitObjectKind::Blob(blob))
        } else if root_path.is_dir() {
            let tree = self.transform_directory_to_tree(root_path)?;
            Ok(GitObjectKind::Tree(tree))
        } else {
            Err(anyhow::anyhow!("Invalid path: expected a file or directory, but got {:?}", root_path))
        }
    }

    fn transform_directory_to_tree(&self, dir_path: &Path) -> Result<Tree> {
        let mut entries = vec![];

        let fs_entries = self.fs_reader.read_directory(dir_path)?;

        for path in fs_entries {
            if let Some(file_name) = path.file_name() {
                if file_name == ".git" {
                    continue;
                }
                let name = file_name.to_string_lossy().into_owned();
                let mode = if path.is_file() { "100644" } else { "40000" }.into();

                if let Some(blob_or_tree) = self.transform_fs_to_object(&path).ok() {
                    let hash = blob_or_tree.get_hash().to_string();
                    entries.push(TreeEntry {
                        mode,
                        name,
                        hash,
                        object: Some(blob_or_tree),
                    });
                }
            }
        }

        Tree::new(entries)
    }

    fn transform_file_to_blob(&self, file_path: &Path) -> Result<Blob> {
        let content = self.fs_reader.read_file(file_path)?;
        Blob::new(content)
    }
}
