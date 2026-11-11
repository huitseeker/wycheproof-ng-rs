//! Edwards to Montgomery public key conversion tests.

use wycheproof_ng_core::*;

define_test_set!("EdDSA to XDH", "eddsa_to_xdh_schema.json");
define_test_set_names!(Ed25519ToX25519 => "ed25519_to_x25519");
define_algorithm_map!("EDDSA-to-XDH" => EddsaToXdh);
define_test_flags!("Ktv" => KnownTestVector, SmallOrderPublicKey, MixedOrderPublicKey);
define_test_group_type_id!("EddsaToXdh" => EddsaToXdh);

define_test_group!(
    "edwardsCurve" => edwards_curve: EdwardsCurve,
    "montgomeryCurve" => montgomery_curve: MontgomeryCurve,
);

define_test!(
    "publicKey" => public_key: ByteString,
    "convertedPublicKey" => converted_public_key: Option<ByteString>,
);
