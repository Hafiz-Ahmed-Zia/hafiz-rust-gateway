# Hafiz Rust Gateway

Hafiz Rust Gateway is a research-led open-source project for a secure-by-default, low-latency, self-hosted gateway for enterprise AI and LLM traffic.

> **Status: discovery and foundation.** The product thesis, architecture, performance targets, and licensing recommendation are drafts until their evidence gates are passed. There is no production release yet.

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

- [`docs/product/PRD.md`](docs/product/PRD.md) — evidence-gated product requirements draft
- [`docs/research/03-independent-baseline-research.md`](docs/research/03-independent-baseline-research.md) — current independent findings
- [`docs/research/02-pro-model-master-research-prompt.md`](docs/research/02-pro-model-master-research-prompt.md) — prompt to run with the external Pro research model
- [`docs/research/04-pro-research-comparison-template.md`](docs/research/04-pro-research-comparison-template.md) — comparison method for the returned Pro report
- [`docs/architecture/ARCHITECTURE.md`](docs/architecture/ARCHITECTURE.md) — candidate technical architecture
- [`docs/architecture/STREAMING-CONTRACT.md`](docs/architecture/STREAMING-CONTRACT.md) — streaming and failure semantics
- [`docs/security/THREAT-MODEL.md`](docs/security/THREAT-MODEL.md) — preliminary threat model
- [`docs/benchmarks/BENCHMARK-SPEC.md`](docs/benchmarks/BENCHMARK-SPEC.md) — benchmark contract
- [`docs/roadmap/FOUNDATION-ROADMAP.md`](docs/roadmap/FOUNDATION-ROADMAP.md) — evidence-gated delivery sequence
- [`docs/README.md`](docs/README.md) — complete documentation index

## Current decisions

- The public working name is **Hafiz Rust Gateway**. Always use the full compound name until formal trademark clearance is complete.
- The recommended Git model is one protected `main` branch plus short-lived topic branches and release tags. Environment branches named `development`, `staging`, and `production` are intentionally not used.
- The first technical spike will compare a `hyper`/Tokio data plane with a Pingora-based implementation before the networking foundation is selected.
- The first release will be deliberately narrow. MCP, A2A, semantic caching, a dashboard, prompt management, and an evaluation platform are not day-one scope.

## What happens next

1. Run the Pro-model research prompt without shortening it.
2. Add the complete returned report and sources under `docs/research/external/`.
3. Compare the report with the independent baseline, resolve conflicting claims, and update the claim ledger.
4. Interview target platform, security, SRE, and FinOps teams.
5. Pass the market-pain and wedge gates before treating the PRD as approved.
6. Build a thin conformance and streaming spike before expanding product scope.

## Open-source status

The repository foundation is being prepared for public open-source development. The recommended license is Apache-2.0 because infrastructure adopters benefit from an explicit patent grant, but the owner must approve the final license before a public release.
