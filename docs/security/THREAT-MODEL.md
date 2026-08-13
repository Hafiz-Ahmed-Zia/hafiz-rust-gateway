# Repository Threat Model

**Status:** Preliminary repository-scoped model; architecture exists, implementation does not  
**Scope:** Intended Hafiz Rust Gateway data plane, configuration/release tooling, and future optional integrations

## Overview

Hafiz Rust Gateway is intended to mediate enterprise AI traffic between client applications and external or self-hosted model providers. It will authenticate callers, protect upstream credentials, enforce policy and resource limits, route requests, proxy or translate streaming protocols, and emit privacy-preserving telemetry.

The gateway is a high-value trust boundary. It may handle proprietary prompts and outputs, tenant/project identity, tool-call data, provider credentials, routing policy, token/cost metadata, and audit evidence. A compromise can expose data across tenants, steal provider access, change model behavior, create large costs, or interrupt production applications.

At this stage, files under `docs/` describe proposed controls; they do not prove those controls exist. Security severity must be based on deployed reachability and evidence once code exists.

## Threat Model, Trust Boundaries, and Assumptions

### Assets

- upstream provider API keys, cloud identities, certificates, and secret-manager references;
- inbound caller credentials and workload identities;
- prompt, response, image/audio metadata, tool arguments/results, and structured outputs;
- tenant, project, model, routing, quota, and policy configuration;
- audit records and telemetry integrity;
- gateway availability, capacity, and provider quota/budget;
- configuration and release signing identities;
- artifact provenance, SBOMs, packages, images, and update channels;
- compatibility fixtures and policy/conformance results.

### Actors

- legitimate application caller;
- malicious external caller;
- malicious or compromised tenant/application;
- gateway operator/administrator;
- developer/contributor and CI/release maintainer;
- upstream AI provider or self-hosted model operator;
- external identity, secret, telemetry, configuration, and rate-limit services;
- supply-chain attacker;
- insider with partial operational or observability access.

### Trust boundaries

1. **Client to gateway:** untrusted network data becomes an authenticated request context.
2. **Tenant to tenant:** shared process/resources must not mix identities, content, limits, routes, or telemetry.
3. **Gateway to upstream:** provider credentials and private data cross into a third-party or separately administered system.
4. **Data plane to optional services:** secret managers, OIDC/JWKS, remote config, quotas, telemetry, and SIEM can fail or be compromised.
5. **Configuration/control plane to data plane:** operator input becomes executable routing/policy behavior.
6. **Data plane to logs/metrics/traces/audit:** sensitive runtime state is transformed into durable or externally exported evidence.
7. **Source/CI to release:** contributor-controlled changes and dependencies become trusted artifacts.
8. **Plugin/guardrail boundary:** future custom logic may process sensitive content or influence allow/deny/routing decisions.

### Attacker-controlled inputs

- method, path, query, headers, authorization material, content encoding, body, JSON depth/shape, stream/cancellation timing;
- model names, tool definitions/arguments, structured-output schemas, metadata, and large multimodal references where supported;
- TCP/TLS fragmentation and slow/abrupt connection behavior;
- DNS and upstream responses when the provider/network is compromised;
- provider response headers, bodies, SSE events, usage fields, errors, and delays;
- community pull requests, dependencies, build inputs, and issue/fixture content.

### Operator-controlled inputs

- listeners, TLS, routes, upstream URLs, credentials, identity issuers, limits, retry/fallback, logging, content capture, and extension configuration;
- configuration distribution and rollback;
- deployment privileges and network policy;
- release and signing policy.

Operator control is not automatically trusted: mistakes, compromised accounts, malicious insiders, stale config, and confused-deputy behavior remain in scope.

### Assumptions requiring validation

- deployments can provide a protected secret source and least-privilege runtime identity;
- upstream TLS/DNS trust is meaningful in the deployment;
- applications can authenticate to the gateway;
- operators understand that a local limit is not a globally consistent quota;
- the gateway cannot guarantee model truth, safety, or prompt-injection resistance solely through transport policy;
- semantic content guardrails, if added, are fallible and require explicit failure policy;
- availability and compliance claims depend on deployment topology and organizational controls outside this repository.

### Security invariants

1. One caller/tenant cannot access another tenant's credentials, content, policy, quota, or telemetry.
2. Inbound credentials are never forwarded as upstream provider credentials.
3. Secrets and prompt/response/tool content never enter default logs, traces, metrics, errors, or config diffs.
4. Invalid, ambiguous, expired, or unverifiable security policy fails closed.
5. Every parser, body, stream event, decompressor, queue, retry plan, and telemetry buffer is bounded.
6. Upstream selection cannot reach forbidden local, link-local, metadata, or internal networks without explicit trusted policy.
7. Configuration is fully validated and compiled before atomic activation; in-flight requests retain one immutable snapshot.
8. No retry/fallback silently replays a request after downstream-visible output.
9. Optional service failure cannot silently bypass a security-critical decision.
10. Release consumers can verify artifact identity, integrity, and provenance.

## Attack Surface, Mitigations, and Attacker Stories

### Ingress parsing and protocol handling

**Stories:** request smuggling, conflicting length/transfer headers, oversized or deeply nested JSON, decompression bombs, slowloris behavior, malformed SSE, and fragmentation-induced state errors.

