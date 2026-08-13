# Enterprise AI Gateway — Deep Research Plan

Status: Active research design baseline for Hafiz Rust Gateway  
Date: 2026-08-13  
Language for project artifacts: English  
Purpose: Determine whether a new open-source AI traffic gateway can earn a genuine 9.5/10 opportunity rating, and identify the narrow product thesis capable of doing so.

## 1. Decision We Are Actually Researching

The decision is not whether a Rust gateway can be built. It can.

The decision is:

> Is there a painful, durable, insufficiently solved enterprise problem at the AI traffic boundary for which companies will adopt, trust, operate, and potentially pay for a new open-source gateway?

The research must be willing to reject or materially change the initial idea. Rust, low latency, self-hosting, a single binary, unified APIs, retries, fallbacks, caching, and cost dashboards are hypotheses or implementation choices—not proof of product-market fit and not standalone moats.

## 2. Current Market Reality to Treat as Baseline

The market already contains substantial competitors:

- TensorZero: historically a direct Rust gateway/LLMOps competitor, but its official site now says it is no longer maintained and its GitHub repository was archived on 2026-06-12. Treat it as both a technical benchmark and a critical sustainability/product-market-fit case study, not an active maintained option.
- Envoy AI Gateway: an Envoy and Kubernetes-native AI gateway with provider integrations and two-tier gateway patterns.
- LiteLLM: a widely adopted unified provider API and proxy with budgets, keys, cost tracking, and rate limits.
- Portkey: open-source gateway with routing, retries, fallbacks, guardrails, budgets, and caching.
- Kong AI Gateway: AI-aware capabilities on an established enterprise gateway.
- Cloudflare AI Gateway: managed global gateway with analytics, caching, spend controls, routing, DLP, BYOK, and logging.
- Azure API Management AI Gateway and Google Apigee: enterprise governance for models, agents, MCP tools, and related protocols.
- Other direct and adjacent products such as Bifrost, Helicone, OpenRouter, provider-native gateways, service meshes, and internal proxies.

Useful current primary references:

- TensorZero current status: https://www.tensorzero.com/ and https://github.com/tensorzero/tensorzero
- Envoy AI Gateway: https://github.com/envoyproxy/ai-gateway
- LiteLLM proxy: https://docs.litellm.ai/
- Portkey gateway: https://portkey.ai/docs/product/ai-gateway
- Kong AI Gateway: https://developer.konghq.com/ai-gateway/
- Cloudflare AI Gateway: https://developers.cloudflare.com/ai-gateway/features/
- Azure API Management AI gateway: https://learn.microsoft.com/azure/api-management/genai-gateway-capabilities
- Google Apigee AI gateway: https://docs.cloud.google.com/apigee/docs/api-platform/get-started/ai-capabilities

Research must verify every fact again at execution time.

## 3. Definition of a 9.5/10 Opportunity

A high score must be earned with evidence. The researcher must not inflate the score to match the founder's preference.

| Dimension | Weight | What a strong result requires |
| --- | ---: | --- |
| Problem intensity and frequency | 15 | Repeated, expensive, urgent pain across a specific buyer segment |
| Differentiation and defensibility | 15 | A clear wedge that alternatives cannot easily copy or bundle away |
| Enterprise trust and reliability | 12 | Measurable safety, correctness, resilience, and operability |
| Security, privacy, and governance | 12 | Secure defaults, auditable controls, data minimization, and supply-chain trust |
| Developer and operator experience | 10 | Fast adoption, safe configuration, low operational burden, excellent diagnostics |
| Performance and resource efficiency | 8 | Reproducible material advantage under realistic workloads |
| Interoperability and future resilience | 8 | Provider, protocol, deployment, and telemetry portability |
| Open-source adoption potential | 8 | Clear user value, contributor path, credible governance, and permissive adoption |
| Commercial viability | 7 | A buyer, budget, expansion path, and sustainable open-source model |
| Execution feasibility | 5 | A staged path that can prove the thesis before building a giant platform |
| Total | 100 | A 95+ result requires evidence, not a feature count |

