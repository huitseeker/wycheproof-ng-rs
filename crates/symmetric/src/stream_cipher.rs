//! Stream cipher counter boundary tests.

use wycheproof_ng_core::*;

define_test_set!("Stream cipher", "stream_cipher_test_schema.json");
define_test_set_names!(ChaCha20Counter => "chacha20_counter");
define_algorithm_map!("CHACHA20" => ChaCha20);
define_test_flags!(CounterCarry, CounterOverflow);
define_test_group_type_id!("StreamCipherTest" => StreamCipher);

define_test_group!(
    algorithm: String,
    "keySize" => key_size: usize,
    "ivSize" => nonce_size: usize,
    "counterSize" => counter_size: usize,
);

define_test!(
    key: ByteString,
    "iv" => nonce: ByteString,
    // Fixed-width counter encoded in little-endian order upstream.
    "initialCounter" => initial_counter: ByteString,
    "msg" => pt: ByteString,
    ct: Option<ByteString>,
);
