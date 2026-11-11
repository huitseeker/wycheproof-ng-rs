use std::collections::HashSet;
use std::path::Path;

#[test]
fn every_manifest_entry_has_a_public_loader() {
    let mut loaded = HashSet::new();

    macro_rules! collect {
        ($($module:path),+ $(,)?) => {
            $(
                {
                    use $module as vectors;
                    for name in vectors::TestName::all() {
                        assert!(loaded.insert(name.json_data()), "duplicate loader: {name:?}");
                    }
                }
            )+
        };
    }

    collect!(
        wycheproof_ng::aead::aead,
        wycheproof_ng::aead::chunked_encryption,
        wycheproof_ng::aead::daead,
        wycheproof_ng::aead::hpke,
        wycheproof_ng::aead::hpke_encap,
        wycheproof_ng::aead::hpke_sequence,
        wycheproof_ng::aead::secretstream,
        wycheproof_ng::bls,
        wycheproof_ng::dh::ec_curve,
        wycheproof_ng::dh::ecdh,
        wycheproof_ng::dh::eddsa_to_xdh,
        wycheproof_ng::dh::ristretto,
        wycheproof_ng::dh::xdh,
        wycheproof_ng::dsa,
        wycheproof_ng::ecdsa,
        wycheproof_ng::eddsa,
        wycheproof_ng::fpe::fpe_list,
        wycheproof_ng::fpe::fpe_str,
        wycheproof_ng::kdf_jose::hkdf,
        wycheproof_ng::kdf_jose::json_web,
        wycheproof_ng::kdf_jose::pbes2,
        wycheproof_ng::kdf_jose::pbkdf2,
        wycheproof_ng::kdf_jose::primality,
        wycheproof_ng::mldsa::mldsa_sign,
        wycheproof_ng::mldsa::mldsa_verify,
        wycheproof_ng::mlkem,
        wycheproof_ng::rsa_encryption::rsa_oaep,
        wycheproof_ng::rsa_encryption::rsa_pkcs1_decrypt,
        wycheproof_ng::rsa_signature::rsa_pkcs1_sig_gen,
        wycheproof_ng::rsa_signature::rsa_pkcs1_verify,
        wycheproof_ng::rsa_signature::rsa_pss_verify,
        wycheproof_ng::symmetric::cipher,
        wycheproof_ng::symmetric::keywrap,
        wycheproof_ng::symmetric::mac,
        wycheproof_ng::symmetric::mac_with_nonce,
        wycheproof_ng::symmetric::stream_cipher,
    );

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let manifest = std::fs::read_to_string(root.join("scripts/wycheproof-data-manifest.tsv"))
        .expect("read vector manifest");
    for line in manifest.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (file, dir) = line.split_once('\t').expect("manifest entry");
        let data = std::fs::read_to_string(root.join(dir).join(file)).expect("read vector file");
        assert!(loaded.remove(data.as_str()), "no public loader for {file}");
    }
    assert!(loaded.is_empty(), "public loaders absent from manifest");
}
