# Six-Month Open-Source Plan

**Status:** Evidence-gated working plan

**Start condition:** Broad Hafiz direction confirmed; market and technical gates still apply

## Month 1 — Executable contracts and repository quality

- compilable Rust workspace, CLI, core stream/retry state machines, deterministic testkit;
- Apache-2.0, security policy, contribution/governance files, CI and issue templates;
- preserve external research and complete the claim-level comparison;
- publish no performance or enterprise-readiness claims.

**Exit:** clean formatting, Clippy, tests and dependency audit; contract simulation is reproducible.

## Month 2 — Provider conformance foundation

- versioned fixture schema and synthetic OpenAI-style SSE fixtures;
- bounded fake HTTP provider with fragmentation, delay, malformed-event, and disconnect controls;
- machine-readable conformance report and compatibility matrix;
- first responsible upstream defect reports where validated.

**Exit:** at least 20 meaningful offline scenarios and three deterministic failure demonstrations.

## Month 3 — Network-stack shootout

- equivalent `hyper`/Tokio and Pingora implementations;
- H1/H2, streaming, cancellation, slow downstream, timeouts, reload and drain;
- identical auth/TLS/telemetry controls and raw benchmark evidence;
- accept or reject ADR-0002 with documented tradeoffs.

**Exit:** selected stack passes correctness gates; benchmark is reproducible.

## Month 4 — Narrow usable data plane

- one OpenAI-compatible ingress subset and one generic compatible upstream;
- strict configuration validation, secret references and local `check`/`doctor` path;
- bounded concurrency, request/event sizes, deadlines and pre-commit retry budget;
- metadata-only local telemetry.

**Exit:** evaluator completes a synthetic local streaming request without a database or hosted service.

## Month 5 — Security and operational hardening

- SSRF, header normalization/smuggling, parser, cancellation and overload tests;
- fuzz/property testing and fault-injection CI profile;
- atomic configuration snapshots, graceful drain and process-level tests;
- SBOM/provenance/release design dry run.

**Exit:** no unresolved critical trust-model flaw in the named evaluation profile.

## Month 6 — Public alpha evidence

- public compatibility/deviation matrix and benchmark raw data;
- signed checksums, SBOM, provenance and reproducible build instructions;
- Docker/systemd evaluation path and concise operator documentation;
- at least one independent integration or upstream fix driven by the conformance suite.

**Exit:** narrow public alpha. It must not claim universal compatibility or enterprise readiness.

## Scope held outside six months

- broad provider catalog;
- hosted control plane or analytics warehouse;
- semantic cache, RAG, prompt management or model evaluation;
- MCP/A2A gateway;
- enterprise UI and distributed global quotas;
- compliance certification claims.
