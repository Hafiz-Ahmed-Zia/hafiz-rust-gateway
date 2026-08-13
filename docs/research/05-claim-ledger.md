# Claim Ledger

**Status:** Active  
**Rule:** A source proves only what it directly establishes.

| ID | Claim | Evidence | Type | Confidence | Counterevidence/limitation | Decision impact |
| --- | --- | --- | --- | --- | --- | --- |
| CLM-001 | Enterprise AI use is broad but scaling remains immature | McKinsey 2025 survey: 88% use in at least one function; nearly two-thirds not scaling enterprise-wide | Primary survey | Medium/high | Respondent survey; not gateway-specific | Supports operational problem, not product choice |
| CLM-002 | Enterprise AI programs suffer fragmentation and low scaling | IBM 2025 CEO study: 50% disconnected tech, 16% scaled enterprise-wide | Primary sponsored study | Medium | CEO perceptions; vendor-sponsored | Supports simplicity/consolidation research |
| CLM-003 | Regulated insurers use GenAI but mostly remain in PoC | EIOPA 2026 survey, 347 undertakings/25 countries | Regulator survey | High for sampled sector | Insurance only | Supports regulated segment research |
| CLM-004 | Unified provider APIs, retries, budgets, and observability are table stakes | LiteLLM, Portkey, Kong, APISIX, Envoy, Azure docs | Official vendor/project docs | High for advertised capability | Edition and implementation quality vary | Reject these as standalone moat |
| CLM-005 | Rust and sub-millisecond claims are already present in category | TensorZero historical docs; Pingora; other high-performance gateways | Official/project claims | Medium | Benchmarks not independently reproduced | Performance must be equivalent-control proof |
| CLM-006 | TensorZero is no longer maintained | Official TensorZero site; GitHub repository archived June 12, 2026 | Primary current status | High | Reason/commercial analysis not fully primary | Elevates sustainability and continuity |
| CLM-007 | Privacy-first telemetry aligns with OTel direction | OTel GenAI guidance: content not captured by default | Standards/community official | High | Conventions evolving | Adopt metadata-only default |
| CLM-008 | Streaming correctness is an enterprise purchase trigger | None yet | Hypothesis | Low | Could be an implementation detail buyers ignore | Must validate before wedge selection |
| CLM-009 | No hosted control plane is an adoption trigger | Anecdotal self-hosted reports only | Hypothesis/anecdote | Low | Enterprises may prefer managed control | Must validate by segment |
| CLM-010 | Open verification assets can be defensible | Architectural inference | Inference | Low | Incumbents can add tests; users may not care | Test with design partners/community |
| CLM-011 | Apache-2.0 is best final license | Enterprise/patent-grant reasoning | Recommendation | Medium | Dual MIT/Apache is common in Rust; business model TBD | Owner/legal decision required |

## Source-quality labels

- **Primary current status:** authoritative project or regulator state.
- **Official vendor/project docs:** reliable for claimed feature presence, not superiority or customer value.
- **Primary sponsored study:** useful with methodology and sponsor bias noted.
- **Direct interview:** strong for the participant, not automatically generalizable.
- **Reproducible experiment:** strong for the published environment/workload.
- **Anecdote:** discovery input only.
- **Inference:** must be explicitly labeled and tested.
