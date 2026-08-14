# Conformance Fixture Schema v1

**Status:** Implemented foundation

**Requirements:** API-001, API-002, STR-001, STR-004, SEC-005, REL-001

## Purpose

Versioned fixtures describe a synthetic HTTP request, deterministic upstream wire behavior, and the
expected network-neutral stream result. They let future Hyper/Tokio and Pingora spikes face the same
fragmentation and failure sequence without credentials or external services.

The first fixture is
[`fixtures/conformance/v1/fragmented-completion.json`](../../fixtures/conformance/v1/fragmented-completion.json).
It splits one sanitized SSE event across three TCP writes and validates the result against the shared
stream state machine. The testkit also replays it through a real loopback TCP connection.

## Reader rules

- `schema_version` is required and must equal `1`.
- Unknown fields are rejected in v1 so misspelled security- or failure-relevant fields cannot be
  ignored.
- Readers reject unsupported versions; they never guess or silently downgrade.
- Compatible optional additions require a new documented reader release. Breaking meaning or field
  changes require a new integer schema version and a versioned fixture directory.
- Fixture content must be synthetic and sanitized. Production prompts, responses, credentials, and
  customer data are prohibited.

## Hard bounds

| Input | v1 bound |
| --- | ---: |
| Fixture before JSON parsing | 256 KiB |
| Request body | 64 KiB |
| Request or response headers | 64 |
| Header name | 128 bytes |
| Header value | 8 KiB |
| Provider steps | 256 |
| One wire fragment | 64 KiB |
| One accumulated protocol event | 64 KiB |
| One scripted delay | 30 seconds |

The fake provider separately caps the received HTTP request at 128 KiB, its header block at 32 KiB,
accept at three seconds, and socket reads/writes at two seconds. It binds only to IPv4 loopback and
accepts one request before stopping.

## Provider steps

- `response_headers` commits the configured status and sanitized response headers.
- `fragment` writes bounded UTF-8 wire bytes. `event_end: true` marks one complete protocol event.
- `delay` pauses for a bounded number of milliseconds.
- `disconnect` closes without clean completion and becomes either a pre-commit transient failure or
  a post-commit partial-stream failure.
- `complete` cleanly ends a committed response and is rejected when an event remains incomplete.

Every script must end exactly once. Steps after `disconnect` or `complete`, duplicate commitment,
fragments before commitment, completion before commitment, and declared-outcome mismatches fail
deterministically.

## Validate locally

```bash
cargo run -p hafiz-gateway -- validate-fixture \
  fixtures/conformance/v1/fragmented-completion.json
cargo test -p hafiz-gateway-testkit
```

## Current boundary

This is test infrastructure, not provider compatibility evidence. It does not establish an
OpenAI-compatible endpoint, production proxy, TLS, cancellation propagation, slow-consumer
backpressure, or network-stack selection. Those remain separate Stage 1 proofs.
