use std::path::PathBuf;
use crate::objects::object_manager;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct GitObjectBase {
    pub hash: String,
}

pub trait GitObject {
    fn get_hash(&self) -> &str;
    fn get_header_prefix(&self) -> &'static str;
    fn get_header(&self) -> String {
        format!("{} {}\0", self.get_header_prefix(), self.compute_size())
    }
    fn compute_file_path(&self) -> Result<PathBuf> {
        if self.get_hash().len() != object_manager::OBJECT_HASH_SIZE {
            return Err(anyhow!("Invalid object identifier: expected {} characters, got {}", object_manager::OBJECT_HASH_SIZE, self.get_hash().len()));
        }

        // Sépare le hash en deux parties : le répertoire (les deux premiers caractères) et le fichier
        let (dir, file) = self.get_hash().split_at(2);
        Ok(PathBuf::from(object_manager::GIT_OBJECTS_DIR).join(dir).join(file))
    }
    fn compute_size(&self) -> usize;
    fn compute_object_data(&self) -> Vec<u8>;
    fn from_object_file(hash: &str, content: &[u8]) -> Result<Self>
    where
        Self: Sized;
}