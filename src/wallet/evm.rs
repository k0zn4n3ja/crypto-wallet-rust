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

use super::hd::{gen_mnemonic, CoinType};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bip44Account {
    pub index: u32,
    // We include a string so that you can
    pub name: String,
    pub changes: HashMap<Bip44ChangeVal, Bip44Change>,
}

/// 0 for receiving address, 1 for internal address. all will be recieivng for now.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
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
pub struct Accounts {
    pub path: String,
    pub next_index: u32,
    pub accounts: HashMap<u32, Bip44Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub mnemonic: [u8; 32],
    pub coins: HashMap<CoinType, Accounts>,
}

impl Wallet {
    pub fn new() -> Result<Self> {
        let mnemonic = gen_mnemonic();
        let new_wallet = Wallet {
            mnemonic: *mnemonic.entropy(),
            coins: HashMap::new(),
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

    pub fn from_file(&self, file_path: &str) -> Result<Wallet> {
        // TODO password encryption
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader: BufReader<std::fs::File> = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub fn show_mnemonic(&self, file_path: &str) -> Result<String> {
        let wallet: Wallet = self.from_file(file_path)?;
        // English is currently the only supported language
        let mnemnonic: Mnemonic = Mnemonic::from_entropy(wallet.mnemonic, Language::English);
        Ok(mnemnonic.phrase().to_string())
    }

    /// Creates a new account for a given `CoinType` and associates it with an account name.
    ///
    /// This function is used to create a new account for a specific `CoinType` and associate it
    /// with a provided account name. If an account with the given `CoinType` already exists, the
    /// new account will be added to the existing ones.
    ///
    /// Follows bip44 specification.
    ///
    ///
    /// # Arguments
    ///
    /// - `coin`: A `CoinType` representing the type of cryptocurrency for the new account.
    /// - `account_name`: A string containing the name for the new account.
    ///
    /// # Returns
    ///
    /// - `Ok(u32)`: The function returns `Ok` with an account index (a non-negative integer) if
    ///   the operation is successful.
    /// - `Err`: The function returns an error if there is a problem with creating the account.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// let mut wallet = Wallet::new(); // Create an instance of your HD wallet
    ///
    /// let coin_type = CoinType::Bitcoin; // Define the coin type
    /// let account_name = "Savings"; // Define the account name
    ///
    /// match wallet.new_account(coin_type, account_name) {
    ///     Ok(account_index) => {
    ///         println!("New account created with index: {}", account_index);
    ///     }
    ///     Err(error) => {
    ///         eprintln!("Error creating a new account: {}", error);
    ///     }
    /// }
    ///
    /// TODO work out whether you want to have internal accounts
    ///
    /// ```
    pub fn new_account(&mut self, coin: CoinType, account_name: &str) -> Result<u32> {
        //
        let accounts_entry = self.coins.entry(coin);

        match accounts_entry {
            Entry::Vacant(vacant) => {
                let entry = Accounts {
                    path: format!("m/44'/{}", coin),
                    next_index: 0,
                    accounts: HashMap::new(),
                };
                let accounts = vacant.insert(entry);

                new_account(accounts, account_name)
            }
            Entry::Occupied(mut entry) => {
                let accounts = entry.get_mut();
                new_account(accounts, account_name)
            }
        }
    }
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

fn new_account(accounts: &mut Accounts, account_name: &str) -> Result<u32> {
    let index = accounts.next_index;

    let mut changes = HashMap::new();

    changes.insert(
        Bip44ChangeVal::RECEIVING,
        Bip44Change {
            change: Bip44ChangeVal::RECEIVING,
            next_address_index: 0,
            addresses: HashMap::new(),
        },
    );

    // create the new accounts
    accounts.accounts.insert(
        index,
        Bip44Account {
            index,
            name: String::from(account_name),
            changes,
        },
    );

    accounts.next_index += 1;
    Ok(index)
}
