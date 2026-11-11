# Vector refresh for 0.4.0

The branch `fix/vector-refresh-ci` updates the bundled vectors from C2SP commit
`3fa63dd0344abb611f1fb1d77e119938603ea230` to
`dab4ea2ed021eb633874959ca601004f3457d911`. The new source changes 11 vector
files and adds 16 files, bringing the total to 359.

## Scope

Upstream drift must fail CI on pull requests and scheduled runs. Release
validation must check drift before publication. The issue workflow remains
responsible for notifying maintainers when upstream changes.

HPKE and Secretstream loaders belong in the AEAD crate. Stream cipher tests
belong in the symmetric crate. Edwards key conversion and Ristretto scalar
tests belong in the DH crate beside the existing curve tests. Each family
continues to depend only on the shared core crate.

Prepare version 0.4.0 across the existing crate family. The Ed25519 vectors now
include groups without a JWK, so `TestGroup::jwk` becomes optional. Multi-prime
RSA support adds `RsaPrivate::other_prime_infos` and exposes three previously
excluded OAEP datasets. These API changes require a new compatibility line.
Keep Rust 1.85 support. Publication and tagging are outside this branch
preparation task.

## Acceptance checks

- Every upstream vector file has a manifest entry and a public loader.
- Bundled bytes match the pinned source and the offline hash manifest.
- Parsing tests cover every loader, including nested HPKE decryption results.
- Workflow validation passes, and drift is no longer allowed to fail silently.
- Formatting and lint checks pass with the workspace tests and documentation.
- Minimal imports, package budgets, and publication dry runs pass.

## Progress

The initial checkout matches the fork's main branch at `0b1fde6ef`. Existing
untracked agent instructions and `docs/plans/` belong to the user and remain
outside these commits.

The workflow repair passes actionlint and zizmor. The repository rules require
pull requests but have no required status checks. The branch changes workflow
behavior without changing remote repository settings.

All 359 files match both the pinned upstream commit and the offline hashes.
The current upstream drift check passes. Parsing tests pass with a coverage
test that checks each manifest entry against the bytes exposed by public
loaders. Additional tests cover HPKE retries, Ed25519 groups without a JWK,
and multi-prime RSA key material. The coverage test lives in the unpublished
`xtask` crate so published crate tests do not depend on repository files.

## Validation results

All local checks below passed on November 11, 2026. The branch prepares all
15 published crates at 0.4.0 and leaves their Rust requirement at 1.85.

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed. |
| `cargo clippy --workspace --all-targets --locked -- --deny warnings` | Passed. |
| `cargo nextest run --workspace --all-targets --no-capture` | All 17 tests passed. |
| `cargo +1.85.0 test --workspace --all-targets --locked` | All 17 tests passed. |
| `cargo test --workspace --doc` | Passed; no doctests were defined. |
| `cargo doc --workspace --no-deps` | Passed. |
| `scripts/verify-wycheproof-data.sh` | All 359 files match the pinned source. |
| `scripts/verify-wycheproof-data-offline.sh` | All 359 files match the hash manifest. |
| `scripts/check-wycheproof-drift.sh` | Current upstream matches. |
| Drift check with `--upstream-ref 3fa63dd0344abb611f1fb1d77e119938603ea230` | Returned exit 20 as expected. |
| `scripts/verify-minimal-imports.sh` | All 13 family crates remain independent. |
| `scripts/verify-package-sizes.sh` | All 15 crates pass the 8 MiB budget. The largest is 4,969,173 bytes. |
| `scripts/publish-workspace.sh --dry-run` | All 15 packages verified without uploading. |
| `cargo deny check` | All policy checks passed. |
| `actionlint` and `zizmor --offline .github/workflows` | Passed with no active findings. |
| Prose checker | Changed Markdown and commit messages have no violations. |

The autoreview helper ran with `--mode branch` and `--base huitseeker/main`
using `--engine codex` against implementation head `4bb353eb8`. It returned
no findings. Tests were run separately from that review.

The branch has not been pushed, and hosted CI has not run for these commits.
No tag or release was created. User acceptance and issue closure remain
pending. Before merging, configure required status checks in the GitHub
repository rules if CI failures must prevent a merge.
