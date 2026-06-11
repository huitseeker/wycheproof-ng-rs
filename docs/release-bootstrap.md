# Release Bootstrap Checklist

This checklist records the first `0.1.0` release of the `wycheproof-ng` crate
family and the checks to run before routine GitHub Actions releases.

## Crates

The `0.1.0` bootstrap publish used this dependency order:

```text
wycheproof-ng-core
wycheproof-ng-aead
wycheproof-ng-symmetric
wycheproof-ng-fpe
wycheproof-ng-ecdsa
wycheproof-ng-dh
wycheproof-ng-dsa
wycheproof-ng-eddsa
wycheproof-ng-bls
wycheproof-ng-rsa-encryption
wycheproof-ng-rsa-signature
wycheproof-ng-mlkem
wycheproof-ng-mldsa
wycheproof-ng-kdf-jose
wycheproof-ng
```

## Release Gates

Work from a clean `main` checkout:

```bash
git switch main
git pull --ff-only huitseeker main
git status --short
```

Run the local release gates before a release:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- --deny warnings
cargo test --workspace --all-targets
cargo doc --workspace --no-deps
scripts/verify-wycheproof-data-offline.sh
scripts/verify-wycheproof-data.sh
scripts/verify-minimal-imports.sh
scripts/verify-package-sizes.sh
scripts/publish-workspace.sh --dry-run
```

Confirm each bootstrap crate version is present:

```bash
for crate in \
  wycheproof-ng-core \
  wycheproof-ng-aead \
  wycheproof-ng-symmetric \
  wycheproof-ng-fpe \
  wycheproof-ng-ecdsa \
  wycheproof-ng-dh \
  wycheproof-ng-dsa \
  wycheproof-ng-eddsa \
  wycheproof-ng-bls \
  wycheproof-ng-rsa-encryption \
  wycheproof-ng-rsa-signature \
  wycheproof-ng-mlkem \
  wycheproof-ng-mldsa \
  wycheproof-ng-kdf-jose \
  wycheproof-ng
do
  code="$(curl -sS -o /dev/null -w '%{http_code}' \
    -A wycheproof-ng-rs-bootstrap \
    "https://crates.io/api/v1/crates/${crate}/0.1.0")"
  printf '%s\t%s\n' "${crate}" "${code}"
done
```

Every line should end in `200`.

## Bootstrap Publish Record

The `0.1.0` bootstrap publish has been completed for all crates. Each crate has
a GitHub Actions trusted publisher configuration on crates.io with these claims:

| Field | Value |
|---|---|
| GitHub owner | `huitseeker` |
| GitHub repository | `wycheproof-ng-rs` |
| Workflow file | `release.yml` |
| Environment | `crates-io` |

The `release.yml` workflow uses the `crates-io` protected GitHub environment and
`rust-lang/crates-io-auth-action`, so these claims must continue to match
exactly.

## Proof Release

Run the manual release dry-run workflow from `main`:

```bash
gh workflow run "release dry run" \
  --repo huitseeker/wycheproof-ng-rs \
  --ref main
```

Approve the `release` environment gate and wait for the workflow to pass.

For the first trusted-publishing proof, bump every crate to the next patch
version in a pull request, tag that commit after it lands, create a GitHub
Release for the tag, and let `release.yml` publish from the protected
`crates-io` environment.
