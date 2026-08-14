# Changelog

All notable changes follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and semantic
versioning. This pre-`1.0` project makes compatibility promises only where its published matrix says
so.

## Unreleased

### Added

- Strict v1 JSON conformance fixtures with pre-parse and field-level bounds, executable outcome
  validation, and deterministic failure semantics.
- First fragmented SSE fixture and a credential-free, single-request loopback fake HTTP provider
  test.
- CLI fixture validation, dependency license/source policy, and a clearer first-screen project
  explanation in the README.

### Changed

- Clarified that squash-based promotion verifies an identical source-tree SHA rather than an
  identical branch commit SHA.

## [0.1.0-alpha.1] - 2026-08-14

### Added

- Three-crate Rust workspace for the network-neutral core, CLI, and deterministic testkit.
- Executable streaming commitment, failure taxonomy, and bounded recovery-policy contracts.
- Apache-2.0 license, maintainer/governance/security/contribution files, issue templates, CI,
  Dependabot, private vulnerability reporting, and secret scanning.
- Evidence-gated PRD, requirements, architecture, threat model, conformance plan, benchmark
  contract, research archive, project fact sheet, and three-stage roadmap.
- Controlled `development` -> `staging` -> `production` source promotion and SemVer release policy.

### Limitations

- Source-only pre-alpha release: no HTTP listener, TLS, provider adapter, authentication,
  configuration loader, real cancellation/backpressure, or production telemetry.
- No verified users, downloads, design partners, compatibility result, or performance result.

[0.1.0-alpha.1]: https://github.com/Hafiz-Ahmed-Zia/hafiz-rust-gateway/releases/tag/v0.1.0-alpha.1
