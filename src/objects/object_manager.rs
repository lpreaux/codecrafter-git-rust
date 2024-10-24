use crate::fs_reader::FSReader;
use crate::objects::object_kind::GitObjectKind;
use crate::objects::object_transformer::GitObjectTransformer;
use crate::objects::object_writer::GitObjectWriter;
use crate::objects::objet_reader::GitObjectReader;
use anyhow::Result;
use std::path::Path;
use crate::objects::commit::commit::Commit;

pub(crate) const OBJECT_CONTENT_SEPARATOR: u8 = 0;
pub(crate) const OBJECT_HASH_SIZE: usize = 40;
pub(crate) const GIT_OBJECTS_DIR: &str = ".git/objects";

pub fn read_object(hash: &str) -> Result<GitObjectKind> {
    let reader = GitObjectReader;
    reader.read_object(hash)
}

pub fn create_object(path: &Path) -> Result<GitObjectKind> {
    let reader = FSReader;
    let transformer = GitObjectTransformer::new(&reader);
    let writer = GitObjectWriter;

    let object = transformer.transform_fs_to_object(path)?;
    writer.write_object(&object)?;

    Ok(object)
}

pub fn create_commit(tree_hash: &String, parent_hash: &Option<String>, message: &String) -> Result<GitObjectKind> {
    let writer = GitObjectWriter;

    let commit = GitObjectKind::Commit(Commit::new(tree_hash, parent_hash, message, None)?);
    writer.write_object(&commit)?;

    Ok(commit)
}