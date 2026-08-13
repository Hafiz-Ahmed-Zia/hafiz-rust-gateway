# Contributing

Hafiz Rust Gateway is in research and foundation. Contributions that improve evidence, conformance fixtures, failure semantics, security analysis, documentation, or small technical spikes are welcome once the public repository and security channel are active.

## Before opening a change

- Read `AGENTS.md` and `docs/README.md`.
- Search existing issues and decisions.
- Open a design discussion before large features, dependencies, public APIs, protocol promises, or architectural changes.
- Never use a public issue for a suspected undisclosed vulnerability.

## Pull requests

- Use one short-lived topic branch per coherent change.
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

## Current limitation

Build and test commands will be documented when the Rust workspace is created. Until then, documentation changes must at minimum pass link/path, Markdown, and internal-consistency checks.
