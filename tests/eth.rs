#[cfg(test)]
mod tests {
    use cryptowallet::wallet::eth::generate_keypair;

    #[test]
    fn gen_keypair_successfully() {
        let (secret_key, pub_key) = generate_keypair();
        assert_eq!(secret_key.len(), 32);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // we need to check what properties there are and then test for them
}
