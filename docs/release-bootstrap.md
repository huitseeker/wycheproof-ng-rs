# Release Bootstrap Checklist

This checklist is for the first `0.1.0` release of the `wycheproof-ng` crate
family. Use it before enabling routine GitHub Actions releases.

Do not run the `release.yml` publish workflow until every crate has been
published once and has a matching crates.io trusted publisher configuration.

## Crates

Publish and configure trusted publishing in this order:

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

## Before Publishing

Work from a clean `main` checkout:

```bash
git switch main
git pull --ff-only huitseeker main
git status --short
```

Run the local release gates:

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

Confirm no `0.1.0` crate is already present:

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

Every line should end in `404` before the first bootstrap publish.

## Bootstrap Publish

Create a short-lived local crates.io API token with publish rights for the new
crate family, then export it only for the current shell:

```bash
export CARGO_REGISTRY_TOKEN=...
```

Publish in dependency order:

```bash
scripts/publish-workspace.sh --publish
```

The script is resumable. If a crate version is already visible on crates.io, it
skips that crate. If crates.io returns anything other than `200` or `404` while
checking a version, it stops before publishing.

Delete the local crates.io token immediately after the bootstrap publish:

```bash
unset CARGO_REGISTRY_TOKEN
```

Then revoke the token in the crates.io web UI.

## Trusted Publishing Setup

For each crate, add a GitHub Actions trusted publisher configuration on
crates.io with these claims:

| Field | Value |
|---|---|
| GitHub owner | `huitseeker` |
| GitHub repository | `wycheproof-ng-rs` |
| Workflow file | `release.yml` |
| Environment | `crates-io` |

This must be repeated for all 15 crates. The `release.yml` workflow uses the
`crates-io` protected GitHub environment and
`rust-lang/crates-io-auth-action`, so these claims must match exactly.

## After Trusted Publishing

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
