# Hafiz Rust Gateway — Product Requirements Document

**Version:** 0.1  
**Status:** Evidence-gated draft; not approved for implementation  
**Date:** 2026-08-13  
**Owner:** Project founder  
**Primary decision gates:** R0 market pain, R1 wedge, R2 trust, R3 technical proof

## 1. Executive summary

Hafiz Rust Gateway is intended to be an open-source, self-hosted AI traffic gateway for teams that need a small operational footprint without giving up enterprise-grade security, streaming correctness, reliability, governance, or observability.

The candidate adoption wedge is:

> A privacy-first, single-binary AI data plane that makes streaming and failure behavior verifiable and predictable under provider errors, cancellations, overload, policy changes, and shutdowns.

This wedge is not validated. Rust, raw speed, provider normalization, retries, fallbacks, rate limits, budgets, logging, and dashboards already exist in competing products. The project proceeds only if interviews and technical spikes show that the combined correctness, privacy, and operational model solves an expensive problem better than adopting or extending an incumbent.

## 2. Problem

Organizations operating multiple AI applications face a fragmented traffic boundary:

- provider APIs and streaming event models differ;
- credentials and model permissions spread across applications;
- provider rate limits, transient failures, and partial streams create inconsistent application behavior;
- prompt and response logging can leak sensitive data;
- token usage, cost, and accountability are difficult to normalize;
- large gateway/control-plane stacks can be disproportionate for self-hosted or hybrid environments;
- generic retry and failover behavior can duplicate side effects or corrupt visible streamed output;
- teams cannot trust broad compatibility or performance claims without conformance evidence.

The product must solve these problems without becoming a mandatory hosted dependency, an opaque data collector, or a large platform that is harder to operate than the applications it protects.

## 3. Target users and buying group

### Primary operator

Platform or AI-platform engineer responsible for a shared model-access layer across multiple applications or teams.

### Primary economic buyer hypothesis

Head of Platform, VP Engineering, or infrastructure leader who owns reliability, cloud/model spend, and developer enablement.

### Required approvers

- Security engineering and application security
- Privacy, data governance, or compliance
- SRE/operations
- Procurement/legal for enterprise deployment

### Secondary users

- Application developers consuming an OpenAI-compatible endpoint
- FinOps teams allocating token and provider spend
- Incident responders investigating AI traffic failures
- Regulated-environment architects evaluating self-hosted controls

## 4. Jobs to be done

1. **When a team adds or changes an AI provider,** let it preserve the application contract and make incompatibilities explicit.
2. **When AI traffic becomes production-critical,** enforce authentication, model policy, quotas, timeouts, and safe failure behavior centrally.
3. **When a provider degrades,** protect applications with bounded, explainable retries, routing, and circuit breaking without duplicating visible output.
4. **When security reviews AI adoption,** demonstrate what data crosses each boundary, what is logged, and which controls fail closed.
5. **When an incident occurs,** reconstruct routing and policy decisions without retaining prompt or response content by default.
6. **When operators evaluate a gateway,** run it locally with minimal dependencies and obtain truthful resource and compatibility evidence.

## 5. Product principles

### 5.1 Secure by default

Unsafe behavior must require an explicit, reviewable opt-in. Invalid configuration, ambiguous identity, missing upstream credentials, and unverifiable policy must fail closed.

### 5.2 Streaming correctness over clever recovery

Once bytes from an upstream response are visible to a client, v1 must not silently restart the request or switch providers. A terminated stream is reported honestly with correlated diagnostic metadata.

### 5.3 The hot path stays small

The request path must not require a database, Redis, message broker, hosted control plane, or synchronous telemetry backend for core operation.

### 5.4 Content privacy

Prompt, response, tool arguments, tool results, authorization headers, and upstream credentials are excluded from telemetry by default. Metadata collection must be documented and bounded.

### 5.5 Compatibility is tested, not advertised

Every supported endpoint/provider pair receives a published conformance tier and known-deviation list.

### 5.6 Operational truth

Benchmarks include configuration, hardware, workload, raw results, and failure rates. Comparisons use equivalent security and telemetry settings.

## 6. Goals and success measures

| Goal | Initial success measure | Gate |
| --- | --- | --- |
| Validate painful demand | At least 15 interviews; at least 5 report the same high-cost problem; at least 3 agree to design-partner testing | R0 |
| Establish an adoption wedge | 3 design partners prefer the candidate wedge over extending an incumbent and can explain why | R1 |
| Establish trust | Threat model, secure-default tests, privacy map, failure matrix, signed-release plan, and security review have no unresolved critical gap | R2 |
| Prove protocol fidelity | Required conformance fixtures pass for each advertised compatibility tier | R3 |
| Prove streaming behavior | Cancellation, backpressure, partial-event, timeout, disconnect, shutdown, and overload tests pass without unbounded buffering or silent replay | R3 |
| Prove lightweight operation | Reference benchmark meets approved added-latency, memory, CPU, startup, and artifact targets | R3 |
| Prove adoption simplicity | New evaluator reaches a working local proxy and a useful diagnostic result in 10 minutes or less from released artifacts | R3/R4 |

