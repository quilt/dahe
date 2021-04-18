use crate::config::{self, Config, KeyInfo};
use anyhow::{bail, Result};
use eth_keystore::encrypt_key;
use ethers::{core::k256::ecdsa::SigningKey, signers::Wallet};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Import {
    /// Import a private key (alias = pk)
    PrivateKey(PrivateKey),
    /// Import a private from a mnemonic
    Mnemonic(Mnemonic),
}

#[derive(Debug, StructOpt)]
#[structopt(alias = "pk")]
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

pub fn import(ctx: &Import) -> Result<()> {
    match ctx {
        Import::PrivateKey(pk) => import_pk(&pk.key.0),
        Import::Mnemonic(m) => import_mnemonic(m.mnemonic.as_str(), m.index),
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
        password: pass == "",
    };

    let mut config = Config::open()?;
    config.keys.push(key);
    config.save()
}

pub fn import_mnemonic(_: &str, _: u64) -> Result<()> {
    unimplemented!("not supported (yet)")
}

#[derive(Debug)]
pub struct HexData(Vec<u8>);

impl std::str::FromStr for HexData {
    type Err = hex::FromHexError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if 1 < s.len() && s.starts_with("0x") {
            s = &s[2..];
        }
        hex::decode(s).map(HexData)
    }
}
