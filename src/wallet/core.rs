use super::evm::{address_from_pubkey, to_checksum_address};
use anyhow::{Error, Result};
use bip32::{
    secp256k1::ecdsa::{SigningKey, VerifyingKey},
    ExtendedPrivateKey, ExtendedPublicKey, Language, Mnemonic, Seed, XPrv,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Eq,
    collections::{hash_map::Entry, HashMap},
    fmt,
    fs::OpenOptions,
    hash::Hash,
    io::{BufReader, BufWriter},
};

// ERR MESSAGES
const INVALID_BIP44_PATH_FORMT: &str = "invalid bip44 path format";
const UNHARDENED_KEY: &str = "must harden child key";
const INVALID_WALLET_PATH: &str = "invalid wallet path for current state";
// TODO change this path
const WALLET_FILE_PATH: &str = "crypto_wallet.json";

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Bip44Account {
    pub index: u32,
    // We include a string so that you can
    pub name: String,
    pub changes: HashMap<Bip44ChangeVal, Bip44Change>,
}

impl fmt::Display for Bip44ChangeVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bip44ChangeVal::INTERNAL => write!(f, "1"),
            Bip44ChangeVal::RECEIVING => write!(f, "0"),
        }
    }
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
        new_wallet.save_to_file()?;
        Ok(new_wallet)
    }

    fn save_to_file(&self) -> Result<()> {
        // TODO password encryption
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(WALLET_FILE_PATH)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }

    pub fn from_file() -> Result<Self> {
        // TODO password encryption
        let file = OpenOptions::new().read(true).open(WALLET_FILE_PATH)?;
        let buf_reader: BufReader<std::fs::File> = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub fn show_mnemonic(&self) -> Result<String> {
        let wallet: Wallet = Wallet::from_file()?;
        // English is currently the only supported language
        let mnemnonic: Mnemonic = Mnemonic::from_entropy(wallet.mnemonic, Language::English);
        Ok(mnemnonic.phrase().to_string())
    }

    fn mnemonic_as_seed(&self, maybe_passphrase: Option<&str>) -> Seed {
        let mnemonic: Mnemonic = Mnemonic::from_entropy(self.mnemonic, Language::English);
        mnemonic.to_seed(maybe_passphrase.unwrap_or(""))
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

    pub fn new_address(&mut self, prms: NewAddressParams) -> Result<()> {
        if let Some(coin_accts) = self.coins.get(&prms.coin) {
            if let Some(acct) = coin_accts.accounts.get(&prms.account) {
                if let Some(change) = acct.changes.get(&prms.change) {
                    let index = change.next_address_index;
                    let path: &str = &format!(
                        "m/44'/{}/{}/{}/{}",
                        prms.coin, prms.change, prms.account, index
                    );
                    validate_bip_44_path(path)?;
                    let key_pair = derive_child(&self.mnemonic_as_seed(None), path)?;

                    // TODO make sure you incorporate the way bitcoin addresses are derived
                    // ALSO REFACTOR address from pubkey fuckery
                    //    let pub_key = address_from_pubkey(key_pair.pub_key)?;

                    //     let new_address = Bip44Address {
                    //         path: path.to_string(),
                    //         address,
                    //         address_checksummed:
                    //     }

                    return Ok(());
                }
            }
        }

        Err(Error::msg(""))
    }
}

pub struct NewAddressParams {
    coin: CoinType,
    // TODO check if u32 is the appropriate size for this
    account: u32,
    change: Bip44ChangeVal,
}

pub fn gen_mnemonic() -> Mnemonic {
    let mnemonic: Mnemonic = Mnemonic::random(&mut OsRng, Default::default());
    mnemonic
}

pub struct DerivedKeyPair {
    pub priv_key: ExtendedPrivateKey<SigningKey>,
    pub pub_key: ExtendedPublicKey<VerifyingKey>,
}

pub fn derive_child(seed: &Seed, path: &str) -> Result<DerivedKeyPair, Error> {
    validate_bip_44_path(path)?;
    let priv_key = XPrv::derive_from_path(&seed, &path.parse()?)?;
    let pub_key = priv_key.public_key();
    Ok(DerivedKeyPair { priv_key, pub_key })
}

// private utility functions

fn validate_bip_44_path(path: &str) -> Result<(), Error> {
    // Split the input string by '/'
    let parts: Vec<&str> = path.split('/').collect();
    // Check that the string starts with "m"
    if parts.get(0) != Some(&"m") {
        return Err(Error::msg(INVALID_BIP44_PATH_FORMT));
    }
    // Check that there are exactly 5 parts in the path
    if parts.len() != 5 {
        return Err(Error::msg(INVALID_BIP44_PATH_FORMT));
    }
    // Check that each part is a valid number
    for i in 1..=4 {
        if !parts[i].parse::<i64>().is_ok() {
            return Err(Error::msg(INVALID_BIP44_PATH_FORMT));
        }
    }
    // Check that the third part ends with "'"
    if !parts[2].ends_with("'") {
        return Err(Error::msg(INVALID_WALLET_PATH));
    }
    Ok(())
}

/// util for reused logic in Wallet::new_account function
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

    changes.insert(
        Bip44ChangeVal::INTERNAL,
        Bip44Change {
            change: Bip44ChangeVal::INTERNAL,
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
