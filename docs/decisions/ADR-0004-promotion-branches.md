# ADR-0004: Use Controlled Promotion Branches

- **Status:** Accepted
- **Date:** 2026-08-14
- **Supersedes:** ADR-0001

## Context

The founder requested the same visible source-promotion model used by the related Sasta Inverter
project: normal integration on `development`, a review boundary on `staging`, and released source
on `production`. Hafiz Rust Gateway has no live environments yet, so branch names must not be
treated as deployment proof.

## Decision

Use three protected long-lived promotion branches:

- `development`: default branch and target for normal topic-branch pull requests;
- `staging`: release-candidate source promoted only from `development`;
- `production`: released source promoted only from `staging` and used for version tags.

Short-lived `feat/*`, `fix/*`, `docs/*`, `research/*`, `security/*`, or `agent/*` branches remain the
unit of implementation. The historical `main` branch is retained for compatibility but is not a
normal integration or release target.

## Controls

- all three promotion branches require pull requests, passing CI, conversation resolution, and no
  force-push or deletion;
- direct implementation occurs only on topic branches and merges into `development`;
- promotion must preserve reviewed source; a branch name does not prove deployment;
- tags use SemVer and point to the released commit on `production`;
- when deployable artifacts exist, promotion records the immutable artifact digest separately.

## Consequences

- the workflow matches the founder's established release mental model;
- reviewers can distinguish integration, candidate, and released source;
- three long-lived branches increase drift and merge risk, so promotions must remain one-way and
  release fixes must return through `development`;
- GitHub's default branch becomes `development`, which is less conventional for open-source
  projects and must be documented prominently.
