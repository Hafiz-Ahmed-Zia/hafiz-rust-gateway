# Candidate Architecture

**Status:** Proposed for technical spikes; not selected  
**Principle:** Keep the mandatory data plane small and make enterprise integrations optional.

## 1. System boundary

Hafiz Rust Gateway sits between trusted or semi-trusted client applications and external or self-hosted model providers.

```text
Client application
      |
      | authenticated HTTP/SSE
      v
+-------------------- Hafiz Rust Gateway data plane --------------------+
| listener -> identity -> limits -> policy -> route -> adapter -> egress |
|                       |                    |                            |
|                       +-> audit metadata   +-> health/circuit state    |
|                       +-> metrics/traces (bounded, asynchronous)       |
+-----------------------------------------------------------------------+
      |
      | verified TLS / provider-native protocol
      v
Model provider or self-hosted inference endpoint
```

The data plane is the product core. A future control plane can distribute signed/versioned configuration, but the data plane must continue safely on a last-known-good snapshot without a live control-plane connection.

## 2. Mandatory data-plane components

### Listener and transport

- HTTP/1.1 and HTTP/2 where supported by the selected stack
- TLS termination optional for trusted sidecar/private deployments, required for exposed deployments
- strict header/body/time limits
- connection lifecycle, drain, and cancellation propagation

### Identity

- inbound credential extraction and validation
- caller, tenant, and project attribution kept distinct
- immutable identity context attached to the request

### Policy compiler and evaluator

- parse and validate configuration off the request path
- compile rules into immutable snapshots
- atomic snapshot activation
- request pins a snapshot version for its lifetime
- explain/dry-run tooling separate from normal traffic

### Router and resilience

- deterministic target selection
- per-target health and circuit state
- bounded retry/fallback plan calculated before execution
- attempt and end-to-end deadlines
- no mid-stream provider switching after downstream-visible bytes

### Provider adapters

- explicit canonical request/response model for normalized routes
- pass-through mode when normalization would destroy semantics
- provider error classification
- usage metadata parsing with source attribution
- streaming event parser/encoder governed by the streaming contract

### Telemetry and audit

- metadata-only by default
- bounded non-blocking export
- local counters remain available if remote export fails
- separate operational telemetry from audit evidence
- dropping optional telemetry must be observable and must not block traffic

## 3. Optional components

- external secrets providers
- OIDC/JWKS cache
- distributed quota/rate-limit backend
- remote configuration source
- SIEM/OTel collector
- persistent audit sink
- Kubernetes service discovery
- control-plane UI

No optional component may become an undocumented mandatory dependency.

## 4. Candidate Rust module boundaries

```text
crates/
  gateway-bin/          process lifecycle and CLI
  gateway-core/         request context and orchestration interfaces
  gateway-http/         listener, transport, headers, bodies, SSE
  gateway-auth/         inbound identities and upstream credentials
  gateway-policy/       schema, validation, compilation, evaluation
  gateway-routing/      targets, health, circuits, retry plans
  gateway-providers/    adapter traits and provider implementations
  gateway-telemetry/    metrics, traces, audit metadata, redaction
  gateway-config/       loading, snapshots, reload, provenance
  gateway-testkit/      fake providers, fixtures, fault injection
```

This is a modularity proposal, not permission to create every crate immediately. Phase 0 should use the smallest structure that can test boundaries without premature fragmentation.

## 5. Network-stack decision

Two implementations must be spiked:

### Option A — Tokio + hyper + focused Tower layers

Strengths:

- low-level control over request/response bodies and streaming translation;
- direct fit for protocol-aware application behavior;
- mature HTTP/1 and HTTP/2 building blocks;
- easier to keep dependencies and callbacks explicit.

Risks:

- more proxy correctness and lifecycle behavior must be designed by the project;
- connection pooling, retries, drain, and production hardening require careful work.

### Option B — Pingora proxy framework

Strengths:

- production-proven proxy primitives, connection reuse, load balancing, failover, and graceful restart patterns;
- programmable request filters;
- strong performance and memory-safety motivation.

Risks:

- LLM-aware JSON transformation and event-stream semantics may fight a generic proxy abstraction;
- TLS/provider choices and async/runtime integration may increase build complexity;
- framework behavior must be verified for client cancellation and backpressure at the exact hooks needed.

### Selection evidence

Use equivalent auth, TLS, streaming, telemetry, and limits. Compare correctness first, then latency, CPU, memory, dependency risk, implementation complexity, and operability. Select neither if a smaller alternative proves better.

## 6. Configuration model

- declarative versioned schema;
- deny unknown security-relevant fields;
- secret references, never embedded examples with real credentials;
- immutable compiled snapshot containing route, policy, limits, and provider metadata;
- last-known-good snapshot retained;
- reload parses, validates, resolves required references, compiles, and only then swaps;
- in-flight requests retain their original snapshot;
- every activation has hash, version, source, actor where known, timestamp, and result.

## 7. State model

### Local state allowed in v1

- connection pools;
- per-target health/circuit windows;
- bounded local rate/concurrency counters;
- configuration snapshots;
- ephemeral credential/JWKS cache;
- telemetry buffers.

### Persistent state excluded from the core hot path

- prompt and response bodies;
- user conversation state;
- semantic cache;
- analytics database;
- centralized billing ledger.

Distributed enforcement semantics must state whether they are strong, bounded-stale, or best-effort. A local limit must not be marketed as a globally consistent quota.

## 8. Extension strategy

Do not load arbitrary native plugins into the gateway process in early releases. Extension approaches are evaluated in this order:

1. built-in, reviewed policy primitives;
2. out-of-process gRPC/HTTP checks with strict timeouts and fail-open/fail-closed policy;
3. sandboxed WASM with explicit capability, memory, CPU, and network limits;
4. native plugins only if the security and ABI cost is justified.

## 9. Deployment modes

- **Local/dev:** loopback listener, one configuration, local metrics.
- **Sidecar:** one application or pod; simple isolation and attribution.
- **Shared regional gateway:** multiple applications/tenants; HA and distributed limits optional.
- **Hybrid/on-prem:** no hosted control plane required; outbound provider access explicitly controlled.

The binary remains the same where practical; behavior changes through validated configuration and compiled feature profiles.

## 10. Architectural invariants

1. No secret or content enters default telemetry.
2. No unbounded request, stream, retry, or telemetry queue.
3. No policy/config activation before complete validation.
4. No silent upstream switch after downstream-visible output.
5. No request depends synchronously on optional telemetry or a hosted control plane.
6. No compatibility claim without fixtures and a published tier.
7. No global consistency claim for locally enforced state.
8. No security/compliance claim based only on the Rust language or a dependency choice.
