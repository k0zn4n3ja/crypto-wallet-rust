use anyhow::{bail, Result};
use bip32::{Language, Mnemonic};
use hex::encode;
use secp256k1::{
    rand::rngs::OsRng,
    Secp256k1, {PublicKey, SecretKey},
};
use serde::{Deserialize, Serialize};
use std::{collections::hash_map::Entry, str::FromStr};
use std::{collections::HashMap, io::BufWriter};
use std::{fs::OpenOptions, io::BufReader};
use tiny_keccak::keccak256;
use web3::{
    transports::{self, WebSocket},
    types::{Address, ChangedType, TransactionParameters, H256, U256},
    Web3,
};

use super::core::{gen_mnemonic, CoinType};

pub async fn establish_web3_connection(url: &str) -> Result<Web3<WebSocket>> {
    let transport = web3::transports::WebSocket::new(url).await?;
    Ok(web3::Web3::new(transport))
}

pub async fn sign_and_send(
    web3: &Web3<transports::WebSocket>,
    transaction: TransactionParameters,
    secret_key: &SecretKey,
) -> Result<H256> {
    let signed = web3
        .accounts()
        .sign_transaction(transaction, secret_key)
        .await?;
    let transaction_result = web3
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;
    Ok(transaction_result)
}

pub fn to_checksum_address(address: &Address) -> String {
    let addr = address.clone();

    let address_lower: String = format!("{:?}", addr);
    let chars: Vec<char> = address_lower.chars().collect();
    let address_lower_hex: String = chars[2..].into_iter().collect();
    let addr_hash = encode(keccak256(address_lower_hex.as_bytes()));

    format!(
        "0x{}",
        address_lower_hex
            .char_indices()
            .map(
                |(index, character)| match (character, addr_hash.chars().nth(index).unwrap()) {
                    (c, h) if h > '7' => c.to_uppercase().to_string(),
                    (c, _) => c.to_string(),
                },
            )
            .collect::<Vec<String>>()
            .join("")
    )
}

pub fn address_from_pubkey(pub_key: [u8; 65]) -> Address {
    // let pub_key = pub_key.serialize_uncompressed();
    // use a result for this with proper error handling
    debug_assert_eq!(pub_key[0], 0x04);
    // get hash from public key
    let hash = keccak256(&pub_key[1..]);
    // use last twenty bytes from the hash
    Address::from_slice(&hash[12..])
}
