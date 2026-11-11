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

Prepare version 0.4.0 across the existing crate family. This is a feature release
because it adds public loaders. A refresh limited to existing loaders could use
0.3.1. Keep Rust 1.85 support. Publication and tagging are outside this branch
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
outside these commits. No implementation changes have been made yet.
