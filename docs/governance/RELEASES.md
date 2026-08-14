# Release and Versioning Policy

**Status:** Accepted foundation policy

## Version authority

The workspace version in root `Cargo.toml` is the source of truth. Release tags use the exact form
`vMAJOR.MINOR.PATCH` with an optional SemVer pre-release suffix, for example
`v0.1.0-alpha.1`. A tag must point to the exact commit on `production` that passed release checks.

## Channels

- `alpha`: executable foundation or incomplete gateway behavior; evaluation only;
- `beta`: supported narrow gateway path with documented compatibility and upgrade limits;
- `rc`: release candidate that has passed the named acceptance profile;
- stable `v0.x.y`: still unstable under SemVer until the project explicitly adopts `v1.0.0`.

No tag or GitHub release means a branch snapshot, not an official release.

## Promotion

1. Topic branches merge into `development` after review and CI.
2. A promotion pull request moves an approved commit from `development` to `staging`.
3. Release checks run against `staging`; fixes return through `development`.
4. The same reviewed source commit is promoted to `production`.
5. The maintainer updates the changelog, verifies the workspace version, creates an annotated tag,
   pushes the tag, and publishes matching release notes.

The project does not rebuild different source for each branch. Future binary/container promotion
must reuse a verified artifact digest or produce traceable source-equivalent artifacts.

## Minimum release checks

- formatting, strict Clippy, tests, and RustSec audit pass;
- repository and raw-research integrity checks pass;
- no unresolved critical security finding in the released scope;
- changelog and known limitations match executable behavior;
- compatibility/performance claims link to versioned evidence;
- source tag and release notes clearly state whether artifacts are evaluation-only.

Signed binaries, checksums, SBOM, and provenance remain mandatory before any production-support
claim. The first foundation tag contains source only and must not imply a deployable HTTP gateway.
