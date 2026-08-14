# Project Fact Sheet

| Field | Value |
| --- | --- |
| Project | Hafiz Rust Gateway |
| License | Apache-2.0 |
| Repository | <https://github.com/Hafiz-Ahmed-Zia/hafiz-rust-gateway> |
| Maintainer | Ahmed Zia (founder and primary maintainer) |
| Status | Pre-alpha, source-only foundation |

## Problem and approach

Hafiz Rust Gateway is exploring a self-hosted AI traffic boundary for teams that need predictable
streaming/failure semantics, content-private telemetry, and a small data plane without a mandatory
database or hosted control plane. Compatibility and performance are intended to be proven through
public fixtures and reproducible evidence rather than broad claims.

## What exists today

- a compiling three-crate Rust workspace;
- executable streaming-commitment, failure-classification, and recovery-budget contracts;
- deterministic clean, pre-commit-disconnect, and partial-stream scenarios;
- CI formatting, strict Clippy, tests, RustSec audit, Dependabot, and secret scanning;
- public license, governance, contribution, support, security, threat-model, roadmap, and research
  records;
- private GitHub vulnerability reporting and protected promotion branches.

There is no HTTP listener, TLS, provider adapter, supported production release, benchmark result,
or verified user adoption yet.

## Why the work may matter

AI gateway features are widely available, but protocol fidelity, streaming failure behavior,
privacy defaults, and comparable resource evidence are often difficult to independently verify.
This project tests whether open conformance and fault evidence can become a useful shared asset for
gateway users and upstream projects. That ecosystem-importance hypothesis remains unvalidated.

## Maintainer work that support would accelerate

- pull-request review and issue triage;
- primary-source research and claim verification;
- conformance fixtures and fake-provider fault scenarios;
- security review, fuzzing, and regression-test reduction;
- release notes, compatibility records, and contributor documentation;
- bounded automation for this authorized repository using synthetic data only.

## Public evidence policy

Stars, downloads, users, design partners, latency, memory, compatibility, and security outcomes are
reported only when the repository links to the underlying evidence. As of 2026-08-14, the project
is new and has no adoption metrics; applications and project materials must say so directly.