**Required mitigations:** mature HTTP stack, strict normalization, byte/depth/time bounds, bounded incremental parsing, header allow/deny rules, fuzzing, differential protocol tests, and slow-client/provider chaos tests.

### Authentication, authorization, and tenant isolation

**Stories:** forged/ambiguous JWT, issuer/audience confusion, credential prefix collision, cache-key omission, policy using caller-provided tenant ID, or shared mutable request context causing cross-tenant routing.

**Required mitigations:** explicit issuer/audience/algorithm policy, immutable server-derived identity, constant-time secret comparison where applicable, namespaced cache/counter keys, negative tests, and per-request context ownership.

### Upstream credentials and SSRF

**Stories:** user-controlled base URL reaches cloud metadata or internal admin services; redirect/DNS rebinding bypasses allow rules; upstream error reveals a key; inbound auth overwrites provider auth.

**Required mitigations:** trusted config-only upstreams by default, scheme/host/IP policy, redirect policy, DNS resolution controls, egress network policy, header separation, redaction canaries, least-privilege credentials, and rotation tests.

### Routing, retry, and cost amplification

**Stories:** crafted requests trigger expensive models, repeated retries multiply cost, provider 429 storms create synchronized retries, partial streams are replayed, or a tenant consumes shared concurrency/quota.

**Required mitigations:** model allowlists, request/tenant budgets, retry classification and budgets, jitter, commitment point, circuit breakers, fairness, fast shedding, authoritative versus estimated token labeling, and spend alerts outside the hot path.

### Configuration and policy

**Stories:** partial reload bypasses auth; unknown field is ignored; compromised control plane pushes malicious upstream; rollback uses stale secrets; policy explain endpoint discloses sensitive configuration.

**Required mitigations:** versioned strict schema, complete offline validation, signed/provenanced snapshots where needed, atomic swap, last-known-good state, authorization separation, redacted explain output, and audited activation/rollback.

### Telemetry and audit

**Stories:** prompts leak into span attributes, authorization headers appear in errors, high-cardinality fields create denial of service/cost, audit export failure is silent, or attackers inject misleading log fields.

**Required mitigations:** content off by default, field allowlist, structured encoding, cardinality limits, redaction/canary tests, bounded async export, drop/backlog metrics, integrity/provenance for audit records, and least-privilege access.

### Extensions and guardrails

**Stories:** native plugin executes arbitrary code, out-of-process guardrail exfiltrates content, timeout causes accidental fail-open, or WASM consumes unbounded CPU/memory.

**Required mitigations:** no arbitrary native plugins early, explicit capabilities, isolation, strict resources/deadlines, data-minimization, authenticated extension channel, and per-policy fail-open/fail-closed decision.

### Supply chain and releases

**Stories:** malicious dependency/build script, compromised maintainer or CI token, unsigned replacement binary, poisoned container base, or unreviewed generated code reaches release.

**Required mitigations:** minimal/pinned dependencies, lockfile review, dependency/license/vulnerability checks, branch protection, least-privilege CI, reproducible builds where feasible, SBOM, signed provenance and artifacts, two-person release review, and documented keyless verification.

### Availability and lifecycle

**Stories:** connection/stream exhaustion, task leak, telemetry outage blocks requests, malformed update crashes all instances, or shutdown drops active streams.

**Required mitigations:** global/per-tenant bounds, soak tests, panic containment without hiding corrupted state, health/readiness separation, atomic config, optional telemetry shedding, graceful drain, and fault injection.

### Lower-relevance or externally owned stories

- Training-data poisoning and model theft are primarily provider/model-owner risks unless Hafiz Rust Gateway later hosts models, training, caches, or datasets.
- Hallucination, misinformation, and application overreliance are not solved by transport alone. They become gateway scope only for explicit, testable guardrail policy.
- Browser CSRF/XSS are low relevance to the headless data plane but become relevant if an administrative web UI is added.
- Conversation authorization and tool side effects belong partly to the application/agent; the gateway must not claim to guarantee them unless it mediates those capabilities with explicit identity and policy.

## Severity Calibration (Critical, High, Medium, Low)

### Critical

- unauthenticated remote code execution in an internet-exposed gateway;
- broad cross-tenant prompt/response or provider-secret compromise;
- release-signing/build compromise distributing malicious official artifacts;
- authorization bypass permitting unrestricted high-value model/tool use across deployments.

### High

- reliable SSRF to cloud metadata/internal control services;
- tenant isolation failure with meaningful but bounded data/cost impact;
- default telemetry leaks sensitive prompts, credentials, or tool results;
- policy/reload bypass disables authentication or model restrictions;
- remotely triggerable process-wide denial of service at realistic cost;
- unsafe retry behavior causes large cost or duplicate side effects.

### Medium

- denial of service requiring authenticated tenant access and substantial traffic;
- incomplete redaction of low-sensitivity metadata;
- local quota inconsistency incorrectly enforced within documented bounded-stale mode;
- diagnostic information that materially helps targeted attacks but exposes no secrets/content;
- provider-specific conformance failure with a safe visible error and limited blast radius.

### Low

- minor non-sensitive version/timing disclosure;
- low-impact log/metric cardinality issue under privileged configuration;
- documentation/default inconsistency that does not bypass a control;
- availability degradation requiring local operator access and easy recovery.

Severity changes with exposure, tenant count, data sensitivity, credentials, and deployment controls. No finding is suppressed merely because a proposed control exists in documentation.
