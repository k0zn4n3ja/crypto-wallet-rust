# 🗝️ crypto-wallet-rust
Crypto wallet 'seeded' from a tutorial and now developed out with a full suite of features.

Original tuorial [here](https://tms-dev-blog.com/build-a-crypto-wallet-using-rust)

## Design Notes

The wallet is a BIP32, BIP39 and BIP44 compliant HD wallet. Currently wallet state is saved to file as JSON and encrypted with a password, later if I feel like it I will move to a more resilient system. 

User interacts with their wallet via a CLI. 

## Some Helpful Documentation for the Learners Out There

[Trezor docs on BIP44](https://trezor.io/learn/a/what-is-bip44)
