use anyhow::{Error, Result};
use bitcoin::{address::Address, Network, PublicKey as PubKeyStructBitcoin};
use secp256k1::{Parity, PublicKey, XOnlyPublicKey};

const INVALID_PARITY_ON_COMPRESSED_KEY: &str = "invalid parity on compressed key";

pub fn address_from_compressed_pub_key(pub_key: [u8; 33], network: Network) -> Result<Address> {
    let key_parity = parity_from_u8(pub_key[0])?;
    let key_x_val = XOnlyPublicKey::from_slice(&pub_key[1..33])?;
    let formatted_key = PublicKey::from_x_only_public_key(key_x_val, key_parity);

    let pub_struct = PubKeyStructBitcoin {
        compressed: false,
        inner: formatted_key,
    };
    Ok(Address::p2pkh(&pub_struct, network))
}

fn parity_from_u8(int: u8) -> Result<Parity, Error> {
    match int {
        2 => Ok(Parity::Even),
        3 => Ok(Parity::Odd),
        _ => Err(Error::msg(INVALID_PARITY_ON_COMPRESSED_KEY)),
    }
}
