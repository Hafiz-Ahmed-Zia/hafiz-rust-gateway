# ADR-0003: Establish a Network-Neutral Domain Core

**Status:** Accepted
**Date:** 2026-08-14
**Requirements:** STR-001, STR-004, REL-001

## Context

ADR-0002 requires equivalent `hyper`/Tokio and Pingora spikes before selecting a network stack.
Commitment, terminal-state, and retry-budget rules must not drift between those implementations.

## Decision

Create three initial crates only:

- `hafiz-gateway-core` for stable failure classes, the streaming state machine, and recovery policy;
- `hafiz-gateway-testkit` for deterministic bounded upstream scripts;
- `hafiz-gateway` for the process/CLI boundary and contract demonstrations.

The domain core has no asynchronous runtime, HTTP framework, serialization format, telemetry client,
or provider dependency. Network spikes must adapt their observed events into this shared contract.

## Consequences

- Both network candidates can be compared against identical failure invariants.
- Pure unit tests run quickly and deterministically.
- This slice does not implement an HTTP listener, real cancellation propagation, backpressure, TLS,
  provider translation, configuration, or production telemetry.
- New crates require evidence that a boundary cannot remain clear inside the existing three.
