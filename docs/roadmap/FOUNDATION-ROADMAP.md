# Foundation Roadmap

**Status:** Active; gates are outcomes, not calendar promises

## F0 — Foundation

- project naming and preliminary clearance;
- research plan, Pro prompt, independent baseline, claim ledger;
- evidence-gated PRD and requirements;
- architecture, streaming, reliability, security, benchmark, and conformance contracts;
- local Git history and GitHub Flow decision.

**Exit:** documentation is internally consistent and external research can be compared.

## F1 — Market and wedge validation

- run Pro research and resolve claims;
- 15+ interviews across selected segments;
- incumbent adoption/rejection studies;
- three design-partner candidates;
- select, narrow, or reject primary wedge.

**Exit:** R0 and R1 pass. Otherwise pivot/contribute/stop.

## F2 — Technical proof

- network-stack comparison;
- canonical request/event model;
- fake-provider and conformance harness;
- streaming/cancellation/backpressure state machine;
- auth, secure config, metadata-only telemetry;
- reproducible equivalent-control benchmarks;
- targeted fuzz/fault tests.

**Current slice (2026-08-14):** the network-neutral stream/retry domain core and deterministic
scenario testkit are implemented. HTTP behavior, real cancellation/backpressure, configuration,
provider conformance, and performance remain unproven.

**Exit:** R2 and R3 pass.

## F3 — Minimum credible open-source release

- narrow supported compatibility matrix;
- single-binary/container evaluation path;
- signed checksums/artifacts, SBOM and provenance;
- active root license and security policy;
- CI, branch protection, issue/discussion/security channels;
- migration, operations, failure, and upgrade documentation.

**Exit:** public alpha; no enterprise-grade claim.

## F4 — Design-partner pilot

- three teams run the same primary use case;
- operational and privacy evidence;
- incident/upgrade drills;
- SLOs and support boundaries;
- close critical/high findings.

**Exit:** R4 and pilot exit criteria pass.

## F5 — Enterprise candidate

- workload/OIDC identity as validated;
- tenant policy and quota semantics;
- HA/drain/rollback and deployment evidence;
- release/security response maturity;
- performance and conformance regression program;
- commercial/support model only after OSS trust boundary is clear.

**Exit:** explicit evidence permits enterprise-ready claim for named deployment profiles.
