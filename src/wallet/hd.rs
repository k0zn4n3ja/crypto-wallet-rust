use anyhow::Error;
use bip32::secp256k1::ecdsa::{SigningKey, VerifyingKey};
use bip32::{ExtendedPrivateKey, ExtendedPublicKey, Mnemonic, Prefix, Seed, XPrv};
use rand::{rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::fmt;
use std::hash::Hash;

const INVALID_BIP44_PATH: &str = "invalid bip44 path format";
const UNHARDENED_KEY: &str = "must harden child key";

impl fmt::Display for CoinType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoinType::Bitcoin => write!(f, "0"),
            CoinType::BitcoinTestnet => write!(f, "1"),
            CoinType::Ethereum => write!(f, "60"),
        }
    }
}

/// BIP-44 compliant enum for major coin types for the wallet
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum CoinType {
    // secp256k1
    // need find address algorithm
    Bitcoin = 0,
    // probably the same
    // check if address is different
    BitcoinTestnet = 1,
    // you've already done these
    Ethereum = 60,
}

pub fn gen_mnemonic() -> Mnemonic {
    let mnemonic: Mnemonic = Mnemonic::random(&mut OsRng, Default::default());
    mnemonic
}

pub fn mnemonic_to_seed(mnemonic: &Mnemonic, maybe_passphrase: Option<&str>) -> Seed {
    // mnemonic.to_seed(maybe_passphrase.unwrap_or(""))
    mnemonic.to_seed(maybe_passphrase.unwrap_or(""))
}

pub struct DerivedKeyPair {
    pub priv_key: ExtendedPrivateKey<SigningKey>,
    pub pub_key: ExtendedPublicKey<VerifyingKey>,
}

pub fn derive_child(seed: &Seed, path: &str) -> Result<DerivedKeyPair, Error> {
    validate_bip_44_path(path)?;
    let priv_key = XPrv::derive_from_path(&seed, &path.parse()?)?;
    let pub_key = priv_key.public_key();
    if !priv_key.attrs().child_number.is_hardened() {
        return Err(Error::msg(UNHARDENED_KEY));
    }
    if !pub_key.attrs().child_number.is_hardened() {
        return Err(Error::msg(UNHARDENED_KEY));
    }
    Ok(DerivedKeyPair { priv_key, pub_key })
}

// private utility functions

fn validate_bip_44_path(path: &str) -> Result<(), Error> {
    // Split the input string by '/'
    let parts: Vec<&str> = path.split('/').collect();
    // Check that the string starts with "m"
    if parts.get(0) != Some(&"m") {
        return Err(Error::msg(INVALID_BIP44_PATH));
    }
    // Check that there are exactly 5 parts in the path
    if parts.len() != 5 {
        return Err(Error::msg(INVALID_BIP44_PATH));
    }
    // Check that each part is a valid number
    for i in 1..=4 {
        if !parts[i].parse::<i64>().is_ok() {
            return Err(Error::msg(INVALID_BIP44_PATH));
        }
    }
    // Check that the third part ends with "'"
    if !parts[2].ends_with("'") {
        return Err(Error::msg(INVALID_BIP44_PATH));
    }
    Ok(())
}
