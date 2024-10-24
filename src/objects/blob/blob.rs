use crate::objects::object_base::{GitObject, GitObjectBase};
use crate::objects::utils;
use anyhow::{anyhow, Result};

pub(crate) const HEADER_PREFIX: &str = "blob";

#[derive(Debug)]
pub struct Blob {
    pub base: GitObjectBase,
    pub content: String,
}

impl Blob {
    pub(crate) fn new(content: Vec<u8>) -> Result<Blob>
    {
        let size = content.len();
        let content = String::from_utf8(content).map_err(|e| anyhow!("Invalid UTF-8 in object file: {}", e))?;
        let blob_data = format!("{} {}{}{}", HEADER_PREFIX, size, "\0", content);
        let hash = utils::compute_sha1(&blob_data);
        Ok(Blob {
            base: GitObjectBase {
                hash,
            },
            content,
        })
    }
}


impl GitObject for Blob {
    fn get_hash(&self) -> &str {
        &self.base.hash
    }

    fn get_header_prefix(&self) -> &'static str {
        HEADER_PREFIX
    }

    fn compute_size(&self) -> usize {
        self.content.as_bytes().len()
    }

    fn compute_object_data(&self) -> Vec<u8> {
        let header_str = self.get_header();
        let header = header_str.as_bytes();
        let mut result = Vec::with_capacity(header.len() + self.compute_size());

        result.extend_from_slice(header);

        result.extend_from_slice(&self.content.as_bytes());

        result
    }

    fn from_object_file(hash: &str, content: &[u8]) -> Result<Blob> {
        let content_str = std::str::from_utf8(content)?.to_string();
        Ok(Blob {
            base: GitObjectBase {
                hash: hash.to_string(),
            },
            content: content_str,
        })
    }
}
