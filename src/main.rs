mod wallet;
use wallet::eth::{address_from_pubkey, generate_keypair, Wallet};

fn main() {
    println!("Hello, world!");
    let (private, public) = generate_keypair();
    println!("private and public keys");
    println!("{}", private);
    println!("{}", public);

    let address = address_from_pubkey(&public);

    println!("address: ");
    println!("{:?}", address);

    let crypto_wallet = Wallet::new(&private, &public);
    println!("crypto_wallet: {:?}", &crypto_wallet);
}
