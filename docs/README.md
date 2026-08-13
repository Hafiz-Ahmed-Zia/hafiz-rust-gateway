# Documentation Index

This index separates validated decisions from hypotheses. A document marked **Draft** or **Proposed** is not an implementation contract.

| Area | Document | Status | Purpose |
| --- | --- | --- | --- |
| Product | `product/PRD.md` | Draft v0.1 | Evidence-gated requirements and scope |
| Product | `product/REQUIREMENTS.md` | Draft | Traceable requirements and validation method |
| Research | `research/01-enterprise-ai-gateway-research-plan.md` | Active | Research program, gates, and kill criteria |
| Research | `research/02-pro-model-master-research-prompt.md` | Ready | External Pro-model research instruction |
| Research | `research/03-independent-baseline-research.md` | Baseline | Independent market and technical findings |
| Research | `research/04-pro-research-comparison-template.md` | Ready | Claim-by-claim synthesis method |
| Research | `research/05-claim-ledger.md` | Active | Evidence, confidence, and unresolved claims |
| Research | `research/06-design-partner-interview-guide.md` | Ready | Non-leading customer discovery interview |
| Architecture | `architecture/ARCHITECTURE.md` | Proposed | Data-plane/control-plane boundaries and options |
| Architecture | `architecture/STREAMING-CONTRACT.md` | Proposed | Streaming state machine and failure behavior |
| Architecture | `architecture/RELIABILITY-CONTRACT.md` | Proposed | Timeouts, retries, fallback, overload, and shutdown |
| Architecture | `architecture/COMPATIBILITY.md` | Proposed | Protocol and provider compatibility tiers |
| Security | `security/THREAT-MODEL.md` | Draft | Assets, adversaries, boundaries, and invariants |
| Security | `security/SECURE-DEFAULTS.md` | Proposed | Default behaviors that must hold |
| Security | `security/SECURITY-POLICY-PROPOSAL.md` | Awaiting approval | Proposed future root security policy |
| Benchmarks | `benchmarks/BENCHMARK-SPEC.md` | Proposed | Reproducible measurement and regression rules |
| Conformance | `conformance/CONFORMANCE-PLAN.md` | Proposed | Protocol fixtures, differential tests, and matrix |
| Decisions | `decisions/ADR-0001-git-workflow.md` | Accepted | GitHub Flow and branch policy |
| Decisions | `decisions/ADR-0002-network-stack-spike.md` | Proposed | Hyper/Tokio versus Pingora evaluation |
| Governance | `governance/BRANCHING.md` | Accepted | Branch, review, environment, and release rules |
| Governance | `governance/LICENSING.md` | Proposed | Apache-2.0 recommendation and decision gate |
| Roadmap | `roadmap/FOUNDATION-ROADMAP.md` | Active | Research-to-production gates |

## Decision authority

The current precedence is:

1. accepted ADRs and owner-approved security policy;
2. approved PRD requirements;
3. architecture and conformance contracts;
4. research evidence and claim ledger;
5. roadmap and issue descriptions.

When documents conflict, do not silently choose. Record the conflict and resolve it in an ADR or PRD revision.
