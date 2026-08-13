# Independent Baseline Research

**Status:** Baseline for comparison with external Pro-model research  
**Cut-off:** 2026-08-13  
**Method:** Current public primary/official sources where available; vendor feature claims are not treated as independent proof of superiority.

## 1. Executive decision

### Current recommendation

**Proceed with research and a narrow technical proof; do not start building a broad enterprise platform.**

The market need for AI governance, secure model access, cost control, and reliability is real, but the gateway category is crowded and increasingly bundled into incumbent API platforms. A new project cannot earn adoption merely through Rust, a unified API, a single binary, low latency, retries, fallbacks, token limits, or dashboards.

The strongest provisional thesis for Hafiz Rust Gateway is:

> A privacy-first, independently deployable data plane that makes streaming correctness and failure behavior verifiable under real production faults, while keeping the hot path free of mandatory databases and hosted control-plane dependencies.

This is a **candidate wedge**, not yet a validated moat.

### Provisional opportunity score

| Dimension | Weight | Current score | Weighted contribution | Confidence |
| --- | ---: | ---: | ---: | --- |
| Problem intensity/frequency | 15 | 7.5 | 11.25 | Medium; survey evidence, no interviews |
| Differentiation/defensibility | 15 | 5.5 | 8.25 | Low |
| Enterprise trust/reliability | 12 | 7.5 | 9.00 | Medium as requirement, low as project proof |
| Security/privacy/governance | 12 | 8.0 | 9.60 | Medium |
| Developer/operator experience | 10 | 7.0 | 7.00 | Low |
| Performance/resource efficiency | 8 | 7.5 | 6.00 | Low until benchmark |
| Interoperability/future resilience | 8 | 6.5 | 5.20 | Medium |
| Open-source adoption potential | 8 | 6.5 | 5.20 | Low/medium |
| Commercial viability | 7 | 4.5 | 3.15 | Low |
| Execution feasibility | 5 | 6.5 | 3.25 | Medium |
| **Total** | **100** |  | **67.9/100** | **Low/medium** |

The score is intentionally below 9.5/10. A 95+ score is impossible without direct demand, design partners, a proven wedge, reproducible technical evidence, and a sustainable open-source/commercial model.

## 2. Enterprise demand signals

### Adoption is broad, but scaling is still difficult

McKinsey's 2025 global survey reports 88% of respondents using AI in at least one business function, while nearly two-thirds say their organizations have not begun enterprise-wide scaling. Only 39% report any enterprise-level EBIT impact, and 51% of respondents at AI-using organizations report at least one negative consequence. This supports a need for operational controls, but does not prove that a new independent gateway is the desired purchase.

IBM's 2025 CEO study reports that only 16% of AI initiatives had scaled enterprise-wide and 50% of surveyed CEOs said rapid investment produced disconnected technology. This supports consolidation and governance pain, while also warning that buyers may prefer platforms that reduce tool count.

EIOPA's 2026 survey of 347 insurers across 25 countries says nearly two-thirds were actively using GenAI but most remained at proof-of-concept stage. Regulated organizations are a plausible segment for privacy and governance, but their procurement and compliance requirements can lengthen adoption and favor established vendors.

### Demand conclusion

There is credible demand for visibility, control, privacy, reliability, and cost accountability. There is not yet evidence that “a new Rust gateway” is the missing solution. Interviews must determine whether teams are blocked by:

- existing gateway complexity;
- mandatory SaaS/control-plane dependence;
- content retention and telemetry privacy;
- incorrect or opaque streaming failure behavior;
- provider compatibility churn;
- distributed quota and attribution gaps;
- security review friction;
- or something else entirely.

## 3. Competitive reality

| Product/group | What current documentation establishes | Implication |
| --- | --- | --- |
| LiteLLM | 100+ providers, OpenAI formats, auth, multi-tenant spend, budgets, rate limits, retries/fallbacks, admin UI | Unified providers and basic governance are table stakes |
| Envoy AI Gateway | Kubernetes/Envoy-native routing, failover, upstream auth, rate limiting, policy, observability | Strong community/proxy foundation owns cloud-native segment |
| Kong AI Gateway | Provider normalization, credentials, governance, prompt guards, semantic cache, observability/cost | Established API-gateway buyers can extend an incumbent |
| Apache APISIX | Multi-provider load balancing, retries/fallbacks, token limits, prompt controls, logging/auditing | Fully open-source plugin ecosystems compete on breadth |
| Portkey | Open-source gateway, routing, fallback, retries, circuits, cache, guardrails, budgets, limits | Gateway orchestration and guardrails are already packaged |
| Bifrost | Open-source high-performance gateway with single-endpoint/provider breadth and enterprise features | Raw speed and “faster than LiteLLM” positioning are occupied |
| Cloudflare AI Gateway | Managed logging, caching, rate limits, billing and provider access | Global managed convenience is difficult to beat directly |
| Azure API Management | Models, self-hosted endpoints, MCP/A2A, token quotas, semantic cache, managed identity, monitoring | Enterprise suites can bundle identity/governance deeply |
| Traefik Hub AI Gateway | Thin AI control layer, OpenAI compatibility, model policy, content guard, GenAI telemetry | “Thin layer over existing gateway” is another substitute |
| TensorZero (archived) | Was a Rust gateway plus observability/evaluation/optimization; official site now says no longer maintained | Rust/performance existed, and project sustainability/product-market fit are critical risks |

