use rand::{thread_rng, Rng};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};

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
