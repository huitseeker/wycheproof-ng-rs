//! HPKE ordered decryption tests on a single recipient context.
//!
//! Each decryption has its own expected result. Failed opens leave the sequence
//! number unchanged, so consumers must retain the listed order and retry state.

use crate::hpke::{Aead, Kdf, Kem};
use wycheproof_ng_core::*;

define_test_set!("HPKE sequence", "hpke_sequence_test_schema.json");
define_test_set_names!(
    MlKem512 => "hpke_mlkem512_sequence",
    MlKem768 => "hpke_mlkem768_sequence",
    MlKem1024 => "hpke_mlkem1024_sequence",
    X25519 => "hpke_x25519_sequence",
);
define_algorithm_map!("HPKE" => Hpke);
define_test_flags!(
    LongSequence,
    MultiMessage,
    OutOfOrderCiphertext,
    ReplayedCiphertext,
    RetryAfterFailedOpen,
);
define_test_group_type_id!("HpkeSequenceTest" => HpkeSequence);

define_test_group!(
    kem: Kem,
    "kemId" => kem_id: u16,
    kdf: Kdf,
    "kdfId" => kdf_id: u16,
    aead: Aead,
    "aeadId" => aead_id: u16,
);

/// One Open call against the recipient context, including its expected outcome.
#[derive(Debug, Clone, Hash, Eq, PartialEq, serde_derive::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decryption {
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: u64,
    pub aad: ByteString,
    pub ct: ByteString,
    pub pt: ByteString,
    pub result: TestResult,
}

define_test!(
    "skRm" => sk_rm: ByteString,
    "pkRm" => pk_rm: ByteString,
    "ikmE" => ikm_e: Option<ByteString>,
    "skEm" => sk_em: Option<ByteString>,
    info: ByteString,
    enc: ByteString,
    decryptions: Vec<Decryption>,
);