## 7. Non-goals for the first public milestone

- Building a general API-management platform
- Supporting every AI provider or modality
- An evaluation/experimentation platform
- Prompt lifecycle management
- A hosted analytics service
- Semantic caching
- A visual control-plane dashboard
- MCP gateway, A2A gateway, RAG engine, or agent runtime
- Mid-stream cross-provider continuation disguised as the original stream
- Storing prompts/responses by default
- Claiming FIPS, SOC 2, HIPAA, PCI DSS, FedRAMP, or regulatory compliance without the corresponding validated system and process

## 8. Scope by phase

### Phase 0 — Research and technical proof

- Interview and evidence program
- `hyper`/Tokio versus Pingora spike
- OpenAI-compatible request and SSE streaming proxy spike
- One native provider plus one generic OpenAI-compatible upstream
- Streaming state-machine tests
- Protocol fixture harness
- Reproducible benchmark harness
- Preliminary threat model and secure-default tests

### Phase 1 — Minimum credible gateway

- Single static or near-static binary where platform permits
- Configuration file plus environment/file secret references
- `check`, `run`, `version`, and `doctor` CLI commands
- Inbound bearer-token authentication with hashed-at-rest credential records
- Provider/model allow policies
- Request-size, concurrency, and time limits
- Priority and weighted routing
- Pre-response retries and fallbacks with budgets
- Circuit breaking and health state
- OpenAI Chat Completions compatibility
- OpenAI Responses compatibility subset defined by fixtures
- Streaming SSE passthrough/translation for advertised providers
- OpenTelemetry-compatible metrics and traces with content capture off
- Structured audit metadata for authentication, policy, routing, and configuration decisions
- Prometheus endpoint optional and local by default
- Atomic configuration validation and reload

### Phase 2 — Enterprise pilot

- OIDC/JWT validation with issuer/audience pinning
- Optional mTLS and workload identity integration
- Tenant/project identity and quotas
- Per-team/model/provider policy bundles
- Distributed rate-limit adapter with standalone local default
- High-availability deployment and graceful drain
- Configuration provenance, signatures, and rollback
- SIEM/OTel export with bounded asynchronous delivery
- Multi-region and data-residency policy primitives
- Kubernetes manifests/Helm only after the standalone path is stable
- Signed artifacts, SBOM, provenance, vulnerability reporting, and release channel policy

### Phase 3 — Validated expansion only

MCP/A2A governance, semantic caching, content guardrails, UI, hosted control plane, advanced cost routing, and policy plugins enter scope only through separate PRDs backed by customer evidence and performance/security analysis.

## 9. Functional requirements

### 9.1 Ingress and identity

- Authenticate every non-health request unless a route is explicitly declared public.
- Reject ambiguous or conflicting identity material.
- Support credential rotation without process restart.
- Separate caller identity from billing/project attribution.
- Never forward inbound gateway credentials upstream.

### 9.2 Provider credentials

- Accept secrets through environment variables, mounted files, and a provider interface for external secret managers.
- Redact secret-bearing configuration from diagnostics.
- Keep secrets out of error messages, metrics, traces, panic output, and configuration diffs.
- Reload rotated secrets atomically where supported.

### 9.3 Routing

- Resolve route from authenticated identity, requested model alias, policy, health, and capacity.
- Record a non-sensitive reason code for the chosen target.
- Support deterministic priority, weighted, and least-outstanding strategies in the first credible release.
- Make cost/latency-aware or semantic routing later opt-in features with explainability and stability bounds.

### 9.4 Retries and fallback

- Retry only failures classified as retryable.
- Enforce per-attempt and end-to-end deadlines plus a retry budget.
- Do not retry client validation/auth failures.
- Do not retry after downstream-visible response bytes unless a protocol-specific contract explicitly proves safety.
- Expose attempts and final disposition in metadata without leaking content.

### 9.5 Streaming

- Preserve event order and event boundaries required by the advertised API contract.
- Bound every queue and buffer.
- Propagate client cancellation to the upstream promptly.
- Respect downstream backpressure instead of accumulating an unbounded response.
- Distinguish clean completion, client cancellation, upstream timeout, malformed event, policy termination, overload, and transport failure.
- Drain or terminate active streams predictably during shutdown.

### 9.6 Policy

- Compile and validate policy before activation.
- Apply configuration snapshots atomically to new requests.
- Pin the active policy snapshot for the lifetime of an in-flight request.
- Provide dry-run and explain modes that do not expose secrets.
- Log policy identifiers, versions, and outcomes; do not log raw prompts by default.

### 9.7 Limits and overload

- Enforce request-body byte limits before full materialization where possible.
- Enforce connection, request, stream, concurrency, and queue limits.
- Reject excess load quickly with stable error codes and `Retry-After` when meaningful.
- Prevent one tenant/model/provider from exhausting global capacity.
- Prefer shedding optional telemetry over blocking or crashing the data plane.

### 9.8 Observability and audit

