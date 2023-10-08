use bip39::{Error, Mnemonic};
use rand::{rngs::OsRng, Rng};

/// For all major platforms, OS Rng is a CSPRNG with physical entropy as seed.
/// Going embedded will result in an update
pub fn gen_mnemonic() -> Result<Mnemonic, Error> {
    let mut buffer: [u8; 32] = [0; 32];
    OsRng.fill(&mut buffer);
    Mnemonic::from_entropy(&buffer)
}
