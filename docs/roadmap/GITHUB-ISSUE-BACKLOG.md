# Initial GitHub Issue Backlog

**Status:** Milestone 1 issues 1-3 seeded; later work remains gate-controlled
**Scope rule:** Every issue must stay within its milestone and reference the named requirement or
research-spike label. Passing an issue does not by itself authorize a product claim.

## Milestone 1 — Evidence and technical proof

1. **Synthesize preserved external research into the claim ledger**
   Labels: `research`, `evidence` · Requirements: MKT-001, MKT-002
   Accept when every decision-relevant external claim has a primary source, confidence level,
   conflict note, and explicit product consequence.
2. **Implement versioned conformance fixture schema**
   Labels: `conformance`, `protocol` · Requirements: API-001, API-002
   Accept when fixtures have a documented schema version, bounded fields, validation failures, and
   at least one synthetic streaming example.
3. **Build bounded deterministic fake HTTP provider**
   Labels: `testkit`, `reliability` · Requirements: STR-001 through STR-004, REL-001
   Accept when tests can control fragmentation, delay, malformed events, pre-commit disconnects,
   post-commit disconnects, and hard size/event limits without external services.

### Network-stack decision

4. **Run the equivalent Hyper/Tokio data-plane spike**
   Labels: `research-spike`, `networking` · Requirements: ADR-0002, STR-001 through STR-004
   Accept when the implementation passes the shared conformance/fault suite and publishes raw
   resource and latency evidence under the benchmark contract.
5. **Run the equivalent Pingora data-plane spike**
   Labels: `research-spike`, `networking` · Requirements: ADR-0002, STR-001 through STR-004
   Accept under the same controls and evidence format as the Hyper/Tokio spike.
6. **Decide the production network stack**
   Labels: `adr`, `architecture` · Requirements: ADR-0002, PER-001, PER-002
   Accept when correctness failures are resolved first and an accepted ADR records the comparable
   results, tradeoffs, rejected option, and rollback implications.

## Milestone 2 — Narrow public alpha

### Usable data plane

7. **Implement one bounded OpenAI-compatible ingress subset**
   Labels: `feature`, `protocol` · Requirements: API-001, API-002, SEC-005
   Accept when supported fields and deviations are versioned, unsupported inputs fail explicitly,
   and fixture coverage includes fragmented streaming and malformed input.
8. **Add strict local configuration check and doctor commands**
   Labels: `feature`, `security`, `operations` · Requirements: SEC-001, SEC-003, OPS-002
   Accept when ambiguous/insecure configuration fails closed and a new evaluator can diagnose a
   synthetic setup without a database, broker, or hosted service.
9. **Enforce resource, deadline, and pre-commit recovery budgets**
   Labels: `reliability`, `security` · Requirements: SEC-005, REL-001, STR-003, STR-004
   Accept when integration and overload tests prove all relevant queues/buffers are bounded and no
   transparent recovery happens after commitment.

### Security and operations

10. **Harden target validation, headers, and content-safe telemetry**
    Labels: `security`, `observability` · Requirements: SEC-002, SEC-004, OBS-001
    Accept when SSRF/DNS-rebinding, smuggling/normalization, and canary-secret tests pass with
    prompt/response capture disabled by default.
11. **Prove cancellation, reload, overload, drain, and shutdown behavior**
    Labels: `reliability`, `operations` · Requirements: STR-002, REL-002, REL-003
    Accept when process-level fault tests demonstrate bounded and documented terminal behavior.
12. **Add fuzz, property, and fault-injection CI profiles**
    Labels: `testing`, `security` · Requirements: SEC-005, STR-001 through STR-004
    Accept when deterministic regression fixtures exist for every validated failure and scheduled
    jobs retain useful bounded artifacts.

### Release evidence

13. **Publish compatibility matrix and reproducible benchmark evidence**
    Labels: `documentation`, `performance` · Requirements: API-002, PER-001, PER-002
    Accept when raw data, environment, controls, variance, deviations, and known limitations are
    independently reproducible.
14. **Produce signed release artifacts, checksums, SBOM, and provenance**
    Labels: `release`, `supply-chain` · Requirements: SUP-001
    Accept when a clean evaluator can verify every artifact back to the tagged source revision.
15. **Run the public-alpha evaluator journey**
    Labels: `usability`, `release` · Requirements: OPS-001, OPS-002
    Accept when the named clean-environment study passes and the release documentation makes no
    universal compatibility or enterprise-readiness claim.

## Milestone 3 — Validated adoption and hardening

Pilot identity, quota, HA, deployment, support, and commercial-boundary issues are created only
after three credible design partners validate the same primary workflow.

## Remote seeding rule

Create labels and issues only when their stage is authorized. Do not bulk-create Stage 2 or Stage 3
work until the preceding gate remains plausible; close or rewrite obsolete issues after a
documented pivot instead of leaving misleading scope open.
