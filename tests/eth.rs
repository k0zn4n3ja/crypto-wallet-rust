#[cfg(test)]
mod tests {
    use cryptowallet::wallet::eth::generate_keypair;

    #[test]
    fn gen_keypair_successfully() {
        let (secret_key, pub_key) = generate_keypair();
        assert_eq!(secret_key.secret_bytes().len(), 32);
    }

    #[test]
    fn key_hex_length() {
        let (secret_key, pub_key) = generate_keypair();
        assert_eq!(secret_key.display_secret().to_string().len(), 64);
    }

    // #[test]
    // fn another() {
    //     assert_eq!(3 + 3, 6);
    // }

    // we need to check what properties there are and then test for them
}
