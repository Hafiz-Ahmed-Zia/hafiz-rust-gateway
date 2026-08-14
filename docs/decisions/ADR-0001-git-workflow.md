# ADR-0001: Use GitHub Flow

- **Status:** Superseded by ADR-0004
- **Date:** 2026-08-13

## Context

The project considered long-lived development, staging, and production branches. Those branches would represent deployment environments rather than independent source lines and would create drift/merge overhead before any deployable artifact exists.

## Decision

Use one protected `main` branch, short-lived topic branches, pull requests, and immutable release tags. Future environments promote the same artifact digest rather than different branch builds.

## Consequences

- simpler contributor workflow;
- `main` should remain releasable after CI exists;
- unfinished work stays in topic branches or behind unexposed experimental code;
- deployments require environment/release metadata rather than branch naming;
- GitHub branch protections and environments must be configured after remote creation.

## Supersession

On 2026-08-14 the founder selected controlled `development` -> `staging` -> `production` source
promotion to match the established workflow used by the related Sasta Inverter project. ADR-0004
records the replacement decision and its added drift controls. This file remains as decision
history.

## Evidence

GitHub documents GitHub Flow as a lightweight branch-based workflow and recommends separate short-lived branches per change, pull-request review, merge, and branch deletion.