### TensorZero lesson

TensorZero is especially important because it directly invalidates the thesis that a technically serious Rust gateway plus a broad LLMOps platform is automatically a durable company or project. Its repository was archived in June 2026, and the official site states that the project is no longer maintained. The last release also referenced a high-risk gateway vulnerability. The correct lesson is not “competition disappeared”; it is:

1. maintenance and security response are product features;
2. a broad gateway/observability/evaluation scope can create a difficult sustainability model;
3. enterprise users need a credible continuity and forkability story;
4. Hafiz Rust Gateway should remain narrow until adoption proves expansion;
5. the project must research why TensorZero wound down before choosing a commercial model.

## 4. Table stakes, triggers, and candidate moat

### Table stakes

- OpenAI-compatible entry point
- multiple upstreams/providers
- credential mediation
- routing, timeouts, retries, fallbacks, circuit breaking
- rate/concurrency/token limits
- usage/cost metadata
- metrics, traces, logs
- Docker/Kubernetes path
- basic policy and model allowlists

### Possible adoption triggers

- no prompt/response capture by default and strong evidence of that behavior;
- no mandatory hosted control plane or database;
- ten-minute local evaluation and strong diagnostics;
- precise stream cancellation/backpressure/failure behavior;
- stable configuration reload and last-known-good operation;
- conformance matrix rather than vague provider support;
- signed, reproducible, auditable releases;
- easy migration from an unmaintained or over-complex incumbent.

### Candidate moat

The defensible unit may be the **verification system**, not the proxy implementation:

- open protocol fixtures and differential provider tests;
- streaming state-machine/property tests;
- chaos/failure-semantics suite;
- privacy canaries proving sensitive content is not emitted;
- equivalent-control benchmark harness;
- compatibility history by release/provider;
- configuration and policy provenance.

These assets can compound over time and are harder to copy credibly than a feature checklist, but only if users value them and the project maintains them continuously.

## 5. Technical baseline

### Rust is appropriate, not unique

Rust provides memory-safety and predictable performance benefits. Cloudflare's Pingora experience demonstrates that Rust proxy infrastructure can operate at immense scale, while `hyper` provides a mature asynchronous HTTP/1/2 foundation and `rustls` provides memory-safe modern TLS. None of this proves Hafiz Rust Gateway will be safe or fast; architecture, dependencies, limits, testing, and operations decide that.

### Network stack remains undecided

- `hyper`/Tokio likely provides more direct control for JSON and typed event translation.
- Pingora provides hardened proxy/load-balancing/restart primitives.
- A controlled spike must compare equivalent features and failure behavior.

### OpenAI compatibility has expanded

The current official OpenAI API includes the Responses API and typed SSE streaming events, with WebSocket mode available for persistent interactions. Supporting only legacy-style Chat Completions is insufficient for a future-facing compatibility claim, but full Responses and Realtime breadth would overwhelm v1. The correct approach is an explicit subset and conformance matrix.

### Observability must be privacy aware

OpenTelemetry GenAI conventions are actively evolving. Current OTel guidance keeps prompt content and tool arguments out of telemetry by default and makes content capture opt-in. Hafiz Rust Gateway should follow that posture and isolate experimental semantic attributes from its stable internal event model.

## 6. Security baseline

The gateway is a high-value trust boundary holding provider credentials and potentially handling proprietary prompts, outputs, tools, and tenant identity. Relevant risk classes include:

- auth bypass and tenant confusion;
- upstream credential leakage;
- SSRF through configurable endpoints;
- request smuggling/header confusion;
- unbounded JSON/SSE/decompression/tokenization work;
- prompt/response leakage through telemetry;
- policy bypass during reload or failure;
- malicious/compromised providers;
- retry amplification and model denial of service;
- dependency/build compromise;
- unsafe plugins/guardrails;
- administrative insider misuse.

