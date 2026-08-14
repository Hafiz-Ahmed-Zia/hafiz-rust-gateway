# Benchmark Specification

**Status:** Proposed
**Goal:** Measure the cost of gateway guarantees, not produce a flattering requests-per-second number.

## 1. Reporting rules

Every published result includes:

- exact commit, build profile, Rust/toolchain, feature flags, allocator, and dependencies;
- operating system, kernel, CPU model/count/governor, memory, NIC, container/runtime limits;
- gateway configuration and enabled controls;
- load-generator and fake-provider versions/configuration;
- connection reuse, HTTP version, TLS mode, payload/event sizes, concurrency, duration, and warmup;
- raw machine-readable results plus processing script;
- error, timeout, cancellation, and dropped-telemetry counts;
- at least five independent runs with median and dispersion;
- idle baseline and direct-to-upstream baseline;
- flamegraph/profile for regressions above the threshold.

No comparison is valid when competitors run without equivalent TLS, authentication, policy, body parsing, streaming, or telemetry.

## 2. Primary metrics

- added latency distribution: p50, p90, p95, p99, p99.9;
- upstream-first-event to downstream-first-event delay;
- throughput at fixed latency/error SLO;
- CPU time per completed request and per streamed event;
- RSS idle, under load, peak, and post-load;
- incremental memory per active stream;
- allocations/bytes per request where measurable;
- startup to readiness;
- executable and container size;
- connection-pool reuse and new-connection rate;
- failure/cancellation cleanup time;
- buffer high-water marks and backpressure duration.

## 3. Reference workloads

### W1 — Metadata-only non-streaming pass-through

- 2 KiB request, 4 KiB response
- auth, model allow policy, route, TLS, metadata telemetry enabled
- 1, 16, 64, 256, and saturation concurrency

### W2 — Streaming text

- 2 KiB request
- 200 SSE events, 64 bytes median payload, controlled cadence
- fast and slow downstream variants
- cancellation at 10%, 50%, and 90% of stream

### W3 — Large valid request

- body sizes: 64 KiB, 1 MiB, configured maximum
- pass-through and normalized parsing variants

### W4 — Provider failure

- connect error, 429, eligible 5xx, header timeout
- retry succeeds, fallback succeeds, circuit opens
- measure amplification and tail latency

### W5 — Adversarial boundaries

- fragmented SSE events;
- maximum-size event;
- malformed JSON/event;
- slowloris headers/body;
- decompression ratio limit;
- telemetry sink blocked;
- configuration reload under load.

### W6 — Long-lived fleet

- thousands of concurrent low-rate streams where hardware allows;
- 30-60 minute soak;
- memory growth, task/FD leaks, connection churn, and cancellation cleanup.

## 4. Stage 1 engineering targets

Targets are provisional and pass only with required controls enabled:

| Metric | Target |
| --- | --- |
| W1 added latency | p50 <= 250 µs, p99 <= 1 ms at approved reference load |
| W2 first-event forwarding overhead | p99 <= 1 ms after complete upstream event availability |
| Idle RSS | <= 50 MiB minimal build/config |
| Incremental steady stream memory | <= 128 KiB at reference event/backpressure profile |
| Startup to local readiness | <= 100 ms excluding external dependency checks |
| Gateway-caused errors | 0 in non-saturation steady-state run |
| Unbounded growth | none in soak/backpressure/telemetry-outage tests |

If the target is missed, publish the result and decide whether to optimize, re-scope, or revise the target. Do not hide required-control cost.

## 5. Competitor comparisons

Competitor results are optional until the harness is stable. When performed:

- use released versions and document license/edition;
- allow reasonable tuning and publish it;
- compare the same endpoint semantics and control set;
- do not compare one tool's pass-through path with another's normalized/guardrailed path;
- separate gateway overhead from provider/network latency;
- invite maintainers to correct configuration or methodology.

## 6. Regression policy

A change blocks merge when it causes, relative to the current approved baseline:

- >5% p50 or >10% p99 added-latency regression;
- >5% throughput loss at the fixed SLO;
- >10% idle or per-stream memory regression;
- new unbounded allocation/queue behavior;
- any new crash, data leak, invalid stream transition, or post-commit replay.

Thresholds can change only through an ADR with raw evidence.
