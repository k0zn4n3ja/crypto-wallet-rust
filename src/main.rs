mod wallet;
use std::env;

use anyhow::Result;
use wallet::eth::{address_from_pubkey, establish_web3_connection, generate_keypair, Wallet};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    println!("Hello, world!");
    let (private, public) = generate_keypair();
    println!("private and public keys");
    println!("{}", private.display_secret().to_string());
    println!("{}", public);

    let address = address_from_pubkey(&public);

    println!("address: ");
    println!("{:?}", address);

    let crypto_wallet = Wallet::new(&private, &public);
    println!("crypto_wallet: {:?}", &crypto_wallet);

    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.save_to_file(wallet_file_path)?;
    let loaded_wallet = Wallet::from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    let endpoint = env::var("TESTNET_WS")?;
    let web3_con = establish_web3_connection(&endpoint).await?;
    let block_number = web3_con.eth().block_number().await?;
    println!("block number: {}", &block_number);
    Ok(())
}
