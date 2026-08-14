# Branching and Release Workflow

**Status:** Accepted under ADR-0004

## Branch roles

| Branch | Purpose | Allowed inbound changes |
| --- | --- | --- |
| `development` | Default integration branch | Reviewed topic-branch pull requests |
| `staging` | Release-candidate source | Promotion pull requests from `development` |
| `production` | Released source and tag target | Promotion pull requests from `staging` |
| `main` | Historical bootstrap retained for compatibility | No routine changes |

Normal work uses short-lived branches such as `feat/*`, `fix/*`, `docs/*`, `research/*`,
`security/*`, or `agent/*`. Contributors must never implement directly on `staging` or
`production`.

## Promotion rules

1. Topic branch -> `development` after review and CI.
2. `development` -> `staging` when a bounded release candidate is approved.
3. Release defects return through a topic branch into `development`; do not patch `staging`
   directly.
4. `staging` -> `production` only after release checks pass.
5. Annotated SemVer tags point to the released `production` commit.

Branches represent source state, not proof that software is deployed. When artifacts exist, build
once from reviewed source and promote a verified digest instead of rebuilding different code for
each environment.

## Required protections

For `development`, `staging`, and `production`:

- pull requests and the `Rust quality gates` check are required;
- branches must be current with their promotion source;
- conversations must be resolved;
- force pushes and deletion are disabled;
- linear/squash history is used consistently;
- founder self-review is documented while there is one maintainer;
- independent approval becomes required after a second maintainer joins;
- security-critical and release changes use two-person review when the maintainer group permits.

See `docs/governance/RELEASES.md` for version and tag rules.
