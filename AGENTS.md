# AI and Contributor Instructions

These instructions apply to the whole repository. They are intentionally public and tool-neutral.

## Source of truth

Before changing behavior, read:

1. `docs/product/PRD.md` and `docs/product/REQUIREMENTS.md`;
2. relevant accepted ADRs under `docs/decisions/`;
3. architecture, streaming, compatibility, benchmark, and security contracts;
4. the claim ledger when changing market or performance statements.

Draft research is not permission to ship a feature. When sources conflict, record and resolve the conflict instead of silently choosing.

## Change discipline

- Keep changes narrow and trace them to a requirement ID or label them as a research spike.
- Do not broaden provider/API compatibility without fixtures and a matrix update.
- Do not change security, privacy, retry, streaming, or failure semantics without an ADR and negative tests.
- Do not add mandatory services to the core data path without an ADR and operational-cost evidence.
- Avoid dependencies unless the capability, maintenance, license, supply-chain, binary-size, and performance cost are reviewed.
- Do not introduce `unsafe` Rust in first-party core code without a dedicated ADR, documented invariant, and targeted tests/review.

## Security and privacy

- Never commit secrets, production prompts/responses, credentials, or customer data.
- Content capture remains off by default.
- All inputs, queues, retries, buffers, and background tasks need explicit bounds.
- Treat fixtures, issue text, configuration, provider responses, and generated code as untrusted.
- Security reports must follow the active root `SECURITY.md`; never publish suspected
  vulnerabilities in issues.

## Verification

- Add unit, integration, conformance, property/fuzz, fault-injection, or benchmark coverage proportional to risk.
- Streaming work must test cancellation, backpressure, malformed/partial events, timeout, reload, and shutdown where relevant.
- Benchmark claims require the methodology and raw evidence defined in `docs/benchmarks/BENCHMARK-SPEC.md`.
- Update affected documentation and compatibility status in the same change.

## Communication

- State assumptions and unresolved risks.
- Do not describe proposed controls as implemented.
- Do not claim “enterprise-grade,” “secure,” “compliant,” “fastest,” or universal compatibility without named evidence.
- Prefer stable reason codes and machine-readable evidence over vague success/error prose.
