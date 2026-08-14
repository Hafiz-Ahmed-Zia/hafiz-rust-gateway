# ADR-0005: Use Strict Versioned JSON for Conformance Fixtures

**Status:** Accepted

**Date:** 2026-08-14

**Requirements:** API-001, API-002, SEC-005

## Context

Equivalent network-stack experiments need deterministic, reviewable, machine-validated provider
behavior. Fixtures are untrusted inputs and must not create unbounded allocations, silently ignore
typos, contain production data, or couple the domain core to an HTTP runtime.

## Decision

- Store sanitized fixtures as versioned JSON under `fixtures/conformance/vN/`.
- Enforce a byte limit before parsing and explicit limits on every variable-length collection,
  string, fragment, event, request, and delay.
- Reject unknown fields and unsupported versions.
- Check each declared outcome against the existing network-neutral stream contract.
- Keep parsing and fake-provider dependencies in `hafiz-gateway-testkit`; the hot-path
  `hafiz-gateway-core` remains dependency-free.

The testkit uses `serde` and `serde_json`. Both are broadly maintained Rust ecosystem libraries with
permissive dual MIT/Apache-2.0 licensing. Their resolved versions remain pinned by `Cargo.lock` and
covered by the repository dependency audit. The capability is isolated from the future production
data plane, so parsing convenience does not select the gateway network stack or add hot-path cost.

## Consequences

- Fixtures are easy for contributors and CI to inspect and validate offline.
- Strict v1 readers require deliberate schema evolution instead of permissive forward guessing.
- JSON strings represent sanitized UTF-8 test bytes; arbitrary binary protocol fixtures require a
  separately reviewed encoding rule.
- The loopback fake provider is evidence for test-harness behavior only, not production compatibility
  or performance.
