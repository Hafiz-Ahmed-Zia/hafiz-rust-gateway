# Pro-Model Research Comparison

**Status:** Ready; complete after the full external report is received

## 1. Source preservation

- Save the original answer without edits under `external/`.
- Save or export every cited URL and the research date.
- Record model name/version if available, run date, prompt version, and whether browsing was enabled.
- Do not paste external conclusions directly into the PRD.

## 2. Executive comparison

| Question | Independent baseline | Pro report | Resolution | Evidence confidence |
| --- | --- | --- | --- | --- |
| Primary segment | TBD |  |  |  |
| Most painful job | TBD |  |  |  |
| Primary wedge | Streaming/failure verification + private minimal data plane hypothesis |  |  |  |
| Strongest incumbent | Segment-dependent |  |  |  |
| Why a new project | Unproven |  |  |  |
| Commercial boundary | Unproven |  |  |  |
| Opportunity score | 67.9/100 provisional |  |  |  |
| Recommendation | Research + narrow spike |  |  |  |

## 3. Claim-level comparison

Create one row per material claim. Do not combine unrelated claims.

| ID | Claim | Baseline position/source | Pro position/source | Agree/conflict | Freshness | Source quality | Action |
| --- | --- | --- | --- | --- | --- | --- | --- |
| CMP-001 | TensorZero maintenance status | Official site/repo: no longer maintained, archived June 2026 |  |  | 2026-08 check | Primary |  |
| CMP-002 | Rust/latency is a differentiator | Already occupied; not sufficient |  |  | Current | Multiple official vendors |  |
| CMP-003 | Enterprise scaling pain exists | Surveys support; direct buyer evidence absent |  |  | 2025-2026 | Primary studies |  |
| CMP-004 | Streaming correctness is purchase-critical | Hypothesis only |  |  | N/A | Needs interviews |  |
| CMP-005 | Single-binary/no-control-plane is purchase-critical | Hypothesis only |  |  | N/A | Needs interviews |  |

## 4. Required conflict checks

- Competitor still active, licensed, and maintained?
- Feature open source or enterprise-only?
- Self-hosted feature equal to hosted feature?
- Performance comparison uses equivalent auth, TLS, policy, telemetry, and payloads?
- “Provider support” means native, compatible subset, or pass-through?
- Cost data current and sourced?
- Survey sample, geography, role, and sponsor disclosed?
- Customer quote independent or vendor marketing?
- Legal/regulatory statement current in the intended jurisdiction?
- Any claim inferred from absence of search results?

## 5. Wedge tournament

Score each candidate from 1-5 with evidence and a disconfirming test.

| Candidate | Pain | Urgency | Incumbent gap | Defensibility | Feasibility | Distribution | Total /30 | Kill test |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| Verified streaming/failure semantics |  |  |  |  |  |  |  | 5 interviews say existing behavior is adequate |
| Privacy-first standalone data plane |  |  |  |  |  |  |  | buyers accept hosted/control-plane dependency |
| Open conformance infrastructure |  |  |  |  |  |  |  | teams do not use compatibility evidence to choose |
| Policy-as-code enterprise boundary |  |  |  |  |  |  | existing API gateways satisfy policies |
| TensorZero migration/continuity path |  |  |  |  |  |  | addressable users too few or migrated already |

## 6. Architecture reconciliation

Compare:

- data/control-plane boundary;
- `hyper`/Tokio versus Pingora versus alternatives;
- provider normalization model;
- streaming state machine;
- retry/fallback commitment point;
- policy engine and reload model;
- local/distributed state semantics;
- observability privacy;
- extension/plugin isolation;
- deployment and upgrade path.

For every recommendation, mark **adopt**, **spike**, **defer**, or **reject**.

## 7. Final gate outcome

| Gate | Pass condition | Evidence attached | Outcome |
| --- | --- | --- | --- |
| R0 Market pain | repeated costly problem, clear owner |  | Not evaluated |
| R1 Wedge | adoption reason beyond Rust/speed |  | Not evaluated |
| R2 Trust | credible secure/private/reliable design |  | Not evaluated |
| R3 Technical proof | conformance, streaming, resource proof |  | Not evaluated |
| R4 Design partners | three teams, same use case |  | Not evaluated |

## 8. Required synthesis output

1. Corrected factual baseline.
2. Claims rejected and why.
3. Unknowns that neither report answers.
4. Interview questions created by those unknowns.
5. Selected wedge or rejection.
6. PRD changes with requirement IDs.
7. Architecture spikes.
8. Updated weighted score with uncertainty.
9. Proceed, narrow, pivot, contribute, or stop decision.
