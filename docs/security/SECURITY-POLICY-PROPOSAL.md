# Proposed Repository Security Policy

**Status:** Proposal only. This file is not the active root `SECURITY.md`. Owner approval is required before activation.

## System and Scope

Hafiz Rust Gateway is intended to be a self-hosted AI/LLM traffic data plane. The future security policy should cover runtime gateway code, provider adapters, authentication and policy modules, configuration/reload, telemetry/audit, build/release automation, official containers/binaries, and first-party deployment assets.

The project is pre-implementation. Current documents describe intended properties and must not be treated as evidence that controls exist.

## Threat Model and Trust Boundaries

The authoritative working model is `docs/security/THREAT-MODEL.md`. Important boundaries are client-to-gateway, tenant-to-tenant, gateway-to-provider, data-plane-to-optional-services, configuration/control-plane-to-data-plane, telemetry export, and source/CI-to-release.

## Security Invariants

- tenant identities, content, credentials, policy, quotas, and telemetry remain isolated;
- inbound credentials never become upstream provider credentials;
- secrets/content are absent from default telemetry and errors;
- invalid or unverifiable security policy fails closed;
- parsers, buffers, queues, retries, and telemetry are bounded;
- untrusted routing cannot reach forbidden internal/metadata destinations;
- configuration activation is complete, validated, atomic, and versioned;
- no silent replay/fallback after downstream-visible output;
- release artifacts are verifiable.

## Reportable Findings and Severity Context

A finding is reportable when a realistic actor can violate an invariant in an official runtime, released artifact, build/release path, or first-party deployment configuration. Severity follows `docs/security/THREAT-MODEL.md` and depends on reachability, tenant/data exposure, credentials, cost, and availability impact.

Documentation gaps are security findings when they create an unsafe default or credible operator footgun, not merely because prose can be improved.

## Out of Scope, Exclusions, and Accepted Risk

No owner-approved exclusions or accepted security risks exist yet.

The following are not automatically product vulnerabilities unless gateway behavior materially causes or worsens them:

- provider model hallucination or training-data poisoning;
- application misuse of otherwise authorized model output;
- unsupported community forks or modified artifacts;
- attacks requiring a deliberately disabled security control, when the override is explicit, development-only, and documented.

These statements require owner confirmation before they become active scan/reporting policy.

## Known Limitations and Compensating Controls

- No implementation exists, so proposed controls are unverified.
- Compliance and availability depend on deployment/organizational controls.
- OpenTelemetry GenAI conventions and provider APIs change; compatibility is release-scoped.
- Content guardrails cannot guarantee semantic safety and are outside the initial core.

## Vulnerability Reporting

Before public release, the owner must establish a monitored private reporting channel, response targets, supported-version policy, coordinated-disclosure process, and advisory/CVE workflow. Public issues must not be the preferred channel for undisclosed vulnerabilities.

## Approval checklist

- Confirm internet-facing and internal-only deployment surfaces.
- Confirm attacker-controlled versus operator-controlled upstream configuration.
- Confirm tenant and data-sensitivity assumptions.
- Confirm severity examples and any exclusions.
- Establish the private security contact.
- Move the approved text to root `SECURITY.md` and verify the resolved policy chain.
