use bip39::{Error, Language, Mnemonic};
use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::hash::Hash;

/// BIP-44 compliant enum for major coin types for the wallet
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum CoinType {
    Bitcoin = 0,
    BitcoinTestnet = 1,
    Dogecoin = 3,
    Ripple = 144,
    Monero = 128,
    Ethereum = 60,
    EthereumClassic = 61,
    Cardano = 1815,
    Tezos = 1729,
    ZCash = 133,
}

impl CoinType {
    pub fn derivation_path(&self) -> String {
        format!("m/44'/{}", *self as u32)
    }
}

fn mnemonic_to_seed(mnemonic: &Mnemonic, maybe_passphrase: Option<&str>) -> [u8; 64] {
    mnemonic.to_seed(maybe_passphrase.unwrap_or(""))
}