Each score must include:

1. Evidence for the score.
2. Evidence against the score.
3. Confidence level.
4. What would change the score.
5. Whether the result applies to startups, mid-market companies, regulated enterprises, or all of them.

## 4. Core Research Principles

### 4.1 Start with company pain, not architecture

Investigate current workflows, incidents, costs, governance gaps, procurement objections, operational toil, and switching triggers before designing features.

### 4.2 Separate table stakes from a moat

Unified API, API-key hiding, retries, routing, basic budgets, cost estimation, logging, and a dashboard are likely table stakes. The research must identify which capabilities are:

- expected table stakes;
- purchase or adoption triggers;
- durable differentiators;
- distractions;
- actively harmful complexity.

### 4.3 Treat lightweight as a measured property

“Lightweight” must be defined through idle memory, memory per active stream, binary or image size, startup time, CPU per request, added latency, throughput, dependency count, required services, upgrade burden, and operational steps.

### 4.4 Treat secure-by-default as behavior

Security claims must map to default settings, threat controls, tests, signed artifacts, safe failure modes, and audit evidence.

### 4.5 Prefer a narrow primary wedge

The final thesis may have one primary wedge and at most two supporting advantages. A product trying to be a gateway, observability suite, evaluation platform, prompt manager, agent framework, and SIEM on day one should be scored down.

## 5. Research Program

## Workstream A — Company Demand and Jobs-to-Be-Done

### Objective

Determine what companies truly need at the AI traffic boundary, who owns the pain, and what causes adoption or switching.

### Segments

- AI-native startups with material model spend.
- SaaS companies adding AI features.
- Mid-market platform teams serving multiple product teams.
- Large enterprises with internal AI platforms.
- Regulated sectors: finance, healthcare, government, legal, telecom, and critical infrastructure.
- Organizations running self-hosted models.
- Hybrid and multi-cloud organizations.
- Agencies and multi-tenant AI platforms.

### Roles to study

- VP/Head of Platform Engineering.
- AI Platform or ML Platform lead.
- SRE and production engineering.
- Application and AI engineers.
- CISO, security architecture, IAM, and data governance.
- FinOps, finance, and procurement.
- Compliance, privacy, risk, and internal audit.
- Engineering managers operating shared model access.

### Questions

- How many providers, models, regions, keys, teams, and environments are in use?
- What sits between applications and models today?
- Which incidents have occurred: cost spikes, provider outages, data leakage, quota exhaustion, poor fallback quality, missing audit evidence, or configuration drift?
- What is the current monthly engineering and infrastructure cost of the solution?
- Which problem is painful enough to justify replacing or inserting infrastructure in a critical request path?
- Who approves the gateway, who operates it, who pays, and who can block it?
- What evidence is required before production adoption?
- What does the security questionnaire or vendor review ask?
- Why are current tools rejected, replaced, or supplemented?
- What is built internally, and why was an existing product insufficient?
- Which features are used weekly versus merely requested during procurement?
- What migration and lock-in concerns exist?
- Would the organization prefer library, sidecar, daemon, centralized service, Kubernetes gateway, or managed control plane?
- What is an acceptable failure mode if the gateway, control plane, telemetry store, policy engine, or provider fails?
- Is prompt or response logging permitted at all?
- What must remain within a country, region, VPC, or air-gapped environment?
- How are provider invoices reconciled with internal usage today?
- Are agents, MCP servers, tools, A2A endpoints, realtime sessions, multimodal models, and batch workloads entering scope?

### Evidence target

- At least 20 credible direct interviews before final PRD.
- At least 5 interviewees from regulated or high-governance environments.
- At least 5 teams operating meaningful production AI traffic.
- At least 3 teams that rejected or replaced an existing gateway.
- An anonymized pain-frequency and willingness-to-switch matrix.

