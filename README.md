# 🗝️ Wolet

Wolet is a BIP32, BIP39 and BIP44 compliant HD wallet. Currently wallet state is saved to file as JSON and encrypted with a password, later if I feel like it I will move to a more resilient system.

I will do a front end for it when I feel like it.

## HD Wallet Tree Structure

As this is a simple side project the eventual UI will only support the one-address-per-account structure. Example tree:

![](/docs/assets/current_tree_structure.png)

However the underlying rust library supports any HD wallet tree structure compliant to BIP44.

## Reference Specifications

[The original bip32 spec](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)

[The original bip39 spec](https://github.com/satoshilabs/slips/blob/master/slip-0039.md)

[The original bip44 spec](https://github.com/satoshilabs/slips/blob/master/slip-0044.md)

[SEC1 Cryptography](http://www.secg.org/sec1-v2.pdf)

## Some Helpful Documentation for the Learners Out There

[Trezor docs on BIP44](https://trezor.io/learn/a/what-is-bip44)

[Article on hardened key derivation](https://medium.com/@blainemalone01/hd-wallets-why-hardened-derivation-matters-89efcdc71671)

[Article on compressed vs uncompressed ethereum keys](https://medium.com/asecuritysite-when-bob-met-alice/02-03-or-04-so-what-are-compressed-and-uncompressed-public-keys-6abcb57efeb6)
