#[test]
fn parses_all_rsa_encryption_vectors() -> Result<(), wycheproof_ng_core::WycheproofError> {
    for test in wycheproof_ng_rsa_encryption::rsa_oaep::TestName::all() {
        let _kat = wycheproof_ng_rsa_encryption::rsa_oaep::TestSet::load(test)?;
    }

    for test in wycheproof_ng_rsa_encryption::rsa_pkcs1_decrypt::TestName::all() {
        let _kat = wycheproof_ng_rsa_encryption::rsa_pkcs1_decrypt::TestSet::load(test)?;
    }

    Ok(())
}

#[test]
fn preserves_additional_rsa_primes() {
    use wycheproof_ng_rsa_encryption::rsa_oaep::{TestName, TestSet};

    for name in [
        TestName::RsaThreePrimes2048Sha1Mgf1Sha1,
        TestName::RsaThreePrimes3072Sha224Mgf1Sha224,
        TestName::RsaThreePrimes4096Sha256Mgf1Sha256,
    ] {
        let set = TestSet::load(name).unwrap();
        for group in set.test_groups {
            assert_eq!(group.key.other_prime_infos.len(), 1);
            assert!(group.key.other_prime_infos[0]
                .iter()
                .all(|value| !value.is_empty()));
        }
    }
    let set = TestSet::load(TestName::Rsa2048Sha1Mgf1Sha1).unwrap();
    assert!(set
        .test_groups
        .iter()
        .all(|group| group.key.other_prime_infos.is_empty()));
}
