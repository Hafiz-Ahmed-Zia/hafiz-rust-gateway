# Streaming Contract

**Status:** Proposed  
**Applies to:** All advertised streaming endpoints and adapters

## 1. Promise

Hafiz Rust Gateway treats streaming as a stateful protocol contract, not ordinary response forwarding. It preserves required ordering and termination behavior, propagates cancellation and backpressure, and reports partial failure honestly.

## 2. Request state machine

```text
ACCEPTED
  -> AUTHENTICATED
  -> POLICY_ALLOWED
  -> TARGET_SELECTED
  -> UPSTREAM_CONNECTING
  -> HEADERS_PENDING
  -> STREAMING
  -> COMPLETED | CLIENT_CANCELLED | UPSTREAM_FAILED | POLICY_TERMINATED
               | DEADLINE_EXCEEDED | OVERLOAD_TERMINATED | SHUTDOWN_TERMINATED
```

Invalid transitions are bugs and must be covered by property/state-machine tests.

## 3. Commitment point

The **downstream commitment point** occurs when response headers or body bytes make a retry/fallback externally observable. The implementation may use a stricter provider-specific point.

Before commitment, a retry or fallback is allowed only when:

- the error class is retryable;
- the attempt and global budgets allow it;
- request semantics allow replay;
- the client deadline can still be respected.

After commitment, v1 never silently retries or switches provider. It terminates the stream using the closest valid protocol behavior and records a stable terminal reason.

## 4. Event handling

- Parse incrementally; never require buffering the full provider response.
- Enforce maximum raw event size and maximum decoded structure depth.
- Preserve ordering.
- Do not merge or split events when the compatibility contract requires exact boundaries.
- If translation is required, each output event must be traceable to input event type and sequence.
- Treat malformed UTF-8, invalid JSON, unknown required event types, and truncated frames according to the provider compatibility matrix.
- Do not expose upstream secrets or raw internal errors in terminal events.

## 5. Backpressure and memory

- Downstream slowness must slow or pause upstream reads where the transport permits.
- Every per-stream channel has a documented finite capacity.
- Queue saturation has a defined timeout and terminal reason.
- Memory tests cover a slow client, fast upstream, fragmented events, maximum-sized valid events, and cancellation while buffers are full.
- Compression and decompression have ratio and output-size bounds.

## 6. Cancellation

- Detect downstream disconnect/cancellation promptly.
- Cancel the upstream request and stop policy/telemetry work that is no longer needed.
- Record `client_cancelled` separately from gateway/provider failure.
- Do not count a client cancellation against provider health unless the provider independently failed.
- Token/cost metadata after cancellation is recorded only if authoritative and available; never fabricate completion usage.

## 7. Timeouts

Separate:

- queue wait timeout;
- connect timeout;
- TLS handshake timeout;
- response-header timeout;
- idle-between-events timeout;
- maximum stream duration;
- end-to-end request deadline.

One generic timeout is insufficient. The terminal reason identifies the phase without revealing sensitive upstream details.

## 8. Shutdown and reload

- Configuration reload never mutates an in-flight request snapshot.
- Graceful shutdown stops accepting new work, signals drain, and lets active streams continue up to a configured drain deadline.
- Forced termination after the deadline is observable with `shutdown_terminated`.
- Restart/upgrade claims require process-level tests with active streams.

## 9. Required tests

1. normal completion;
2. client disconnect before headers;
3. client disconnect mid-event and between events;
4. upstream disconnect before commitment;
5. upstream disconnect after commitment;
6. malformed/truncated event;
7. slow downstream with fast upstream;
8. stalled upstream idle timeout;
9. end-to-end deadline;
10. retry succeeds before commitment;
11. fallback succeeds before commitment;
12. retry/fallback is refused after commitment;
13. config reload during stream;
14. graceful and forced shutdown;
15. telemetry sink failure during stream;
16. circuit opening while streams are active;
17. provider usage event arriving after content events;
18. tool-call argument fragments and structured-output deltas.

## 10. Metrics

- active streams;
- accepted/completed/cancelled/failed streams by terminal reason;
- first-upstream-event and first-downstream-event latency;
- forwarded bytes/events;
- per-stream high-water buffer usage;
- backpressure duration;
- malformed event count;
- forced shutdown termination count;
- retry/fallback attempts before commitment;
- prohibited post-commit retry attempts, which must remain zero.
