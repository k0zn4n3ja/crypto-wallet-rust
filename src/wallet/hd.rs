use bip39::{Error, Language, Mnemonic};
use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::hash::Hash;
use tiny_hderive::bip32::ExtendedPrivKey;

/// BIP-44 compliant enum for major coin types for the wallet
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum CoinType {
    Bitcoin = 0,
    BitcoinTestnet = 1,
    Monero = 128,
    Ethereum = 60,
    EthereumClassic = 61,
    ZCash = 133,
}

pub fn mnemonic_to_seed(mnemonic: &Mnemonic, maybe_passphrase: Option<&str>) -> [u8; 64] {
    mnemonic.to_seed(maybe_passphrase.unwrap_or(""))
}

pub fn master_key(seed: &[u8; 64], chain: CoinType) -> ExtendedPrivKey {
    let interpolated_path = format!("m/44'/{}'/0'/0/0", chain as u32);
    ExtendedPrivKey::derive(seed, &*interpolated_path).unwrap()
}