If interviews are unavailable during desk research, produce the interview instrument and clearly label all market conclusions as provisional.

## Workstream B — Competitive and Substitute Analysis

### Objective

Understand what is already solved, where products fail in practice, and where a new entrant could win.

### Competitor groups

1. AI-native gateways: LiteLLM, TensorZero, Portkey, Bifrost, Helicone, OpenRouter, and other actively maintained projects.
2. General enterprise gateways: Kong, Envoy AI Gateway, Tyk, Traefik, and related service-mesh products.
3. Cloud platforms: Cloudflare AI Gateway, Azure API Management, Apigee, Amazon Bedrock capabilities, and other cloud-native controls.
4. Model-serving and inference-routing systems: vLLM ecosystem components, Kubernetes Gateway API Inference Extension, KServe-adjacent systems, and load-balancing layers.
5. Substitutes: direct provider SDKs, NGINX/Envoy custom configuration, homegrown proxies, libraries, service meshes, and doing nothing.

### Matrix fields

- Product category and target buyer.
- License, open-core boundary, governance, and contributor activity.
- Implementation language and deployment architecture.
- Required databases and external services.
- Supported providers, endpoints, modalities, and protocols.
- Translation fidelity and provider-specific escape hatches.
- Streaming and realtime behavior.
- Authentication, identity propagation, authorization, tenancy, and secrets.
- Rate, token, concurrency, cost, and budget controls.
- Routing, load balancing, failover, retries, hedging, and circuit breaking.
- Data residency, retention, DLP, redaction, and auditability.
- Observability, OpenTelemetry, logs, metrics, traces, and billing reconciliation.
- Configuration model, validation, GitOps, hot reload, rollout, and rollback.
- HA, multi-region, disaster recovery, fleet management, and upgrades.
- Performance claims and whether they are reproducible.
- Security posture, disclosures, signed releases, SBOM, and supply-chain controls.
- Documentation quality, onboarding time, and failure diagnostics.
- Pricing, enterprise packaging, and support model.
- GitHub issue themes, unresolved pain, breaking-change history, and maintenance risk.
- Known production users and independently verifiable adoption.
- Switching costs and ecosystem integrations.

### Required output

Produce:

- a factual capability matrix;
- a table-stakes list;
- a gap heatmap;
- a “why not use X?” answer for every major competitor;
- a competitor-response analysis describing how quickly each could copy the proposed wedge;
- a build, integrate, contribute, or compete recommendation.

## Workstream C — Wedge Discovery and Thesis Falsification

Investigate at least these candidate wedges without assuming any is correct:

1. Verifiable AI traffic governance: deterministic policy decisions, explainable routing, tamper-evident audit receipts, and provable configuration provenance.
2. Reliability-correct routing: capability-aware translation, safe retry boundaries, semantic fallback contracts, and provider compatibility certification.
3. Ultra-light data plane: small stateless binary, no mandatory database, sidecar or centralized mode, predictable tail latency, and optional external control plane.
4. Privacy and sovereignty gateway: metadata-only defaults, local redaction, strict egress policy, regional routing, retention enforcement, and air-gapped operation.
5. Identity-aware agent and tool gateway: end-user and workload identity propagation across model, MCP, and agent calls with least-privilege policy.
6. Auditable AI FinOps ledger: accurate attribution, negotiated pricing, provider-invoice reconciliation, budgets, forecasting, and signed usage evidence.
7. Embedded gateway engine: a Rust library or daemon that platforms can embed without adopting a large opinionated stack.
8. Compatibility certification layer: continuous conformance testing across provider API changes, streaming edge cases, tool calls, structured output, and modalities.

For every wedge:

- identify the exact buyer and urgent job;
- quantify frequency, severity, and budget impact;
- list existing alternatives;
- explain why incumbents cannot trivially copy it;
- define the smallest proof;
- identify adoption friction;
- identify new security or operational risk;
- define a kill criterion.

