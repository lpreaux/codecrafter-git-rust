use crate::objects::object_base::{GitObject, GitObjectBase};
use crate::objects::objects;
use anyhow::Result;

pub(crate) const HEADER_PREFIX: &str = "blob";

pub struct Blob {
    pub base: GitObjectBase,
    pub content: String,
}

impl Blob {

}

impl GitObject for Blob {
    fn get_hash(&self) -> &str {
        &self.base.hash
    }

    fn get_header_prefix(&self) -> &'static str {
        HEADER_PREFIX
    }

    fn compute_size(&self) -> usize {
        self.content.len()
    }

    fn compute_object_data(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.get_header().len() + 1 +self.compute_size());

        result.extend_from_slice(&self.get_header());

        result.push(objects::OBJECT_CONTENT_SEPARATOR);

        result.extend_from_slice(&self.content.as_bytes());

        result
    }

    fn from_data(hash: &str, content: &[u8]) -> Result<Blob> {
        let content_str = std::str::from_utf8(content)?.to_string();
        Ok(Blob {
            base: GitObjectBase {
                hash: hash.to_string(),
            },
            content: content_str,
        })
    }
}
