# Master Prompt — Enterprise-Grade Open-Source AI Gateway Research

Copy everything below into the research model. If the model supports deep-research mode, web access, long context, source exports, or asynchronous research, enable them.

---

## Role

Act as a combined:

- Principal distributed-systems architect;
- enterprise AI platform product strategist;
- API gateway and service-mesh specialist;
- Rust performance engineer;
- SRE and resilience engineer;
- application and AI security architect;
- privacy and compliance researcher;
- FinOps practitioner;
- open-source maintainer and ecosystem strategist;
- skeptical technical due-diligence analyst.

You are not being asked to confirm a founder's preferred idea. Your job is to discover whether the opportunity is genuinely exceptional, identify the narrow thesis that could win, and recommend stopping, contributing to an incumbent, or pivoting if the evidence does not support a new project.

## Date and Freshness

Treat the current date as 2026-08-13. Verify all time-sensitive claims using current sources. Include exact publication/access dates and relevant software versions. Explicitly mark any source or fact that may have changed.

## Project Context

The founder is considering a major international open-source project: a high-performance, self-hosted AI traffic gateway, likely with a Rust data plane.

The initial concept is a layer between applications and model providers or self-hosted inference endpoints. Possible capabilities include unified APIs, routing, retries, fallbacks, rate and token limits, budgets, cost attribution, observability, privacy controls, logging, caching, policy enforcement, and enterprise governance.

The desired product qualities are:

- extremely low and predictable latency;
- very small resource footprint;
- secure and private by default;
- zero-complexity initial deployment;
- optional scale from one binary to enterprise HA;
- strong protocol fidelity and streaming correctness;
- honest, reproducible benchmarks;
- vendor-neutral and truly self-hostable;
- open-source trust and sustainable governance;
- credible enough for top global companies to operate in a critical request path.

Do not assume these qualities create product-market fit. Do not assume Rust is a moat. Do not assume a feature-rich gateway is better than a narrow one.

## Primary Decision

Answer:

> Is there a painful, durable, insufficiently solved enterprise problem at the AI traffic boundary for which companies will adopt, trust, operate, and potentially pay for a new open-source product—and what exact product thesis would make this opportunity score at least 9.5/10?

If the evidence cannot support a 9.5/10 rating, give the honest score and explain the shortest path to improve it or why the project should not proceed.

## Mandatory Research Standards

1. Use primary sources wherever possible:
   - official documentation and source repositories;
   - release notes and roadmaps;
   - GitHub issues and discussions;
   - standards and regulatory texts;
   - public incident reports and status postmortems;
   - engineering blogs describing real deployments;
   - RFPs, security questionnaires, procurement material, and credible case studies.
2. Use independent systems research and benchmarks where available.
3. Use forums, Reddit, social posts, and vendor comparison pages only as leads or qualified practitioner evidence.
4. Distinguish:
   - verified fact;
   - vendor claim;
   - independent evidence;
   - inference;
   - estimate;
   - hypothesis.
5. Never fabricate customer quotes, adoption numbers, benchmarks, incidents, prices, or feature support.
6. For every important conclusion, present supporting and contradictory evidence.
7. Link citations directly to the exact supporting page.
8. Record publication date, access date, target segment, and confidence.
9. Do not equate OpenAI compatibility with semantic parity.
10. Do not equate self-hosting with privacy, security, compliance, or easy operations.
11. Do not reward feature count. Penalize unjustified scope and operational weight.
12. Treat the thesis as falsifiable.

## Current Competitor Baseline

At minimum investigate the current versions and actual capabilities of:

- LiteLLM;
- TensorZero as a historical direct competitor and current wind-down case: verify the official “no longer maintained” status, repository archival on 2026-06-12, reasons stated by maintainers, migration impact, security-maintenance implications, and lessons for open-source/commercial sustainability;
- Envoy AI Gateway;
- Portkey;
- Kong AI Gateway;
- Bifrost;
- Helicone;
- OpenRouter;
- Cloudflare AI Gateway;
- Azure API Management AI gateway;
- Google Apigee AI gateway;
- Amazon Bedrock routing, guardrail, governance, and inference capabilities;
- Kubernetes Gateway API Inference Extension;
- relevant model-serving routers and gateways around vLLM, SGLang, KServe, Ollama, llama.cpp, TGI, and similar systems;
- traditional substitutes such as NGINX, Envoy, service meshes, provider SDKs, and internal proxies.

Discover additional active competitors rather than treating this list as complete.