The research must recommend one primary wedge, at most two supporting advantages, and explicitly reject the rest for the first product.

## Workstream D — Protocol and Compatibility Surface

Research the present and likely two-to-three-year protocol surface:

- OpenAI Chat Completions, Responses, Realtime, embeddings, images, audio, batch, tool calls, and structured outputs.
- Anthropic Messages, streaming, tools, prompt caching, and provider-specific semantics.
- Google Gemini, Vertex AI, Azure OpenAI/Foundry, Amazon Bedrock, and common OpenAI-compatible servers.
- Self-hosted inference servers such as vLLM, Ollama, llama.cpp, SGLang, and TGI where relevant.
- Server-Sent Events, WebSockets, HTTP/2, HTTP/3, gRPC, and JSON-RPC implications.
- MCP, A2A, and emerging agent/tool protocols, but only if demand evidence supports inclusion.
- Kubernetes Gateway API and the Inference Extension.
- OpenTelemetry GenAI semantic conventions.

The report must distinguish:

- true normalization;
- lossy translation;
- provider passthrough;
- capability negotiation;
- unsupported semantics;
- versioning and deprecation policy.

## Workstream E — Data-Plane Architecture and Performance

### Architecture questions

- Should control plane and data plane be separate?
- Can the data plane remain stateless?
- Which features must be synchronous, asynchronous, optional, or external?
- Which dependencies are allowed on the hot path?
- How are configuration snapshots validated, signed, distributed, activated, and rolled back?
- Can the gateway operate safely if telemetry, database, cache, policy service, or control plane is unavailable?
- Which deployment modes are required: single process, sidecar, daemon, centralized cluster, Kubernetes, VM, edge, and air-gapped?
- Is a plugin system necessary, and can it avoid ABI, security, and performance hazards?
- Should extensibility use compiled modules, WASM, external processors, policy DSL, or stable RPC?

### Performance dimensions

Do not pick targets without baselines. Research and benchmark:

- cold and warm startup;
- idle RSS and resident memory under load;
- memory per concurrent stream;
- container image and compressed binary size;
- p50, p95, p99, and p99.9 added gateway latency;
- time-to-first-byte and time-to-first-token overhead;
- stream inter-chunk delay and backpressure behavior;
- requests and tokens per second;
- concurrent long-lived streams;
- CPU time and allocations per request;
- connection reuse, DNS, TLS, and upstream pool behavior;
- configuration-reload impact;
- telemetry-on versus telemetry-off overhead;
- policy, cache, redaction, and guardrail overhead independently;
- overload behavior, queue growth, load shedding, and recovery;
- performance across small JSON, large contexts, tool payloads, images, audio, and streaming.

### Benchmark integrity

- Publish hardware, OS, runtime, topology, versions, source, configuration, workload, warmup, run duration, and raw data.
- Separate loopback proxy overhead from real network and provider latency.
- Compare equivalent features and security settings.
- Prevent coordinated omission and report error rates.
- Use repeated runs and confidence intervals.
- Test at saturation and below saturation.
- Include resource efficiency, not only requests per second.
- Maintain an independent reproducibility script and CI regression thresholds.
- Never market a “times faster” number without reproducible apples-to-apples evidence.

## Workstream F — Reliability and Failure Semantics

Research and specify:

- end-to-end deadline propagation and per-attempt budgets;
- connection, header, idle, body, and total timeouts;
- retry eligibility before and after upstream acceptance;
- idempotency and duplicate-generation risks;
- streaming failures before first byte, mid-stream, and after usage reporting;
- circuit breaking, outlier detection, adaptive concurrency, and load shedding;
- provider, model, region, account, and credential health;
- fail-open versus fail-closed behavior per policy;
- fallback compatibility, quality, safety, context limits, tools, schemas, and data-residency constraints;
- hedging risks and cost duplication;
- graceful shutdown and connection draining;
- configuration rollout, canary, validation, atomic activation, and rollback;
- distributed rate-limit consistency and expected approximation;
- cache correctness, privacy boundaries, tenant isolation, and invalidation;
- usage reconciliation when providers return delayed, incomplete, or conflicting counts;
- disaster recovery, multi-region behavior, split-brain risks, and recovery objectives;
- chaos, fault injection, soak, and upgrade testing.

