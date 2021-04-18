use anyhow::{anyhow, bail, Result};
use directories::ProjectDirs;
use eth_keystore::decrypt_key;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};

pub fn config_dir() -> Result<PathBuf> {
    ProjectDirs::from("com", "quilt", "dahe")
        .map(|dirs| dirs.config_dir().to_owned())
        .ok_or(anyhow!("unable to determine config directory"))
}

pub fn config_file() -> Result<PathBuf> {
    config_dir().map(|dir| dir.join("keys.toml"))
}

pub fn init() -> Result<PathBuf> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub keys: Vec<KeyInfo>,
}

impl Config {
    pub fn open() -> Result<Config> {
        match fs::read_to_string(config_file()?) {
            Ok(s) => Ok(toml::from_str(&s)?),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(Config::default())
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub fn fetch_key(&self, n: usize) -> Result<Vec<u8>> {
        if self.keys.len() <= n {
            bail!("unknown key {}, max {}", n, self.keys.len())
        }

        let key = &self.keys[n];

        let mut pass = String::new();
        if key.password {
            pass = rpassword::read_password_from_tty(Some("Password: "))?;
        }

        let k = decrypt_key(&key.path, pass)?;
        Ok(k)
    }

    pub fn save(&self) -> Result<()> {
        let mut f = fs::File::create(config_file()?)?;
        f.write_all(toml::to_vec(&self)?.as_ref())?;
        Ok(())
    }

    pub fn print(&self) {
        if self.keys.is_empty() {
            println!("Keystore empty.");
            return;
        }

        println!("Available keys");
        println!("---");

        for (i, key) in self.keys.iter().enumerate() {
            println!("{}:\t{}", i, key.address);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyInfo {
    pub path: PathBuf,
    pub address: String,
    pub password: bool,
}
