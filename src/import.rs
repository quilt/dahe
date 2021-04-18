use crate::config::{self, Config, KeyInfo};
use crate::hex::HexData;
use anyhow::{bail, Result};
use eth_keystore::encrypt_key;
use ethers::{core::k256::ecdsa::SigningKey, signers::Wallet};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Import {
    /// Imports a private key
    #[structopt(name = "pk")]
    PrivateKey(PrivateKey),
    /// Imports a derived private key from a mnemonic
    Mnemonic(Mnemonic),
    /// Imports a keystore file
    Keystore(Keystore),
}

#[derive(Debug, StructOpt)]
pub struct PrivateKey {
    /// Private key
    pub key: HexData,
}

#[derive(Debug, StructOpt)]
pub struct Mnemonic {
    /// Mnemonic value
    pub mnemonic: String,
    /// Imports the account derived at the supplied index
    #[structopt(short, long, default_value = "0")]
    pub index: u64,
}

#[derive(Debug, StructOpt)]
pub struct Keystore {
    /// Path
    pub path: PathBuf,
}

pub fn import(ctx: &Import) -> Result<()> {
    match ctx {
        Import::PrivateKey(pk) => import_pk(&pk.key.0),
        Import::Mnemonic(_) => unimplemented!(),
        Import::Keystore(_) => unimplemented!(),
    }
}

pub fn import_pk(pk: &[u8]) -> Result<()> {
    let w: Wallet<SigningKey> = SigningKey::from_bytes(pk)?.into();
    let address = format!("{:?}", w.address());
    println!("importing {}", address);

    let pass = rpassword::read_password_from_tty(Some("Password: "))?;
    let pass2 = rpassword::read_password_from_tty(Some("Password (again): "))?;

    if pass != pass2 {
        bail!("passwords did not match");
    }

    let config_dir = config::init()?;
    let uuid = encrypt_key(&config_dir, &mut rand::thread_rng(), &pk, &pass)?;

    let key = KeyInfo {
        path: config_dir.join(uuid),
        address,
        password: pass != "",
    };

    let mut config = Config::open()?;
    config.keys.push(key);
    config.save()
}
