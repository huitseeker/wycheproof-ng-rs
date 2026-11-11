#[test]
fn parses_all_aead_vectors() -> Result<(), wycheproof_ng_core::WycheproofError> {
    for test in wycheproof_ng_aead::aead::TestName::all() {
        let _kat = wycheproof_ng_aead::aead::TestSet::load(test)?;
    }

    for test in wycheproof_ng_aead::chunked_encryption::TestName::all() {
        let _kat = wycheproof_ng_aead::chunked_encryption::TestSet::load(test)?;
    }

    for test in wycheproof_ng_aead::daead::TestName::all() {
        let _kat = wycheproof_ng_aead::daead::TestSet::load(test)?;
    }

    for test in wycheproof_ng_aead::hpke::TestName::all() {
        let _kat = wycheproof_ng_aead::hpke::TestSet::load(test)?;
    }

    for test in wycheproof_ng_aead::hpke_encap::TestName::all() {
        let _kat = wycheproof_ng_aead::hpke_encap::TestSet::load(test)?;
    }

    for test in wycheproof_ng_aead::hpke_sequence::TestName::all() {
        let _kat = wycheproof_ng_aead::hpke_sequence::TestSet::load(test)?;
    }

    for test in wycheproof_ng_aead::secretstream::TestName::all() {
        let _kat = wycheproof_ng_aead::secretstream::TestSet::load(test)?;
    }

    Ok(())
}