“Fallback” must never be presented as transparent unless compatibility and policy conditions are met.

## Workstream G — Security, Privacy, and Governance

Use current primary standards and guidance, including:

- OWASP GenAI Security Project and current LLM/agent guidance.
- NIST AI Risk Management Framework and Generative AI Profile.
- NIST Secure Software Development Framework.
- OpenSSF OSPS Baseline, Scorecard, and best practices.
- SLSA and signed build provenance.
- Applicable privacy, sector, and AI regulations by target market.

### Threat-model scope

- External clients, compromised clients, malicious tenants, insiders, maintainers, plugins, providers, control plane, data plane, telemetry pipeline, storage, CI/CD, update channel, and dependencies.
- API key theft, credential confusion, identity loss, broken tenant isolation, SSRF, request smuggling, header confusion, path confusion, oversized payloads, decompression bombs, denial of service, resource starvation, cache poisoning, policy bypass, malicious config, downgrade, replay, log injection, secret leakage, data exfiltration, and supply-chain compromise.
- Prompt injection and model-output risks must be scoped honestly. The gateway may enforce policy, but must not claim to “solve prompt injection.”

### Required control research

- OIDC/OAuth2, workload identity, mTLS, SPIFFE/SPIRE, API keys, short-lived credentials, and end-user identity propagation.
- RBAC, ABAC, policy-as-code, separation of duties, approval workflows, and break-glass access.
- Provider key storage, rotation, KMS/HSM/Vault integration, envelope encryption, and zeroization.
- Tenant isolation in memory, cache, logs, quotas, metrics, and storage.
- Egress allowlists, private networking, proxies, DNS controls, and regional pinning.
- Request/response data classification, selective capture, redaction, tokenization, encryption, and retention.
- Immutable or tamper-evident audit records and trustworthy time.
- Secure admin and debug surfaces.
- Signed releases, SBOMs, dependency review, vulnerability disclosure, security policy, provenance, reproducible builds where feasible, and emergency release process.
- Fuzzing, property testing, parser differential testing, unsafe-code policy, secrets scanning, static analysis, dependency audit, and penetration testing.
- Defaults that remain safe without an enterprise license.

## Workstream H — Observability, Logging, and Audit Design

### Principle

Prompt and response bodies are sensitive content, not ordinary logs. Default to metadata-only telemetry. Content capture must be explicit, scoped, redacted where possible, access-controlled, encrypted, retention-bound, and auditable.

### Three separate evidence classes

1. Operational telemetry: metrics, traces, sampled diagnostics, and health.
2. Security and compliance audit: append-only records of identity, policy, configuration, and privileged actions.
3. Usage and financial ledger: attributable token, request, cache, and cost records with estimation and reconciliation status.

Do not mix these into one unrestricted log table.

### Request-lifecycle events to research

- request received and admitted;
- authentication result;
- authorization and policy decision;
- quota and budget decision;
- cache lookup and decision;
- route candidates evaluated;
- route selected with reason;
- upstream attempt started;
- upstream headers received;
- stream started;
- retry, hedge, or fallback decision;
- upstream attempt completed;
- stream completed, cancelled, disconnected, or aborted;
- guardrail or DLP decision;
- usage estimated;
- provider usage received;
- cost estimated;
- invoice or billing record reconciled;
- audit export or retention action;
- configuration snapshot activated or rejected.

### Candidate fields

