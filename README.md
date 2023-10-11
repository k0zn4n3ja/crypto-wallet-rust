# 🗝️ Wolet

Wolet is a BIP32, BIP39 and BIP44 compliant HD wallet. Currently wallet state is saved to file as JSON and encrypted with a password, later if I feel like it I will move to a more resilient system.

The CLI implementation is subpar, this will be converted to a tauri application with a GUI.

## Reference Specifications

[The original bip32 spec](https://github.com/satoshilabs/slips/blob/master/slip-0032.md)

[The original bip39 spec](https://github.com/satoshilabs/slips/blob/master/slip-0039.md)

[The original bip44 spec](https://github.com/satoshilabs/slips/blob/master/slip-0044.md)

## Some Helpful Documentation for the Learners Out There

[Trezor docs on BIP44](https://trezor.io/learn/a/what-is-bip44)

[Article on hardened key derivation](https://medium.com/@blainemalone01/hd-wallets-why-hardened-derivation-matters-89efcdc71671)
