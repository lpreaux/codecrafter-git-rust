use std::path::PathBuf;
use crate::objects::objects;
use anyhow::{anyhow, Result};

pub struct GitObjectBase {
    pub hash: String,
}

pub(crate) trait GitObject {
    fn get_hash(&self) -> &str;
    fn get_header_prefix(&self) -> &'static str;
    fn get_header(&self) -> Vec<u8> {
        format!("{} {}", self.get_header_prefix(), self.compute_size()).into_bytes()
    }
    fn compute_file_path(&self) -> Result<PathBuf> {
        if self.get_hash().len() != objects::OBJECT_HASH_SIZE {
            return Err(anyhow!("Invalid object identifier: expected {} characters, got {}", objects::OBJECT_HASH_SIZE, self.get_hash().len()));
        }

        // Sépare le hash en deux parties : le répertoire (les deux premiers caractères) et le fichier
        let (dir, file) = self.get_hash().split_at(2);
        Ok(PathBuf::from(objects::GIT_OBJECTS_DIR).join(dir).join(file))
    }
    fn compute_size(&self) -> usize;
    fn compute_object_data(&self) -> Vec<u8>;
    fn from_data(hash: &str, content: &[u8]) -> Result<Self>
    where
        Self: Sized;
}