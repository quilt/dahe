use crate::config::Config;
use crate::hex::HexData;
use anyhow::Result;
use ethereum_types::{Address, H256};
use ethers::core::k256::{
    ecdsa::{recoverable::Signature as RecoverableSignature, signature::Signer, SigningKey},
    elliptic_curve::FieldBytes,
    Secp256k1,
};
use sha3::{Digest, Keccak256};
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Sign {
    pub invoker: Address,
    pub params: Vec<HexData>,
    #[structopt(long, short, default_value = "0")]
    pub key: usize,
    #[structopt(long, short)]
    pub packed: bool,
}

pub fn sign(ctx: &Sign) -> Result<()> {
    let config = Config::open()?;
    let pk = config.fetch_key(ctx.key)?;

    let commit = compute_commit(&ctx.params);
    println!("commit:\t{}", hex::encode(commit.clone()));

    let sig = compute_sig(ctx.invoker, &commit, &pk)?;
    println!("{:?}", sig);

    Ok(())
}

struct SignatureValues {
    v: u8,
    r: H256,
    s: H256,
}

impl fmt::Display for SignatureValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.v,
            hex::encode(self.r),
            hex::encode(self.s)
        )
    }
}

impl fmt::Debug for SignatureValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v: {}\n", self.v)?;
        write!(f, "r: {}\n", hex::encode(self.r))?;
        write!(f, "s: {}\n", hex::encode(self.s))
    }
}

fn compute_sig(invoker: Address, commit: &[u8], pk: &[u8]) -> Result<SignatureValues> {
    let signer = SigningKey::from_bytes(&pk)?;

    let mut preimage = [0u8; 65];
    preimage[0] = 0x03;
    preimage[13..33].copy_from_slice(invoker.as_bytes());
    preimage[33..65].copy_from_slice(commit);

    println!("sig preimage: {}", hex::encode(preimage));
    let recoverable_sig: RecoverableSignature = signer.sign(&preimage);

    let v: u8 = recoverable_sig.recovery_id().into();
    let r_bytes: FieldBytes<Secp256k1> = recoverable_sig.r().into();
    let s_bytes: FieldBytes<Secp256k1> = recoverable_sig.s().into();
    let r = H256::from_slice(&r_bytes.as_slice());
    let s = H256::from_slice(&s_bytes.as_slice());

    Ok(SignatureValues { v, r, s })
}

fn compute_commit(params: &[HexData]) -> Vec<u8> {
    let mut preimage = Vec::new();
    for param in params {
        preimage.extend(&param.0);
    }

    println!("commit preimage: {}", hex::encode(&preimage));

    let mut hasher = Keccak256::new();
    hasher.input(&preimage[..]);

    let mut commit = [0; 32];
    commit.copy_from_slice(&hasher.result());
    commit.to_vec()
}
