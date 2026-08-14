# Branching and Release Workflow

**Status:** Accepted foundation decision

## Decision

Use GitHub Flow:

- one long-lived protected branch: `main`;
- short-lived branches such as `feat/*`, `fix/*`, `docs/*`, `research/*`, `security/*`, or the automation-specific `agent/*` convention;
- pull requests for review and CI;
- merge, then delete topic branches;
- immutable version tags for releases.

Do not create long-lived `development`, `staging`, or `production` branches.

## Why

Environment branches drift, accumulate merge-only work, and do not prove what artifact is deployed. Hafiz Rust Gateway is an open-source binary/container project, not three separate codebases.

When deployment exists:

- build once from a reviewed commit/tag;
- promote the same artifact digest through GitHub Environments named `development`, `staging`, and `production`;
- attach approvals and deployment evidence to environments/releases;
- never rebuild different source for each environment.

## Target protections for `main`

- pull request required;
- required checks and conversation resolution;
- independent approval once a second maintainer exists; until then, passing CI plus documented
  founder self-review;
- two approvals for release/security-critical changes when the maintainer group permits;
- no force pushes or branch deletion;
- linear/squash history policy selected consistently;
- signed commits/tags where feasible;
- release workflow pinned and least-privilege.

## Release channels

- `v0.x.y` tags during unstable development;
- release candidates such as `v0.1.0-rc.1`;
- stable semantic versions only after compatibility/support policy exists;
- environment promotion uses artifact digest, not branch name.
