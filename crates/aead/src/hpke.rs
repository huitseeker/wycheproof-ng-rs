//! HPKE base-mode decryption tests. See the upstream `doc/hpke.md` for test semantics.

use wycheproof_ng_core::*;

define_test_set!("HPKE decryption", "hpke_decrypt_test_schema.json");
define_test_set_names!(
    MlKem512 => "hpke_mlkem512_decrypt",
    MlKem768 => "hpke_mlkem768_decrypt",
    MlKem1024 => "hpke_mlkem1024_decrypt",
    X25519 => "hpke_x25519_decrypt",
);
define_algorithm_map!("HPKE" => Hpke);
define_test_flags!(
    AppendedByteCiphertext,
    EmbeddedZeroAad,
    EmbeddedZeroInfo,
    EmbeddedZeroPlaintext,
    EmptyAad,
    EmptyCiphertext,
    EmptyInfo,
    EmptyPlaintext,
    EncTrailingByteInContext,
    ImplicitRejection,
    KemKdfDiffersFromHpkeKdf,
    LongInfo,
    LongPlaintext,
    MalleableCiphertext,
    MismatchedEncCt,
    ModifiedCiphertext,
    ModifiedEnc,
    ModifiedTag,
    NonCanonicalEncLargeOrder,
    Pseudorandom,
    RawEncInKemContext,
    RecommendedLimitInfo,
    SeedAllOnes,
    SeedAllZero,
    SmallOrderEnc,
    Strcmp,
    TagOnlyCiphertext,
    TrailingZeroAad,
    TrailingZeroInfo,
    TruncatedCiphertext,
    Twist,
    WrongAad,
    WrongInfo,
    WrongLengthEnc,
    WrongRecipientEnc,
);
define_test_group_type_id!("HpkeTest" => Hpke);

/// The HPKE key encapsulation mechanism.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, serde_derive::Deserialize)]
#[non_exhaustive]
pub enum Kem {
    #[serde(rename = "ML-KEM-512")]
    MlKem512,
    #[serde(rename = "ML-KEM-768")]
    MlKem768,
    #[serde(rename = "ML-KEM-1024")]
    MlKem1024,
    #[serde(rename = "DHKEM(X25519, HKDF-SHA256)")]
    X25519,
}

/// The HPKE key derivation function, which can differ from the KEM's KDF.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, serde_derive::Deserialize)]
#[non_exhaustive]
pub enum Kdf {
    #[serde(rename = "HKDF-SHA256")]
    HkdfSha256,
    #[serde(rename = "HKDF-SHA384")]
    HkdfSha384,
    #[serde(rename = "HKDF-SHA512")]
    HkdfSha512,
}

/// The HPKE authenticated encryption algorithm.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, serde_derive::Deserialize)]
#[non_exhaustive]
pub enum Aead {
    #[serde(rename = "AES-128-GCM")]
    Aes128Gcm,
    #[serde(rename = "AES-256-GCM")]
    Aes256Gcm,
    ChaCha20Poly1305,
}

define_test_group!(
    kem: Kem,
    "kemId" => kem_id: u16,
    kdf: Kdf,
    "kdfId" => kdf_id: u16,
    aead: Aead,
    "aeadId" => aead_id: u16,
);

define_test!(
    // ML-KEM private keys use the 64-byte d || z seed, rather than an expanded key.
    "skRm" => sk_rm: ByteString,
    "pkRm" => pk_rm: ByteString,
    // Optional inputs for deterministic sender-side reproduction.
    "ikmE" => ikm_e: Option<ByteString>,
    "skEm" => sk_em: Option<ByteString>,
    info: ByteString,
    aad: ByteString,
    enc: ByteString,
    ct: ByteString,
    pt: ByteString,
);
