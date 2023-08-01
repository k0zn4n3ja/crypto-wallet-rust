use anyhow::{bail, Result};
use rand::{thread_rng, Rng};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};
use tiny_keccak::keccak256;
use web3::types::Address;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub address: String,
}

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    // setup generator for random seed
    let mut rng = thread_rng();
    // Generate a random u64 seed
    let random_seed: u64 = rng.gen();
    // get random number for keypair from seed
    let mut rng = rngs::StdRng::seed_from_u64(random_seed);
    secp.generate_keypair(&mut rng)
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

impl Wallet {
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let addr: Address = address_from_pubkey(&public_key);
        Wallet {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            address: format!("{:?}", addr),
        }
    }
}
