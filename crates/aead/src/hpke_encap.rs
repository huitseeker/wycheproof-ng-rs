//! HPKE base-mode encapsulation and recipient key validation tests.

use crate::hpke::{Aead, Kdf, Kem};
use wycheproof_ng_core::*;

define_test_set!("HPKE encapsulation", "hpke_encap_test_schema.json");
define_test_set_names!(
    MlKem512 => "hpke_mlkem512_encap",
    MlKem768 => "hpke_mlkem768_encap",
    MlKem1024 => "hpke_mlkem1024_encap",
    X25519 => "hpke_x25519_encap",
);
define_algorithm_map!("HPKE" => Hpke);
define_test_flags!(
    LowOrderPublic,
    ModulusOverflow,
    NonCanonicalPkRmLargeOrder,
    NonCanonicalPublic,
    Pseudorandom,
    PublicKeyAllZero,
    SmallPublicKey,
    WrongLengthPublicKey,
    ZeroSharedSecret,
);
define_test_group_type_id!("HpkeEncapTest" => HpkeEncap);

define_test_group!(
    kem: Kem,
    "kemId" => kem_id: u16,
    kdf: Kdf,
    "kdfId" => kdf_id: u16,
    aead: Aead,
    "aeadId" => aead_id: u16,
);

define_test!(
    "pkRm" => pk_rm: ByteString,
    "ikmE" => ikm_e: Option<ByteString>,
    "skEm" => sk_em: Option<ByteString>,
    "skRm" => sk_rm: Option<ByteString>,
    info: ByteString,
    aad: ByteString,
    pt: ByteString,
    enc: ByteString,
    ct: ByteString,
);