- gateway request ID, trace ID, span ID, parent ID, correlation ID, and idempotency key hash;
- pseudonymous tenant, project, environment, application, workload, user, and credential identifiers;
- requested API, requested model alias, resolved provider/model/deployment/region, and capability profile;
- route, policy, pricing, tokenizer, gateway, and configuration versions;
- wall-clock timestamps plus monotonic durations;
- queue, auth, policy, cache, upstream connect, TLS, first-byte, first-token, stream, and total duration;
- request size, response size, input, cached-input, output, reasoning, image, audio, and other usage units when supported;
- usage provenance: estimated, provider-reported, corrected, or reconciled;
- price source, negotiated rate version, currency, estimated cost, billed cost, and variance;
- attempt number, retry reason, fallback reason, circuit state, and selected route reason;
- HTTP and provider status, gateway error taxonomy, retryability, and safe diagnostic code;
- cache namespace, key version, hit type, similarity policy version, and tenant boundary;
- security decision, data classification, redaction count/type, policy ID, action, and reason code;
- client disconnect, cancellation, partial stream, and usage uncertainty;
- telemetry sampling and body-capture policy.

### Metrics

Research stable names and cardinality limits for:

- request and attempt counts;
- success, rejection, timeout, cancellation, and error taxonomy;
- request and token throughput;
- latency histograms including time to first token;
- active and queued requests/streams;
- provider/model/region health;
- rate-limit, budget, policy, DLP, cache, retry, fallback, and circuit events;
- token and cost totals with estimated versus reconciled status;
- memory, CPU, file descriptors, connection pools, queues, dropped telemetry, and config age;
- SLO burn rates and availability/error-budget signals.

Use OpenTelemetry conventions where mature. Version experimental GenAI conventions explicitly and avoid high-cardinality labels in metrics.

### Privacy and operations questions

- Which identifiers are raw, hashed, tokenized, or prohibited?
- Who can enable content capture and for how long?
- How are data-subject deletion, legal hold, regional retention, and tenant export handled?
- What happens when the telemetry sink is down?
- Which telemetry is dropped first under pressure?
- Can the hot path remain available without the analytics store?
- How are audit records protected from operators with ordinary debug access?

## Workstream I — Enterprise Control Plane

Research whether enterprises require:

- organizations, tenants, projects, environments, and delegated administration;
- SSO through OIDC/SAML and lifecycle through SCIM;
- RBAC/ABAC, service accounts, workload identity, and just-in-time access;
- policy-as-code, policy simulation, explain mode, staged rollout, approvals, and rollback;
- model catalog, aliases, capability contracts, ownership, and deprecation;
- provider credentials, regional endpoints, negotiated pricing, and secret rotation;
- developer self-service with centrally enforced guardrails;
- fleet inventory, health, configuration drift, and upgrade management;
- HA, multi-region, backup, disaster recovery, and support bundles;
- private networking, proxies, air-gap operation, and data-residency enforcement;
- audit exports, SIEM, OpenTelemetry, data lake, and billing exports;
- chargeback/showback, budget hierarchy, anomaly alerts, and invoice reconciliation;
- compliance evidence packs and control mappings.

Every control-plane feature must be classified as:

- required in open-source core;
- optional open-source module;
- enterprise/commercial capability;
- integration with an existing system;
- rejected complexity.

## Workstream J — Zero-Complexity Developer and Operator Experience

Research and prototype the desired path:

- download one verified binary or run one minimal container;
- validate one configuration file;
- use environment or external secret references without copying provider keys into plaintext;
- start with no mandatory database;
- change one base URL in an existing SDK;
- receive a useful health and readiness report;
- inspect why a route or policy decision occurred;
- upgrade and roll back safely.

Study:

- installation on Linux, macOS, Windows where justified, Docker, Kubernetes, systemd, and air-gapped environments;
- config schema, strict validation, safe defaults, lint, format, dry-run, diff, explain, migration, and backward compatibility;
- hot reload versus restart semantics;
- diagnostics without secret disclosure;
- provider conformance tests;
- local development without cloud accounts;
- failure messages and remediation guidance;
- quickstart time measured with new users;
- optional UI versus CLI/API/GitOps first;
- migration guides from LiteLLM, direct SDKs, Envoy/Kong, and custom proxies.

