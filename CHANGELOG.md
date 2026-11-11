# Changelog

## 0.4.0 (unreleased)

Version 0.4.0 updates the vectors to C2SP commit
`dab4ea2ed021eb633874959ca601004f3457d911` and exposes all 359 bundled files.

New loaders cover HPKE and Secretstream. New vectors cover ChaCha20 counters
and Edwards key conversion, plus Ristretto scalar inversion. RSA OAEP now
supports keys with three primes.

The EdDSA `TestGroup::jwk` field is now optional. `RsaPrivate` adds
`other_prime_infos`. Set it to `Vec::new()` when constructing a key with two
primes.

CI and release checks now fail when bundled vectors differ from the latest
upstream vectors. A new test checks that every bundled file has a loader.

All crates move to 0.4.0. Rust 1.85 remains supported.