Do not copy an older comparison that still treats TensorZero as actively maintained. Use its former feature/performance scope to test whether “Rust + fast gateway + broad LLMOps” was sufficient, and use primary sources for any claim about why the project ended.

## Research Workstream 1 — Enterprise Demand

Determine what companies actually need, not what gateway vendors advertise.

Segment the market by:

- AI-native startups;
- SaaS companies adding AI;
- mid-market platform teams;
- large multi-team enterprises;
- regulated organizations;
- hybrid/multi-cloud organizations;
- self-hosted-model operators;
- multi-tenant AI platforms and agencies.

Analyze these stakeholders separately:

- AI/ML platform lead;
- platform engineering;
- SRE/production engineering;
- application and AI developers;
- CISO/security architecture/IAM;
- privacy, governance, compliance, and internal audit;
- FinOps/finance/procurement;
- engineering leadership.

Find evidence for:

- current architecture and workflow;
- number and type of providers/models/endpoints;
- operational and security incidents;
- cost leakage and reconciliation problems;
- outage and provider-quota problems;
- privacy, sovereignty, and audit blockers;
- internal engineering toil;
- procurement requirements;
- switching triggers and switching costs;
- budget owner and willingness to pay;
- reasons existing gateways are rejected or replaced;
- features requested during procurement versus used in operations;
- preferred deployment and control model;
- required evidence before production adoption.

Produce a Jobs-to-be-Done matrix with:

- job;
- persona;
- segment;
- trigger;
- current workaround;
- pain frequency;
- severity;
- economic impact;
- security/compliance impact;
- satisfaction with alternatives;
- willingness to switch;
- confidence.

If direct interview evidence is unavailable, state that limitation prominently and produce a detailed interview guide for subsequent validation.

## Research Workstream 2 — Competitor and Substitute Matrix

For every major product capture:

- target user and positioning;
- license and open-core boundary;
- governance and activity;
- language and architecture;
- deployment modes and mandatory dependencies;
- providers, endpoints, protocols, modalities, and passthrough;
- streaming and realtime fidelity;
- routing, balancing, retries, fallback, hedging, circuit breaking, and health;
- rate, token, concurrency, cost, and budget limits;
- identity, authentication, authorization, tenancy, and secrets;
- privacy, content retention, DLP, redaction, residency, and audit;
- caching semantics and isolation;
- observability, OpenTelemetry, logs, metrics, traces, and cost accuracy;
- control plane, GitOps, policy, hot reload, rollout, rollback, and drift;
- HA, multi-region, fleet management, upgrades, backup, and DR;
- benchmarks, methodology, reproducibility, and equivalent-feature fairness;
- security posture, disclosure, SBOM, signed releases, and provenance;
- onboarding, diagnostics, documentation, and migration experience;
- enterprise packaging, price, and support;
- independently verifiable adoption;
- recurring issues and complaints;
- likely response to the proposed wedge.

Output:

- factual matrix;
- table-stakes list;
- adoption-trigger list;
- moat candidates;
- gap heatmap;
- “why not use this competitor?” analysis;
- build versus contribute versus integrate versus compete recommendation.

## Research Workstream 3 — Candidate Wedges

Test, rank, and attempt to falsify:

1. Verifiable AI traffic governance:
   deterministic policy decisions, explainable routing, signed configuration provenance, and tamper-evident receipts.
2. Reliability-correct routing:
   capability-aware translation, safe retries, semantic fallback contracts, conformance certification, and explicit partial-stream behavior.
3. Ultra-light Rust data plane:
   tiny stateless binary, no mandatory database, sidecar and centralized modes, predictable tail latency, and optional control plane.
4. Privacy and sovereignty:
   metadata-only defaults, local redaction, egress restrictions, regional routing, retention enforcement, and air-gap support.
5. Identity-aware agent/tool traffic:
   human and workload identity propagation, least privilege, delegated authorization, and audit across model, MCP, tool, and A2A calls.
6. Auditable AI FinOps ledger:
   precise attribution, negotiated price versions, provider-usage reconciliation, budgets, anomaly detection, and verifiable usage evidence.
7. Embedded gateway engine:
   reusable Rust library or daemon for platforms that reject a large opinionated stack.
8. Compatibility and conformance:
   continuous provider contract tests across API changes, streaming, structured output, tool calls, context limits, and modalities.

Add other wedges discovered from evidence.

For each wedge report:

- buyer and operator;
- urgent job;
- evidence;
- alternatives;
- defensibility;
- incumbent copy risk;
- distribution advantage or disadvantage;
- smallest valuable proof;
- adoption friction;
- performance and security implications;
- commercial path;
- kill criterion;
- weighted score and confidence.

