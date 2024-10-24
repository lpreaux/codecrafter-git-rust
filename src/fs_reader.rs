use std::path::{Path, PathBuf};
use anyhow::Result;

pub struct FSReader;

impl FSReader {
    pub fn read_file(&self, file_path: &Path) -> Result<Vec<u8>> {
        std::fs::read(file_path).map_err(|e| anyhow::anyhow!(e))
    }

    pub fn read_directory(&self, dir_path: &Path) -> Result<Vec<PathBuf>> {
        let mut entries = vec![];

        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            entries.push(entry.path());
        }

        Ok(entries)
    }
}
