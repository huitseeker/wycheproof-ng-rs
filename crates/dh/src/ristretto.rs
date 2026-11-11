//! Ristretto scalar inversion tests.

use wycheproof_ng_core::*;

define_test_set!("Ristretto scalar", "ristretto_scalar_test_schema.json");
define_test_set_names!(Ristretto255ScalarInvert => "ristretto255_scalar_invert");
define_algorithm_map!("RISTRETTO255" => Ristretto255);
define_test_flags!(ZeroInverse, "Ktv" => KnownTestVector, EdgeCase);
define_test_group_type_id!("RistrettoScalarTest" => RistrettoScalar);

define_test_group!(curve: String, operation: String);

define_test!(
    scalar: ByteString,
    "resultScalar" => result_scalar: Option<ByteString>,
);
