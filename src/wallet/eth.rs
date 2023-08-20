use anyhow::{bail, Result};

use hex::encode;
use secp256k1::{
    rand::rngs::OsRng,
    Secp256k1, {PublicKey, SecretKey},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::BufWriter};
use std::{fs::OpenOptions, io::BufReader};
use std::{hash::Hash, str::FromStr};
use tiny_keccak::keccak256;
use web3::{
    transports::{self, WebSocket},
    types::{Address, TransactionParameters, H256, U256},
    Web3,
};

use super::hd::CoinType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bip44Account {
    pub index: u32,
    pub changes: Vec<Bip44Change>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bip44Change {
    pub index: u32,
    pub addresses: Vec<Bip44Address>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bip44Address {
    pub index: u32,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub address: String,
    pub address_checksummed: String,
    pub accounts: HashMap<CoinType, Bip44Account>,
}

impl Wallet {
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let addr: Address = address_from_pubkey(&public_key);
        Wallet {
            secret_key: secret_key.display_secret().to_string(),
            public_key: public_key.to_string(),
            address: format!("{:?}", addr),
            address_checksummed: to_checksum_address(&addr),
            accounts: HashMap::new(),
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }

    pub fn from_file(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader: BufReader<std::fs::File> = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = SecretKey::from_str(&self.secret_key)?;
        Ok(secret_key)
    }
    pub fn get_public_key(&self) -> Result<PublicKey> {
        let pub_key = PublicKey::from_str(&self.public_key)?;
        Ok(pub_key)
    }

    pub async fn get_balance(&self, web3_connection: &Web3<WebSocket>) -> Result<U256> {
        let wallet_address = Address::from_str(&self.address)?;
        let balance = web3_connection.eth().balance(wallet_address, None).await?;
        Ok(balance)
    }
}

/// Generates a keypair using OS Rng.
/// For all major platforms, OS Rng is a CSPRNG with physical entropy as seed.
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    secp.generate_keypair(&mut OsRng)
}

pub fn address_from_pubkey(pub_key: &PublicKey) -> Address {
    let public_key = pub_key.serialize_uncompressed();
    // use a result for this with proper error handling
    debug_assert_eq!(public_key[0], 0x04);
    // get hash from public key
    let hash = keccak256(&public_key[1..]);
    // use last twenty bytes from the hash
    Address::from_slice(&hash[12..])
}

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
