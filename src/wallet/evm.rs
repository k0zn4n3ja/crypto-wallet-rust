use anyhow::{bail, Result};
use bip32::Mnemonic;
use hex::encode;
use secp256k1::{
    rand::rngs::OsRng,
    Secp256k1, {PublicKey, SecretKey},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{collections::HashMap, io::BufWriter};
use std::{fs::OpenOptions, io::BufReader};
use tiny_keccak::keccak256;
use web3::{
    transports::{self, WebSocket},
    types::{Address, TransactionParameters, H256, U256},
    Web3,
};

use super::hd::{gen_mnemonic, CoinType};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bip44Account {
    pub next_index: u32,
    pub changes: HashMap<u32, Bip44Change>,
}

// 0 for receiving address, 1 for internal address. all will be recieivng for now.
#[derive(Serialize, Deserialize, Debug)]
pub enum Bip44ChangeVal {
    RECEIVING,
    INTERNAL,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bip44Change {
    pub change: Bip44ChangeVal,
    pub next_address_index: u32,
    pub addresses: HashMap<u32, Bip44Address>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bip44Address {
    pub path: String,
    pub address: String,
    pub address_checksummed: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub mnemonic: [u8; 32],
    pub accounts: HashMap<CoinType, Bip44Account>,
}

impl Wallet {
    pub fn new() -> Result<Self> {
        let mnemonic = gen_mnemonic();
        let new_wallet = Wallet {
            mnemonic: *mnemonic.entropy(),
            accounts: HashMap::new(),
        };
        Ok(new_wallet)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        // TODO password encryption
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }

    pub fn from_file(file_path: &str) -> Result<Wallet> {
        // TODO password encryption
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader: BufReader<std::fs::File> = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    // pub fn get_secret_key(&self) -> Result<SecretKey> {
    //     let secret_key = SecretKey::from_str(&self.secret_key)?;
    //     Ok(secret_key)
    // }
    // pub fn get_public_key(&self) -> Result<PublicKey> {
    //     let pub_key = PublicKey::from_str(&self.public_key)?;
    //     Ok(pub_key)
    // }

    // pub async fn get_balance(&self, web3_connection: &Web3<WebSocket>) -> Result<U256> {
    //     let wallet_address = Address::from_str(&self.address)?;
    //     let balance = web3_connection.eth().balance(wallet_address, None).await?;
    //     Ok(balance)
    // }
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

pub fn address_from_pubkey(pub_key: &PublicKey) -> Address {
    let public_key = pub_key.serialize_uncompressed();
    // use a result for this with proper error handling
    debug_assert_eq!(public_key[0], 0x04);
    // get hash from public key
    let hash = keccak256(&public_key[1..]);
    // use last twenty bytes from the hash
    Address::from_slice(&hash[12..])
}
