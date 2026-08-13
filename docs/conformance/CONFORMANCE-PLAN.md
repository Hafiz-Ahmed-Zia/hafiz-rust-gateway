# Conformance Plan

**Status:** Proposed

## Objectives

- Define exactly what “compatible” means.
- Catch provider drift before it reaches users.
- Verify streaming and error semantics, not only happy-path JSON shapes.
- Let community adapters prove a compatibility tier without relying on reputation.

## Test layers

### Offline golden fixtures

Versioned sanitized requests, responses, SSE event sequences, errors, headers, and usage objects. Fixtures must contain no production customer data.

### Differential fake-provider tests

A deterministic provider server emits fragmentation, delays, partial events, malformed data, rate limits, and disconnects. Gateway output is checked against the endpoint contract.

### SDK compatibility tests

Pinned official or widely used clients execute the advertised endpoint subset against the gateway.

### Live provider canaries

Small, budgeted, secret-protected tests run on schedule and before release for native adapters. They avoid unstable content assertions and validate protocol shape, streaming sequence, error/auth behavior where safely possible, and usage metadata.

### Property and fuzz tests

- arbitrary fragmentation/coalescing of byte chunks;
- bounded JSON and event parsing;
- header normalization and hop-by-hop removal;
- state-machine transitions;
- cancellation at every transition;
- redaction invariants.

## Adapter acceptance

An adapter cannot be Native without:

- named maintainer/owner;
- endpoint and model/API version scope;
- offline fixtures;
- non-streaming and streaming tests;
- error/rate-limit classification tests;
- cancellation behavior;
- usage-accounting source;
- live canary or documented reason it cannot run;
- published deviations and last-verified date.

## Drift handling

- Canary failure marks the relevant matrix entry degraded; it does not silently rewrite behavior.
- Breaking provider drift triggers an issue and release note.
- A compatibility tier can be downgraded independently of a gateway release.
- Historical results remain available so operators can see when behavior changed.

## Privacy and cost

Live tests use dedicated low-privilege credentials, non-sensitive prompts, strict spend limits, and no production data. Test logs follow the same no-content-by-default rule unless fixture content is explicitly synthetic and labeled.