Recommend exactly one primary wedge and no more than two supporting advantages for the first product. Explicitly defer or reject the others.

## Research Workstream 4 — Protocol and Compatibility Reality

Research the current and likely two-to-three-year importance of:

- OpenAI Chat Completions, Responses, Realtime, embeddings, images, audio, batch, tool calls, and structured outputs;
- Anthropic Messages, streaming, tools, caching, and provider-specific semantics;
- Google Gemini/Vertex, Azure OpenAI/Foundry, Amazon Bedrock, and common compatible endpoints;
- vLLM, SGLang, Ollama, llama.cpp, TGI, and other self-hosted servers;
- SSE, WebSocket, HTTP/2, HTTP/3, gRPC, and JSON-RPC;
- MCP, A2A, and emerging agent/tool protocols;
- Kubernetes Gateway API and Inference Extension;
- OpenTelemetry GenAI semantic conventions.

For each, specify:

- customer demand;
- maturity and stability;
- normalization feasibility;
- lossy translation risks;
- passthrough requirements;
- capability-negotiation needs;
- version and deprecation strategy;
- conformance-test requirements;
- first-release inclusion, later inclusion, or rejection.

## Research Workstream 5 — Architecture Options

Compare at least:

- one stateless gateway process;
- separate data plane and optional control plane;
- sidecar or node-local agent plus central policy/config;
- Envoy extension or external processor;
- standalone Rust proxy;
- embeddable Rust engine;
- Kubernetes-native gateway;
- hybrid managed control plane with customer-hosted data plane.

Evaluate:

- hot-path dependencies;
- blast radius;
- latency;
- resource use;
- failure modes;
- offline operation;
- scale;
- configuration consistency;
- extensibility;
- upgrade safety;
- developer experience;
- enterprise operations;
- business model;
- compatibility with an open-source core.

Answer:

- what must be synchronous;
- what can be asynchronous;
- what is optional;
- what should integrate with an existing system instead of being built;
- behavior when storage, telemetry, cache, policy, control plane, or provider fails;
- how signed configuration snapshots are validated, activated, canaried, and rolled back;
- whether plugins should use WASM, external processing, RPC, compiled modules, or no general plugin API initially.

## Research Workstream 6 — Extremely Lightweight and Very Low Latency

Do not select performance targets arbitrarily. Derive them from user needs and competitor baselines.

Define and research:

- idle RSS;
- memory per active stream;
- binary and image size;
- cold and warm startup;
- p50/p95/p99/p99.9 added latency;
- time-to-first-byte and time-to-first-token overhead;
- stream inter-chunk delay;
- throughput and concurrency;
- CPU time and allocations;
- connection reuse and upstream pools;
- TLS, DNS, compression, and serialization costs;
- config reload impact;
- observability-on/off difference;
- overhead of policy, DLP, caching, and guardrails;
- overload, bounded queues, load shedding, and recovery;
- long-running stream behavior.

Design an honest reproducible benchmark:

- pinned hardware/software;
- raw data;
- repeated trials and confidence intervals;
- warmup and duration;
- equivalent security and telemetry settings;
- saturation and non-saturation;
- error rates;
- coordinated-omission avoidance;
- loopback and realistic network topologies;
- multiple payload shapes and streaming cases;
- regression thresholds in CI.

Explain which Rust techniques may help, but do not assume they are required:

- bounded async concurrency;
- careful allocation and buffer ownership;
- streaming without full-body buffering;
- connection pooling;
- cancellation propagation;
- zero-copy opportunities;
- parser and serialization choices;
- lock avoidance;
- runtime tuning;
- profile-guided optimization where justified.

Reject micro-optimizations that damage correctness, safety, or maintainability.

## Research Workstream 7 — Reliability and Correct Failure Semantics

Specify:

- deadline propagation and attempt budgets;
- timeout taxonomy;
- retry eligibility and idempotency;
- duplicate-generation and duplicate-billing risks;
- pre-stream and mid-stream failure behavior;
- cancellation and client disconnect;
- fallback capability, quality, safety, context, tool, schema, region, and compliance checks;
- circuit breaking and outlier detection;
- adaptive concurrency and admission control;
- load shedding and bounded queues;
- health by provider, model, endpoint, region, account, and credential;
- fail-open/closed per control;
- hedging and duplicate-cost risk;
- graceful shutdown and draining;
- configuration canary and rollback;
- distributed quota consistency;
- cache correctness and isolation;
- delayed or missing provider usage;
- multi-region and split-brain;
- backup, restore, RPO, and RTO;
- chaos, fault-injection, soak, and upgrade testing.