- Emit request ID, trace context, tenant/project identifiers, route, model alias, upstream target ID, attempt count, timing phases, token counts when authoritative, byte counts, terminal status, and policy outcome.
- Treat provider-reported and gateway-estimated tokens as different fields.
- Export using OpenTelemetry conventions where stable; isolate experimental attributes.
- Disable prompt/response/tool content capture by default.
- Provide sampling and redaction controls before any content opt-in.

### 9.9 Configuration and diagnostics

- Reject unknown security-relevant fields by default.
- Print actionable path-specific validation errors.
- `doctor` must test DNS, TLS trust, credentials without disclosure, upstream reachability, and optional telemetry sinks.
- `check` must work offline for schema and policy validation.
- Every configuration option must document default, security impact, reload behavior, and resource impact.

## 10. Quality attributes

### Performance targets for the Phase 0 reference workload

These are engineering targets to validate, not current claims:

- Added gateway latency: p50 <= 250 microseconds and p99 <= 1 millisecond for non-transforming local proxy traffic at the approved concurrency point.
- Streaming time-to-first-forwarded-byte overhead: p99 <= 1 millisecond after the first complete upstream event is available.
- Idle resident memory: <= 50 MiB for the minimal build and configuration.
- Incremental gateway memory: <= 128 KiB per steady-state text stream at the reference event size and backpressure profile.
- Cold start to readiness: <= 100 milliseconds on the reference Linux environment, excluding external checks.
- No unbounded queues; configured limits must be observable.

Targets must be re-baselined if protocol translation, TLS termination, auth, policy, and required telemetry cannot meet them together. Results without production-equivalent controls do not pass.

### Reliability targets for enterprise pilot

- No process crash or cross-request data exposure under malformed-input, cancellation, or overload suites.
- Atomic config activation with rollback to last-known-good state.
- Graceful termination rejects new work, drains within policy, and reports forced terminations.
- At-least-once audit export must never be misrepresented as exactly once.
- Gateway availability target and recovery objective are set only after deployment topology is selected.

## 11. Security and privacy requirements

- Threat model against external callers, malicious tenants, compromised upstreams, insiders, dependency compromise, configuration attackers, and telemetry exfiltration.
- Least privilege for file, network, and runtime access.
- TLS verification on by default; insecure TLS requires an explicit development-only override and loud diagnostic.
- Modern TLS through a reviewed provider; FIPS mode is a separate build/deployment claim.
- Deny private/link-local/metadata destinations for untrusted dynamic upstream configuration unless explicitly allowed.
- Bound decompression, parsing depth, JSON size, headers, event size, and token-estimation work.
- Protect against request smuggling and hop-by-hop header confusion.
- Signed release artifacts, SBOM, provenance, dependency review, and vulnerability intake before public production claims.
- Map relevant controls to OWASP GenAI risks and NIST AI RMF guidance without claiming certification.

## 12. Compatibility promise

The project will publish four tiers:

- **Native:** maintained provider adapter with differential and live conformance tests.
- **Compatible:** OpenAI-compatible upstream with a tested subset.
- **Pass-through:** minimally interpreted route; provider-specific behavior remains visible.
- **Experimental:** incomplete, unstable, or community-maintained support.

An endpoint is not “supported” without its request, non-streaming response, streaming response, error, cancellation, and usage-accounting behavior in the matrix.

## 13. Adoption journey

The intended evaluation path is:

1. Download a signed binary or pinned container.
2. Create one minimal configuration referencing an upstream secret.
3. Run `hafiz-gateway check`.
4. Run `hafiz-gateway run`.
5. Change an OpenAI SDK base URL and gateway key.
6. Execute a streaming example.
7. Use `doctor` and local metrics to understand the request.

Median time for a competent developer following released documentation should be 10 minutes or less in usability tests.

## 14. Launch gates

### R0 — Market pain

Fail if direct research does not find a repeated, expensive problem with a clear owner.

### R1 — Wedge

Fail if the preferred solution is simply an incumbent configuration or if “Rust/faster/single binary” is the only reason to adopt.

### R2 — Trust

Fail if privacy, security, reliability, or audit requirements require a hot-path architecture that contradicts the product promise.

### R3 — Technical proof

Fail if streaming correctness, compatibility, or equivalent-control performance targets cannot be demonstrated reproducibly.

### R4 — Design partners

Fail or narrow if three credible teams will not test the same core use case.

## 15. Open decisions

- Exact primary market segment and buyer
- Hyper/Tokio versus Pingora network foundation
- First native provider set
- OpenAI Responses coverage required for v1
- Local identity/key store format
- Distributed quota consistency model
- Apache-2.0 versus dual MIT/Apache-2.0 licensing
- Definition of enterprise support and commercial boundary
- Whether content guardrails belong in-process, out-of-process, or outside the product

## 16. Approval record

This PRD becomes implementation-authoritative only when the owner records approval after R0 and R1 evidence is attached. Until then, issues and prototypes must identify the hypothesis they test and must not turn draft requirements into irreversible compatibility promises.