The dashboard must not become a mandatory dependency of the data plane.

## Workstream K — Open-Source Trust, Governance, and Sustainability

Research:

- Apache-2.0, MPL-2.0, AGPLv3, dual licensing, and other viable license models;
- trademark and project-name clearance;
- DCO versus CLA;
- maintainer governance, roadmap transparency, and conflict-of-interest policy;
- contributor guide, code of conduct, issue templates, RFCs, ADRs, and compatibility policy;
- SECURITY.md, vulnerability reporting, embargo handling, security contacts, and support windows;
- release cadence, semantic versioning, LTS policy, deprecation, migration, signed artifacts, checksums, SBOM, and provenance;
- public benchmarks and regression dashboards;
- neutral integrations and avoidance of artificial cloud lock-in;
- sustainable commercial boundary that does not cripple security or self-hosting in the open-source edition;
- path toward OpenSSF best-practice maturity and potential foundation stewardship if adoption warrants it.

## Workstream L — Commercial and Adoption Model

Research:

- exact economic buyer and budget source;
- managed control plane with customer-hosted data plane;
- fully managed service;
- enterprise support and LTS;
- compliance, fleet, governance, and collaboration capabilities;
- professional services only as an initial bridge, not the primary moat;
- pricing unit: gateway instance, request, token, workspace, developer, cluster, or support tier;
- conflict between usage pricing and the promise of cost control;
- cloud-provider bundling risk;
- sales cycle and procurement evidence;
- design-partner program and conversion signals.

## 6. Research Method and Evidence Standard

### Source hierarchy

1. Direct customer interviews, design-partner evidence, observed workflows, anonymized incidents, RFPs, and security questionnaires.
2. Official product documentation, source repositories, release notes, issue trackers, standards, regulatory texts, and vendor status reports.
3. Independent benchmarks, peer-reviewed or strong systems research, public architecture talks, and incident postmortems.
4. High-quality practitioner reports with disclosed context.
5. Community posts only as leads or qualitative evidence, never sole proof of a market-wide claim.

### Claim ledger

For every material claim record:

- claim;
- source URL or interview code;
- source type;
- publication and access date;
- target segment;
- supporting evidence;
- contradictory evidence;
- confidence;
- implication;
- whether the claim is fact, inference, estimate, or hypothesis.

### Anti-hallucination rules

- Do not invent adoption numbers, benchmarks, prices, customer quotes, incidents, or feature support.
- Separate vendor claims from independent evidence.
- Label missing evidence.
- Use exact dates and versions.
- Prefer primary sources.
- Re-check facts likely to change.
- Do not convert “supports OpenAI-compatible APIs” into “full compatibility.”
- Do not assume self-hosted means private, compliant, secure, or operationally simple.
- Do not assume Rust automatically produces low latency or reliability.

## 7. Required Research Deliverables

The completed research package must contain:

1. Executive decision memo with proceed, narrow, pivot, contribute, or stop recommendation.
2. Evidence-backed company-needs report by segment and persona.
3. Jobs-to-be-done and pain-frequency matrix.
4. Buyer, operator, approver, blocker, and procurement map.
5. Competitor and substitute matrix.
6. Table-stakes versus adoption-trigger versus moat classification.
7. Gap heatmap and opportunity shortlist.
8. Primary wedge recommendation with rejected alternatives.
9. Product thesis and one-sentence positioning.
10. Initial open-source/commercial boundary.
11. Protocol and compatibility report.
12. Data-plane and control-plane architecture options with tradeoffs.
13. Performance benchmark and capacity-test specification.
14. Reliability model and failure-semantics matrix.
15. Security threat model and control mapping.
16. Privacy, logging, audit, and telemetry specification.
17. Enterprise readiness requirements.
18. Zero-complexity installation and operations target journey.
19. OSS license, governance, release, and supply-chain recommendation.
20. Business model, pricing hypotheses, and adoption strategy.
21. Risk register with probability, impact, detection, mitigation, and owner.
22. Validation experiments and design-partner plan.
23. Phased roadmap from proof to production to enterprise.
24. Final weighted score with uncertainty and evidence.
25. Kill criteria and pivot options.
26. Source appendix and claim ledger.

