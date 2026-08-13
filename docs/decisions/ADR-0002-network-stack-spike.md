# ADR-0002: Compare hyper/Tokio and Pingora Before Selecting the Network Stack

**Status:** Proposed  
**Date:** 2026-08-13

## Context

The gateway needs high-performance HTTP proxying and unusually precise streaming translation, cancellation, backpressure, retry commitment, reload, and shutdown behavior. Pingora offers production proxy primitives; `hyper`/Tokio offers lower-level protocol control.

## Proposed decision

Build minimal equivalent spikes and select using correctness, security, performance, resource use, dependency/build risk, implementation complexity, and operability. No stack wins by reputation or a synthetic pass-through benchmark.

## Required spike

- identical TLS/auth/model policy;
- one non-streaming and one typed SSE path;
- client cancellation and slow downstream;
- upstream error before and after commitment;
- config reload and graceful shutdown;
- bounded metadata telemetry;
- raw benchmark/profile evidence.

## Acceptance

Promote this ADR only after the evidence is attached and reviewed.
