# Requirements Register

**Status:** Approved foundation companion to PRD v0.2; validation states remain authoritative

| ID | Requirement | Priority | Validation | Gate | Status |
| --- | --- | --- | --- | --- | --- |
| MKT-001 | A specific segment reports repeated, expensive AI-gateway pain | Must | Interviews and evidence ledger | R0 | Unvalidated |
| MKT-002 | Three design partners prefer the same wedge over incumbent extension | Must | Written design-partner commitments | R1/R4 | Unvalidated |
| API-001 | Advertised OpenAI-compatible requests conform to published fixtures | Must | Differential conformance suite | R3 | Strict v1 schema and first synthetic fixture implemented; gateway differential proof pending |
| API-002 | Compatibility tier and deviations are published per endpoint/provider | Must | Matrix review and CI | R3 | Unimplemented |
| STR-001 | Streaming event order and terminal semantics are preserved | Must | State-machine and differential tests | R3 | Domain state machine plus fragmented loopback provider fixture implemented; gateway proof pending |
| STR-002 | Client cancellation propagates upstream promptly | Must | Integration/chaos test | R3 | Unimplemented |
| STR-003 | Stream buffers and queues are bounded under slow clients | Must | Backpressure load test | R3 | Unimplemented |
| STR-004 | No silent retry/fallback occurs after visible output | Must | Fault-injection test | R3 | Domain guard, deterministic scenario, and post-commit fixture test implemented; gateway proof pending |
| SEC-001 | Invalid or ambiguous security configuration fails closed | Must | Negative configuration suite | R2 | Unimplemented |
| SEC-002 | Secrets and content are absent from default logs/traces/errors | Must | Canary-secret and snapshot tests | R2 | Unimplemented |
| SEC-003 | TLS verification is on and insecure mode is explicit/dev-only | Must | Configuration and integration tests | R2 | Unimplemented |
| SEC-004 | Dynamic upstream targets are protected against SSRF | Must | URL/IP/DNS rebinding suite | R2 | Unimplemented |
| SEC-005 | Input, headers, JSON, SSE events, decompression, and queues have bounds | Must | Fuzz, property, and load tests | R2/R3 | Unimplemented |
| REL-001 | Retryability, deadlines, and budgets are explicit | Must | Failure-semantics matrix tests | R3 | Core budget evaluator implemented; integration proof pending |
| REL-002 | Configuration activation is validated, atomic, and versioned | Must | Concurrency and rollback tests | R3 | Unimplemented |
| REL-003 | Shutdown drains or terminates streams predictably | Must | Process integration tests | R3 | Unimplemented |
| OBS-001 | Metadata-only OTel telemetry is useful with content capture off | Must | Collector integration and privacy test | R2/R3 | Unimplemented |
| OBS-002 | Estimated and authoritative token usage are distinguishable | Must | Provider fixture tests | R3 | Unimplemented |
| OPS-001 | Core proxy requires no database, broker, or hosted control plane | Must | Clean-environment acceptance test | R3 | Proposed |
| OPS-002 | Minimal local adoption journey completes in <=10 minutes median | Must | Five-person usability study | R3/R4 | Unvalidated |
| PER-001 | Equivalent-control added latency meets approved benchmark budget | Must | Reproducible benchmark | R3 | Unvalidated |
| PER-002 | Idle and per-stream memory meet approved benchmark budget | Must | Reproducible profile | R3 | Unvalidated |
| SUP-001 | Released artifacts carry SBOM, provenance, checksums, and signatures | Must for public production | Release verification | R6 | Unimplemented |
| GOV-001 | Protected `development` -> `staging` -> `production` promotion and reviewed topic branches are enforced | Must | GitHub rules audit | Foundation | Enforced and verified on all three promotion branches |

## Priority language

- **Must:** required to pass the named gate.
- **Should:** expected unless evidence justifies removal.
- **Could:** optional and must not jeopardize a Must requirement.
- **Will not now:** explicitly outside the current milestone.

Every implementation issue must reference at least one requirement ID or state that it is a research spike.
