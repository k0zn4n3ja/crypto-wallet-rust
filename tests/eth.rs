#[cfg(test)]
mod tests {
    use std::u64;

    use cryptowallet::wallet::eth::{address_from_pubkey, generate_keypair, Wallet};
    use rand::RngCore;
    use secp256k1::rand::{Rng, SeedableRng};
    use secp256k1::{PublicKey, Secp256k1, SecretKey};

    #[test]
    fn gen_correct_pubkey() {
        let (_secret_key, pub_key) = generate_keypair();
        // first byte as 0x04 indicates uncompressed key
        assert_eq!(pub_key.serialize_uncompressed()[0], 0x04);
        // with the first byte and the key we expect 1 + 64 bytes for the entire public key
        assert_eq!(pub_key.serialize_uncompressed().len(), 65);
    }

    #[test]
    fn gen_correct_private_key() {
        let (secret_key, _pub_key) = generate_keypair();
        // the 32 bytes are displayed as a 64 byte hexadecimal char string
        assert_eq!(secret_key.display_secret().to_string().len(), 64);
        // the actual length of the byte array is 32
        assert_eq!(secret_key.secret_bytes().len(), 32);
    }

    #[test]
    fn pub_key_to_address_correct_hash() {
        let secp = Secp256k1::new();
        // we use a non-random secret key for testing purposes
        let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");

        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let address = address_from_pubkey(&public_key);
        // test it with the representative hash
        assert_eq!(address.as_bytes().len(), 20);
        // address prevalidated using trusted tool
        assert_eq!(
            format!("{:?}", address),
            "0x89aef553a06ab0c3173e79de1ce241a9ed3b992c"
        )
    }

    // think about what other props the address with have
    #[test]
    fn address_is_checksumed() {
        let secp = Secp256k1::new();
        // we use a non-random secret key for testing purposes
        let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");

        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let wallet = Wallet::new(&secret_key, &public_key);

        println!("{}", wallet.address_checksummed);

        assert_eq!(
            "0x89AEF553A06ab0C3173e79DE1Ce241A9ed3b992C",
            &wallet.address_checksummed
        )
    }

    // we need to check what properties there are and then test for them
}
