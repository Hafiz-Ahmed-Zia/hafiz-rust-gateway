# Hafiz Rust Gateway

[![CI](https://github.com/Hafiz-Ahmed-Zia/hafiz-rust-gateway/actions/workflows/ci.yml/badge.svg?branch=development)](https://github.com/Hafiz-Ahmed-Zia/hafiz-rust-gateway/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Hafiz-Ahmed-Zia/hafiz-rust-gateway?include_prereleases)](https://github.com/Hafiz-Ahmed-Zia/hafiz-rust-gateway/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Hafiz Rust Gateway is a research-led open-source project for a secure-by-default, low-latency, self-hosted gateway for enterprise AI and LLM traffic.

> **Status: `v0.1.0-alpha.1` pre-alpha foundation.** The product thesis, architecture, and
> performance targets remain evidence-gated. A network-neutral Rust contract skeleton exists, but
> there is no HTTP gateway or production-ready runtime yet.

## Candidate product promise

**A privacy-first AI traffic boundary with verifiably correct streaming, predictable failure behavior, and a lightweight data plane that does not require a hosted control plane.**

This is a hypothesis to validate, not a marketing claim. The project will not treat "written in Rust," unified provider APIs, retries, fallbacks, cost tracking, or a dashboard as a moat; established gateways already offer those capabilities.

## Design principles

- **Secure by default:** fail closed on invalid policy, protect secrets, minimize collected data, and require explicit opt-in for prompt or response capture.
- **Streaming is a contract:** preserve event ordering, propagate cancellation and backpressure, bound buffers, and never silently replay or switch providers after visible output.
- **Lightweight is measurable:** publish reproducible latency, memory, CPU, startup, image-size, and per-stream resource results.
- **Operational simplicity:** support useful local evaluation with one binary and one configuration file; keep databases and distributed services optional.
- **Enterprise evidence:** make policy decisions, configuration changes, failures, and releases auditable without placing sensitive content in logs.
- **Protocol honesty:** publish conformance results and known incompatibilities instead of claiming universal compatibility.

## Documentation map

- [`docs/product/PRD.md`](docs/product/PRD.md) — approved foundation boundary with evidence-gated claims
- [`docs/research/03-independent-baseline-research.md`](docs/research/03-independent-baseline-research.md) — current independent findings
- [`docs/research/02-pro-model-master-research-prompt.md`](docs/research/02-pro-model-master-research-prompt.md) — prompt to run with the external Pro research model
- [`docs/research/04-pro-research-comparison-template.md`](docs/research/04-pro-research-comparison-template.md) — comparison method for the returned Pro report
- [`docs/architecture/ARCHITECTURE.md`](docs/architecture/ARCHITECTURE.md) — candidate technical architecture
- [`docs/architecture/STREAMING-CONTRACT.md`](docs/architecture/STREAMING-CONTRACT.md) — streaming and failure semantics
- [`docs/security/THREAT-MODEL.md`](docs/security/THREAT-MODEL.md) — preliminary threat model
- [`docs/benchmarks/BENCHMARK-SPEC.md`](docs/benchmarks/BENCHMARK-SPEC.md) — benchmark contract
- [`docs/roadmap/FOUNDATION-ROADMAP.md`](docs/roadmap/FOUNDATION-ROADMAP.md) — evidence-gated delivery sequence
- [`docs/roadmap/GITHUB-ISSUE-BACKLOG.md`](docs/roadmap/GITHUB-ISSUE-BACKLOG.md) — milestone and issue seed backlog
- [`docs/project/PROJECT-FACT-SHEET.md`](docs/project/PROJECT-FACT-SHEET.md) — truthful public project and maintainer snapshot
- [`docs/governance/RELEASES.md`](docs/governance/RELEASES.md) — version, promotion, and tag policy
- [`docs/README.md`](docs/README.md) — complete documentation index

## Current decisions

- The public working name is **Hafiz Rust Gateway**. Always use the full compound name until formal trademark clearance is complete.
- `development` is the default integration branch; reviewed source promotes one-way through
  `staging` to tagged `production` releases under ADR-0004.
- The first technical spike will compare a `hyper`/Tokio data plane with a Pingora-based implementation before the networking foundation is selected.
- The first release will be deliberately narrow. MCP, A2A, semantic caching, a dashboard, prompt management, and an evaluation platform are not day-one scope.

## Implemented foundation

- Cargo workspace with a network-neutral core, deterministic provider testkit, and research CLI.
- Executable failure taxonomy, streaming commitment state machine, and bounded retry/fallback policy.
- Built-in simulations for clean completion, pre-commit disconnect, and partial-stream disconnect.
- Apache-2.0 license, root security policy, contribution/governance files, and active GitHub CI/templates.
- Public maintainer ownership, protected promotion branches, private vulnerability reporting, and
  an evidence-gated release policy.

This foundation implements domain contracts only. It does **not** yet implement HTTP, TLS, provider
adapters, authentication, configuration loading, backpressure, real cancellation, or telemetry.

### Local verification

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo run -p hafiz-gateway -- simulate partial-disconnect
```

Expected final lines for the partial-disconnect scenario include:

```text
committed=true
partial=true
transparent_recovery_candidate=false
```

This demonstrates a domain contract only; it does not send network traffic.

## What happens next

1. Preserve complete external research reports and cited sources under `docs/research/external/`.
2. Resolve report conflicts against primary evidence and update the claim ledger.
3. Build the bounded fake HTTP provider and first versioned conformance fixtures.
4. Run the equivalent `hyper`/Tokio and Pingora correctness spikes required by ADR-0002.
5. Interview target platform, security, SRE, and FinOps teams while technical proof proceeds.
6. Pass the market-pain, wedge, trust, and technical gates before expanding provider scope.

## Community and maintenance

- Primary maintainer and responsibility map: [`MAINTAINERS.md`](MAINTAINERS.md)
- Contribution workflow: [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Questions and design discussion: [GitHub Discussions](https://github.com/Hafiz-Ahmed-Zia/hafiz-rust-gateway/discussions)
- Reproducible defects and bounded proposals: [GitHub Issues](https://github.com/Hafiz-Ahmed-Zia/hafiz-rust-gateway/issues)
- Vulnerabilities: follow [`SECURITY.md`](SECURITY.md) and use private vulnerability reporting

The repository was created on 2026-08-14 and currently has no verified adoption metrics. Stars,
downloads, users, performance, or compatibility will not be claimed without public evidence.

## Open-source status

Source code and documentation are licensed under Apache-2.0. The Hafiz Rust Gateway name and other
project branding are not granted as trademarks by that source-code license.