## 8. Research Gates

### R0 — Market pain gate

Pass only if a specific segment has repeated, costly pain and a credible owner.

### R1 — Wedge gate

Pass only if one narrow advantage creates a realistic adoption reason beyond “written in Rust” or “faster.”

### R2 — Trust gate

Pass only if security, privacy, reliability, and logging defaults can satisfy the target segment without making the hot path heavy.

### R3 — Technical proof gate

Pass only after a thin spike demonstrates core protocol fidelity, streaming correctness, resource bounds, and realistic performance.

### R4 — Design-partner gate

Pass only if at least three credible teams agree to test the same primary use case and provide feedback or traffic replays.

### R5 — PRD gate

Only now create and lock the first PRD, architecture baseline, threat model, benchmark contract, and implementation roadmap.

### R6 — Public launch gate

Pass only when the project has signed reproducible artifacts where feasible, security policy, conformance tests, truthful benchmarks, migration docs, failure diagnostics, and a supportable compatibility promise.

## 9. Kill Criteria

Recommend contribution to an existing project, a narrower component, or a pivot if:

- interviews do not reveal a repeated high-cost problem;
- the only differentiation is Rust, latency, a single binary, or a nicer dashboard;
- an incumbent can copy the wedge with modest effort and already owns distribution;
- required protocol breadth makes a small team permanently reactive;
- enterprise trust requires a control plane too large to validate incrementally;
- performance advantage disappears under equivalent security and telemetry;
- buyers prefer cloud-native bundled gateways and will not adopt an independent layer;
- the open-source/commercial boundary cannot be made trustworthy and sustainable;
- no credible design partners commit to a common use case.

## 10. Documentation Structure After Research, Not Before

Once the project name and thesis are approved, use a public OSS structure inspired by the strongest Sasta Inverter patterns:

- docs/README.md — navigation and document ownership.
- docs/product/ — vision, personas, jobs, scope, non-goals, and PRD.
- docs/architecture/ — data plane, control plane, protocols, reliability, telemetry, deployment, and performance contracts.
- docs/security/ — threat model, security architecture, privacy, secure defaults, disclosure, and supply chain.
- docs/decisions/ — ADRs.
- docs/benchmarks/ — methodology, environments, raw-data policy, and regression thresholds.
- docs/conformance/ — provider and protocol compatibility.
- docs/roadmap/ — phased outcomes and explicit gates.
- docs/research/ — evidence, market map, competitor analysis, and validation history.
- CONTRIBUTING.md, GOVERNANCE.md, SECURITY.md, CODE_OF_CONDUCT.md, SUPPORT.md, and release policy at repository root.

Use AGENTS.md because AI will perform substantial development, but make it concise, public, testable, contributor-safe, and subordinate to product and architecture truth. Do not copy personal-project orchestration rules or exaggerated quality language into the public repository.

## 11. Immediate Next Sequence

1. Keep **Hafiz Rust Gateway** as the complete working mark and complete formal clearance before commercial brand investment.
2. Run the accompanying master research prompt with the strongest available research model.
3. Return the complete report and source list for critical review.
4. Compare it against `03-independent-baseline-research.md` with `04-pro-research-comparison-template.md`.
5. Challenge both reports, resolve contradictions, and identify missing primary evidence.
6. Conduct direct company interviews and design-partner discovery.
7. Select or reject the candidate primary wedge.
8. Promote `docs/product/PRD.md` from evidence-gated draft only after R0 and R1 pass.
