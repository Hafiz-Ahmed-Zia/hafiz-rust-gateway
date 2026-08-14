# Contributing

Hafiz Rust Gateway is in research and foundation. Contributions that improve evidence, conformance fixtures, failure semantics, security analysis, documentation, or small technical spikes are welcome once the public repository and security channel are active.

## Before opening a change

- Read `AGENTS.md` and `docs/README.md`.
- Search existing issues and decisions.
- Open a design discussion before large features, dependencies, public APIs, protocol promises, or architectural changes.
- Never use a public issue for a suspected undisclosed vulnerability.

## Pull requests

- Use one short-lived topic branch per coherent change.
- Open normal pull requests against `development`; only controlled promotion pull requests target
  `staging` or `production`.
- Link a requirement ID, issue, or research hypothesis.
- Explain what changed, why, risks, and validation.
- Include tests/evidence proportional to risk.
- Update docs, conformance, and changelog entries when user-visible behavior changes.
- Keep generated files reproducible and identify their generator/source.

## Commit and branch style

Examples:

- `feat/stream-cancellation`
- `fix/header-normalization`
- `docs/provider-matrix`
- `research/pingora-hyper-spike`

Use descriptive commits that each represent an isolated, reviewable change.

Do not include internal phase/step numbering in ordinary commit subjects. Versions belong in the
workspace manifest, changelog, annotated tags, and GitHub releases.

## Developer Certificate of Origin

Contributions use the [Developer Certificate of Origin 1.1](https://developercertificate.org/).
Sign each commit with `git commit --signoff` to certify that you have the right to submit it under
the repository license.

## Local quality gates

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo audit --deny warnings
```

`cargo audit` is a separate RustSec tool. CI installs the repository-pinned version. The current
workspace intentionally has no third-party runtime dependencies; every future dependency still
requires capability, maintenance, license, supply-chain, binary-size, and performance review.
