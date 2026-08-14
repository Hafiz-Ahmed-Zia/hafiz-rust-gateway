# Proposed Repository Security Policy

**Status:** Superseded as policy by the root `SECURITY.md` on 2026-08-14; retained as a future-scope
design note. The root file is authoritative.

## System and Scope

Hafiz Rust Gateway is intended to be a self-hosted AI/LLM traffic data plane. The future security policy should cover runtime gateway code, provider adapters, authentication and policy modules, configuration/reload, telemetry/audit, build/release automation, official containers/binaries, and first-party deployment assets.

The project has a network-neutral domain skeleton but no network runtime or supported release.
Current documents describe intended properties and must not be treated as evidence that controls
exist beyond their named executable tests.

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

The active root policy defines current reportability and exclusions. This design note cannot expand
or narrow that policy.

The following are not automatically product vulnerabilities unless gateway behavior materially causes or worsens them:

- provider model hallucination or training-data poisoning;
- application misuse of otherwise authorized model output;
- unsupported community forks or modified artifacts;
- attacks requiring a deliberately disabled security control, when the override is explicit, development-only, and documented.

These statements are design guidance only unless incorporated into the root policy.

## Known Limitations and Compensating Controls

- No implementation exists, so proposed controls are unverified.
- Compliance and availability depend on deployment/organizational controls.
- OpenTelemetry GenAI conventions and provider APIs change; compatibility is release-scoped.
- Content guardrails cannot guarantee semantic safety and are outside the initial core.

## Vulnerability Reporting

Before public exposure, enable GitHub private vulnerability reporting. Before a supported release,
establish a monitored response role, supported-version policy, coordinated-disclosure process, and
advisory/CVE workflow. Public issues must never be the preferred channel for undisclosed
vulnerabilities.

## Remaining follow-up checklist

- Confirm internet-facing and internal-only deployment surfaces.
- Confirm attacker-controlled versus operator-controlled upstream configuration.
- Confirm tenant and data-sensitivity assumptions.
- Confirm severity examples and any exclusions.
- Establish the private security contact.
- Verify GitHub private reporting and the resolved policy chain after remote creation.
