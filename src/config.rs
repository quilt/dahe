use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

pub fn config_dir() -> Result<PathBuf> {
    ProjectDirs::from("com", "quilt", "dahe")
        .map(|dirs| dirs.config_dir().to_owned())
        .ok_or(anyhow!("unable to determine config directory"))
}

pub fn init() -> Result<PathBuf> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;
    Ok(dir)
}
