# Three-Stage Roadmap

**Status:** Active; stages are evidence gates, not calendar promises

## Stage 1 — Evidence and technical proof

- maintain a truthful open-source foundation, research archive, claim ledger, and requirements;
- validate market pain and a specific adoption wedge through interviews and primary evidence;
- build versioned conformance fixtures and a bounded fake HTTP provider;
- select the network stack through equivalent Hyper/Tokio and Pingora correctness spikes;
- prove streaming, cancellation, backpressure, recovery-budget, security, and resource contracts.

**Current state (2026-08-14):** repository governance and the network-neutral stream/retry domain
core exist. HTTP behavior, market demand, compatibility, and performance remain unproven.

**Exit:** R0 through R3 have named evidence. Otherwise narrow, pivot, contribute upstream, or stop.

## Stage 2 — Narrow public alpha

- one bounded OpenAI-compatible ingress subset and one compatible upstream;
- single-binary evaluation path without a mandatory database, broker, or hosted control plane;
- strict configuration, secrets, auth, policy, limits, deadlines, telemetry, and reload behavior;
- public compatibility/deviation matrix and reproducible benchmark data;
- checksums, SBOM, provenance, vulnerability intake, operations, and upgrade documentation.

**Exit:** an independent evaluator completes the synthetic streaming journey and the alpha passes
its named trust/technical profile. No universal compatibility or enterprise-readiness claim.

## Stage 3 — Validated adoption and hardening

- three design partners use the same primary workflow;
- pilot-driven workload identity, tenant policy, quotas, HA/drain, rollback, and deployment assets;
- incident, upgrade, security-response, conformance, and performance regression programs;
- multi-maintainer continuity and explicit open-source/commercial support boundaries.

**Exit:** evidence supports a clearly scoped production/enterprise candidate. Expansion such as
MCP/A2A, semantic caching, content guardrails, UI, or a hosted control plane needs a separate PRD.
