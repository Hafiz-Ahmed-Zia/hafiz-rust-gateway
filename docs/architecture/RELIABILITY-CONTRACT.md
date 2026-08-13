# Reliability and Failure Contract

**Status:** Proposed

## Failure taxonomy

| Class | Example | Retry? | Health impact? | Client behavior |
| --- | --- | --- | --- | --- |
| Client | invalid schema, unauthorized model | No | No | stable 4xx-style gateway error |
| Policy | denied route, quota exhausted | No | No | policy-specific stable error |
| Overload | gateway queue/concurrency full | Usually no | Gateway saturation metric | fast rejection, optional `Retry-After` |
| Network transient | connect reset before commitment | Bounded | Yes | retry/fallback if budget permits |
| Provider throttle | 429 with usable hints | Bounded/alternate | Capacity signal, not hard failure | honor hints or route alternate |
| Provider server | eligible 5xx before commitment | Bounded | Yes | retry/fallback if safe |
| Provider client | provider rejects transformed request | No by default | Adapter/conformance signal | normalized compatible error |
| Stream partial | upstream dies after visible output | No silent replay | Yes | terminate partial stream honestly |
| Telemetry | exporter unavailable | No request retry | No | traffic continues; drops/backlog observable |
| Configuration | new snapshot invalid | N/A | Config activation failure | retain last-known-good snapshot |

## Retry budget

Retries are bounded by all of:

- maximum attempt count;
- maximum retry time;
- remaining end-to-end deadline;
- per-route retry concurrency/token budget;
- provider-specific `Retry-After` and idempotency/replay rules;
- commitment point.

Backoff includes jitter. Retry storms are tested under correlated provider failure.

## Circuit breaker

- Separate connect/transport failure, overload/throttle, provider 5xx, and caller error signals.
- Use minimum sample thresholds and bounded windows.
- Half-open probes are limited.
- Operator override is audited and expires.
- Circuit state is local unless a distributed design is explicitly selected.

## Load shedding

The priority order is:

1. protect process health;
2. reject new low-priority work;
3. preserve admitted streams within budgets;
4. shed optional telemetry/export work;
5. terminate admitted work only through explicit deadlines or emergency policy.

## Configuration availability

- Startup requires one fully valid configuration snapshot.
- Remote configuration loss does not invalidate the active snapshot.
- Expiry/freshness policy is explicit; fail-closed expiry is available for sensitive deployments.
- Rollback is a first-class activation of a known snapshot, not an ad hoc file replacement.

## Dependency failure

Each optional dependency declares:

- timeout;
- fail-open or fail-closed behavior;
- cache/freshness behavior;
- fallback;
- observability;
- maximum resource use during outage.

Security-critical identity/policy checks default to fail closed. Optional telemetry defaults to fail operationally open while making loss visible.

## Chaos scenarios required before enterprise pilot

- provider 429/5xx storm;
- slow DNS and failed DNS;
- TLS handshake stall and invalid certificate;
- slowloris client;
- slow downstream and fast upstream;
- OTel/SIEM outage;
- remote config outage and malformed update;
- process termination during active streams;
- node clock skew within documented assumptions;
- rate-limit backend partition;
- credential rotation during traffic;
- memory pressure and file-descriptor exhaustion.
