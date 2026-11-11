#[test]
fn parses_all_eddsa_vectors() -> Result<(), wycheproof_ng_core::WycheproofError> {
    for test in wycheproof_ng_eddsa::TestName::all() {
        let _kat = wycheproof_ng_eddsa::TestSet::load(test)?;
    }

    Ok(())
}

#[test]
fn preserves_groups_without_a_jwk() {
    use wycheproof_ng_eddsa::{TestFlag, TestName, TestSet};

    let set = TestSet::load(TestName::Ed25519).unwrap();
    assert!(set.test_groups.iter().any(|group| group.jwk.is_some()));
    assert!(set.test_groups.iter().any(|group| {
        group.jwk.is_none()
            && group.tests.iter().any(|test| {
                test.flags.contains(&TestFlag::SmallOrderPublicKey)
                    || test.flags.contains(&TestFlag::MixedOrderPublicKey)
            })
    }));
}
