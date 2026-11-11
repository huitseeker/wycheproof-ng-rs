# Changelog

## 0.4.0 (unreleased)

This release refreshes all bundled vectors to C2SP Wycheproof commit
`dab4ea2ed021eb633874959ca601004f3457d911`. It updates 11 existing files and
adds 16 files. Every one of the 359 bundled files has a public loader.

The AEAD crate adds HPKE base mode with X25519 or ML-KEM. The `hpke` module
covers decryption and `hpke_encap` covers recipient key validation. The
`hpke_sequence` module exposes the result and sequence number for each Open
call, including failures followed by retries. The `secretstream` module
covers state transitions.

The symmetric crate adds `stream_cipher` for ChaCha20 counter boundary tests.
The DH crate adds `eddsa_to_xdh` for public key conversion and `ristretto` for
scalar inversion. RSA OAEP loaders now expose the three bundled multi-prime
datasets that were previously excluded.

### API changes

`wycheproof_ng_eddsa::TestGroup::jwk` changes to `Option<EddsaPublicJwk>`.
Upstream groups without a JWK still provide the raw public key and its DER
and PEM encodings. Consumers can use these encodings or handle the missing
JWK explicitly.

`wycheproof_ng_core::RsaPrivate` gains `other_prime_infos`. Each entry stores
an additional prime with its exponent and CRT coefficient. Code constructing
this struct must supply the field. Use `Vec::new()` for two-prime keys.

These changes require a new compatibility line at version 0.4. All 15
published crates move together to 0.4.0. Rust 1.85 remains supported.

### CI changes

Upstream drift now fails CI on pull requests and scheduled runs. Both release
workflows check drift, so publication stops when the bundled vectors lag
behind upstream. The weekly issue workflow continues to notify maintainers.

A workspace test checks that every manifest entry has a public loader. This
prevents future refreshes from bundling new files without exposing them to
consumers. The test lives in the unpublished `xtask` crate because it reads
the repository manifest.
