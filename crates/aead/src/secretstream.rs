//! Secretstream state transition tests.

use wycheproof_ng_core::*;

define_test_set!("Secretstream", "secretstream_test_schema.json");
define_test_set_names!(XChaCha20Poly1305 => "xchacha20_poly1305_secretstream");
define_algorithm_map!("XCHACHA20-POLY1305-SECRETSTREAM" => XChaCha20Poly1305);
define_test_flags!("Ktv" => KnownTestVector, NoPrematureRekey, CounterWrapRekey);
define_test_group_type_id!("SecretStreamTest" => SecretStream);

define_test_group!(operation: String);

/// Secretstream subkey, little-endian counter, and internal nonce.
#[derive(Debug, Clone, Hash, Eq, PartialEq, serde_derive::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub key: ByteString,
    pub counter: ByteString,
    pub inonce: ByteString,
}

define_test!(
    "initialState" => initial_state: State,
    tag: u8,
    aad: ByteString,
    "msg" => pt: ByteString,
    ct: ByteString,
    "expectedState" => expected_state: State,
);