OWASP GenAI risks and NIST's Generative AI Profile are useful control lenses, not certifications. Workload identity standards such as SPIFFE may be valuable for enterprise deployments but should remain optional.

## 7. Zero-complexity interpretation

“Zero complexity” cannot mean zero configuration or zero operational responsibility. It should mean:

- no external service for the core happy path;
- secure minimal example;
- offline validation;
- path-specific diagnostics;
- atomic reload and rollback;
- explicit defaults;
- one documented way before multiple deployment variants;
- safe degradation when optional systems fail.

The evaluator experience should be measured with people unfamiliar with the repository, not by maintainers.

## 8. Commercial and open-source risk

The project needs separate answers for:

1. why users adopt the open-source data plane;
2. why a subset pays without feeling that core trust/security has been withheld;
3. how provider churn, security response, conformance infrastructure, and releases are funded;
4. how users continue safely if the founding company stops.

Possible paid boundaries include fleet management, enterprise identity integrations, policy distribution, managed conformance monitoring, compliance evidence, and support. Core routing correctness, secure defaults, conformance tests, and local observability should not be artificially crippled.

## 9. Highest-priority validation questions

1. Do enterprise teams experience stream/failure correctness as an expensive problem, or merely an engineering detail?
2. Is local/private data-plane independence a purchase trigger or just a preference?
3. What specifically makes LiteLLM/Envoy/Kong/APISIX/Portkey/Bifrost unacceptable in the target environment?
4. Will buyers add a separate gateway, or insist on their existing API gateway/cloud platform?
5. Which content and metadata may be retained, and who controls the exception process?
6. How much operational footprint is actually unacceptable?
7. Will three teams test the same narrow wedge?
8. What sustainability lesson should be taken from TensorZero's wind-down?

## 10. Decision after the Pro report

Do not average the two reports. Compare claims and source quality. A newer primary source overrides stale vendor facts; direct interview evidence outranks speculative feature demand; reproducible benchmarks outrank vendor charts. The final output must select one of:

- **Proceed:** same wedge has demand and technical proof path.
- **Narrow:** useful component, test suite, or migration gateway is stronger than a platform.
- **Pivot:** another boundary problem is more painful.
- **Contribute:** an existing gateway plus contributions solves the need.
- **Stop:** no defensible adoption reason.

## Sources

### Enterprise adoption and risk

- [McKinsey, The state of AI in 2025](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai)
- [IBM 2025 CEO study](https://newsroom.ibm.com/2025-05-06-ibm-study-ceos-double-down-on-ai-while-navigating-enterprise-hurdles)
- [EIOPA 2026 GenAI market survey](https://www.eiopa.europa.eu/publications/generative-ai-market-survey-outlook-use-cases-and-risk-management_en)
- [NIST AI Risk Management Framework and GenAI Profile](https://www.nist.gov/itl/ai-risk-management-framework)
- [OWASP GenAI/LLM Top 10](https://genai.owasp.org/llm-top-10/)

### Competitors and substitutes

- [LiteLLM documentation](https://docs.litellm.ai/)
- [Envoy AI Gateway](https://aigateway.envoyproxy.io/docs/)
- [Kong AI Gateway](https://docs.konghq.com/gateway/latest/ai-gateway/)
- [Apache APISIX AI Gateway](https://apisix.apache.org/ai-gateway/)
- [Portkey AI Gateway](https://portkey.ai/docs/product/ai-gateway)
- [Bifrost repository](https://github.com/maximhq/bifrost)
- [Cloudflare AI Gateway REST API](https://developers.cloudflare.com/ai-gateway/usage/rest-api/)
- [Azure API Management AI gateway](https://learn.microsoft.com/azure/api-management/genai-gateway-capabilities)
- [Traefik Hub AI Gateway](https://doc.traefik.io/traefik-hub/ai-gateway/overview)
- [TensorZero official status](https://www.tensorzero.com/)
- [TensorZero archived repository](https://github.com/tensorzero/tensorzero)

### Protocol, telemetry, and implementation

- [OpenAI streaming Responses](https://developers.openai.com/api/docs/guides/streaming-responses)
- [OpenTelemetry GenAI observability](https://opentelemetry.io/blog/2026/genai-observability/)
- [Pingora open-source overview](https://blog.cloudflare.com/pingora-open-source/)
- [hyper](https://hyper.rs/)
- [rustls](https://rustls.dev/)
- [SPIFFE concepts](https://spiffe.io/docs/latest/spiffe/concepts/)
