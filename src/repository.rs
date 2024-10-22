use std::fs;
use anyhow::Result;

pub fn init_repository() -> Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    Ok(println!("Initialized git directory"))
}