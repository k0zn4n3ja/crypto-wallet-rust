mod wallet;
use wallet::eth::generate_keypair;

fn main() {
    println!("Hello, world!");
    let (private, public) = generate_keypair();
    println!("{}", private);
    println!("{}", public);
}
