use wycheproof_ng_aead::hpke_sequence::{TestName, TestSet};

#[test]
fn preserves_failed_opens_and_retry_sequence_numbers() {
    let mut retried = false;
    let mut long_sequence = false;
    for name in TestName::all() {
        let set = TestSet::load(name).unwrap();
        for group in set.test_groups {
            for test in group.tests {
                let mut expected_sequence = 0;
                let mut previous_failed = false;
                for decryption in test.decryptions {
                    assert_eq!(decryption.sequence_number, expected_sequence);
                    long_sequence |= decryption.sequence_number >= 256;
                    if decryption.result.must_fail() {
                        assert!(decryption.pt.is_empty());
                        previous_failed = true;
                    } else {
                        retried |= previous_failed;
                        expected_sequence += 1;
                        previous_failed = false;
                    }
                }
            }
        }
    }
    assert!(retried, "missing successful retry after a failed Open");
    assert!(long_sequence, "missing sequence numbers above 255");
}