Create a failure-semantics matrix covering every request phase and every dependency.

## Research Workstream 8 — Secure and Private by Default

Use current versions of:

- OWASP GenAI Security guidance;
- NIST AI RMF and Generative AI Profile;
- NIST SSDF;
- OpenSSF OSPS Baseline and Scorecard;
- SLSA;
- applicable privacy, cyber, sector, and AI regulations for recommended target markets.

Threat-model:

- internet client;
- compromised internal client;
- malicious tenant;
- insider and privileged operator;
- compromised control plane;
- compromised provider;
- malicious plugin;
- dependency and CI/CD compromise;
- update-channel compromise;
- telemetry and storage compromise.

Cover:

- authentication, workload identity, end-user identity, and mTLS;
- OIDC/OAuth2, SPIFFE/SPIRE, API keys, and short-lived credentials;
- RBAC/ABAC, policy-as-code, separation of duties, approvals, and break glass;
- provider secrets, KMS/HSM/Vault, rotation, envelope encryption, and zeroization;
- tenant isolation across memory, cache, quotas, telemetry, and storage;
- private networking, egress allowlists, proxies, DNS, SSRF, and regional pinning;
- request smuggling, header confusion, parser differentials, oversized input, and denial of service;
- content classification, redaction, DLP, encryption, retention, and deletion;
- audit tamper resistance;
- secure admin/debug interfaces;
- signed releases, SBOM, provenance, dependency review, vulnerability response, and emergency updates;
- fuzzing, property tests, differential tests, unsafe-code policy, static checks, secret scanning, and pen testing.

State which controls must be free and open source to preserve trust.

Do not claim the gateway solves prompt injection or model safety. Define exactly what can and cannot be enforced at this layer.

## Research Workstream 9 — Logging, Telemetry, Audit, and FinOps

Design a privacy-first model with three separated classes:

1. operational telemetry;
2. security/compliance audit;
3. usage/financial ledger.

Default to metadata-only telemetry. Treat prompts and responses as sensitive content requiring explicit, scoped, access-controlled, encrypted, retention-bound capture.

Research lifecycle events:

- request arrival and admission;
- authentication and authorization;
- quota/budget decision;
- policy decision;
- cache decision;
- route candidates and selection reason;
- upstream attempt lifecycle;
- retry/hedge/fallback;
- stream start, completion, cancellation, or abort;
- guardrail/DLP decision;
- usage estimation/provider report/correction;
- price and invoice reconciliation;
- configuration activation/rejection;
- retention/export action.

Evaluate fields for:

- request, trace, span, correlation, and idempotency identifiers;
- pseudonymous tenant/project/environment/app/workload/user/credential;
- requested and resolved model/provider/deployment/region;
- policy/config/route/price/tokenizer/software versions;
- wall and monotonic timing;
- queue/auth/policy/cache/connect/TLS/first-byte/first-token/stream/total duration;
- bytes and provider-specific usage units;
- estimated, reported, corrected, and reconciled usage;
- attempt, retry, fallback, circuit, and route reasons;
- gateway/provider error taxonomy and retryability;
- cache namespace/policy/tenant boundary;
- security decision, classification, redaction, and reason;
- partial stream, disconnect, cancellation, and uncertainty;
- sampling and content-capture policy.

Define:

- OpenTelemetry traces, metrics, logs, and exports;
- stable versus experimental semantic conventions;
- cardinality budgets;
- sampling;
- dropped-telemetry behavior;
- retention;
- tenant export/deletion/legal hold;
- SIEM and data-lake integration;
- SLO and burn-rate metrics;
- cost attribution and invoice reconciliation;
- behavior when observability storage is unavailable.

## Research Workstream 10 — Enterprise Readiness

Identify what is truly required versus procurement theater:

- organization/tenant/project/environment hierarchy;
- SSO, SAML, OIDC, SCIM;
- RBAC/ABAC and delegated administration;
- policy simulation, approvals, rollout, and rollback;
- developer self-service under central policy;
- model catalog, aliases, ownership, and capability contracts;
- private networking and air-gap;
- residency and retention;
- HA, multi-region, backup, DR, and upgrades;
- fleet health and configuration drift;
- SIEM, OpenTelemetry, audit, and cost exports;
- chargeback/showback and budgets;
- compliance evidence and support bundles;
- LTS, SLA, support, and vulnerability response.

Classify every item:

- open-source core;
- optional open-source module;
- commercial/enterprise;
- external integration;
- rejected.

## Research Workstream 11 — Zero-Complexity Experience

Design the target first-use path:

1. obtain one verified binary or minimal image;
2. validate one small configuration;
3. reference secrets safely;
4. run with no mandatory database;
5. change one SDK base URL;
6. obtain health/readiness and an explainable route decision;
7. upgrade and roll back safely.

Research:

- Linux, macOS, Windows if justified, Docker, Kubernetes, systemd, edge, and air-gap;
- strict config schema, lint, format, dry-run, diff, explain, migration, and compatibility;
- safe hot reload;
- useful diagnostics without leaking secrets;
- provider conformance checks;
- offline local development;
- quickstart usability tests;
- optional UI versus CLI/API/GitOps;
- migration from LiteLLM, direct SDKs, Envoy/Kong, and custom proxies.

Set a measurable time-to-first-success target only after comparing realistic alternatives.

## Research Workstream 12 — Open Source and Business

Compare:

- Apache-2.0;
- MPL-2.0;
- AGPLv3;
- dual licensing;
- other defensible options.

Recommend:

- license and commercial boundary;
- DCO versus CLA;
- governance and roadmap transparency;
- RFC/ADR process;
- contribution and review model;
- code of conduct;
- security disclosure and embargo process;
- semantic versioning, compatibility, deprecation, LTS, and support windows;
- signed releases, checksums, SBOM, provenance, and reproducibility;
- benchmark transparency;
- OpenSSF maturity path;
- trademark and naming checks;
- possible managed control plane/customer-hosted data plane;
- managed service, support, compliance, and fleet-management options;
- pricing unit and the conflict between usage pricing and cost-control positioning;
- design-partner and adoption strategy.

The open-source edition must remain genuinely useful, self-hostable, secure, and free of artificial reliability limitations.

## Required Scoring Model

Score out of 100:

| Dimension | Weight |
| --- | ---: |
| Problem intensity and frequency | 15 |
| Differentiation and defensibility | 15 |
| Enterprise trust and reliability | 12 |
| Security, privacy, and governance | 12 |
| Developer/operator experience | 10 |
| Performance and resource efficiency | 8 |
| Interoperability and future resilience | 8 |
| Open-source adoption potential | 8 |
| Commercial viability | 7 |
| Execution feasibility | 5 |

For each dimension include:

- score;
- support;
- contradiction;
- confidence;
- missing evidence;
- what would raise or lower it.

Do not force 95/100. Give a score range if uncertainty is material.

## Required Final Deliverable

Write a self-contained English research report with:

1. Executive verdict.
2. Recommended action: proceed, narrow, pivot, contribute, or stop.
3. Honest score and confidence interval.
4. Market and company-needs analysis by segment.
5. Jobs-to-be-Done and pain matrix.
6. Buyer/operator/approver/blocker map.
7. Competitor and substitute matrix.
8. Table stakes, adoption triggers, and moat candidates.
9. Gap heatmap.
10. Candidate-wedge scorecards.
11. One primary wedge and no more than two supporting advantages.
12. Rejected scope and explicit non-goals.
13. One-sentence product positioning.
14. Protocol and compatibility strategy.
15. Recommended architecture and rejected alternatives.
16. Performance targets with derivation and benchmark design.
17. Reliability and failure-semantics matrix.
18. Threat model and security-control map.
19. Privacy-first logging, audit, telemetry, and FinOps design.
20. Enterprise feature classification.
21. Zero-complexity installation and operations journey.
22. OSS license, governance, release, and supply-chain plan.
23. Commercial and adoption strategy.
24. Validation experiments and design-partner program.
25. Phased roadmap:
    - research proof;
    - technical spike;
    - minimal useful open-source data plane;
    - production hardening;
    - enterprise control plane only after validation.
26. Risk register.
27. Kill criteria and pivot options.
28. Questions requiring direct interviews.
29. Claim ledger.
30. Annotated bibliography with direct links and dates.

## Output Quality Rules

- Be specific and technically deep.
- Prefer tables where they improve comparison.
- Use diagrams only when they clarify architecture or failure flow.
- Mark facts, estimates, and recommendations.
- Cite every material external claim.
- Include conflicting evidence.
- Explain tradeoffs rather than declaring a universal best practice.
- Do not use vague phrases such as enterprise-grade, secure, scalable, zero downtime, or lightweight without measurable definitions.
- Do not recommend a giant feature bundle merely to improve the score.
- End with the smallest next experiment that could disprove the recommended thesis.

---

End of prompt.
