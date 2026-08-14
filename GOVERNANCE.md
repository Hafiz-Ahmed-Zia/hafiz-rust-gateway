# Governance

**Status:** Initial project governance; revise before broad contributor onboarding

## Principles

- Product scope follows evidence and explicit gates.
- Security, privacy, protocol honesty, and reproducibility cannot be traded for marketing speed.
- Decisions and rationale remain public unless disclosure would create a security or privacy risk.
- Core correctness and secure defaults remain available in the open-source project.

## Roles

- **Founder/owner:** final decision authority during foundation, including brand, license, roadmap, release, and commercial boundary.
- **Maintainer:** reviews/merges changes, owns components, triages issues, and participates in releases.
- **Contributor:** proposes issues, research, docs, tests, code, or reviews.
- **Security responder:** privately triages vulnerabilities and coordinates advisories/releases; this role must be assigned before public production release.

## Decision types

- Requirements: PRD revision with evidence and owner approval.
- Architecture/security/protocol: ADR plus relevant contract/test updates.
- Compatibility: matrix evidence and maintainer review.
- Release: checklist, signed artifacts, SBOM/provenance, and required approvals.
- Governance/license: public proposal and owner approval; legal review when appropriate.

## Merge policy

The target remote policy is protected `main`, pull-request changes, required CI, no force push, and
no branch deletion. Required independent approval activates when a second maintainer exists. Until
then, the founder may merge a passing pull request after a documented self-review. Security-critical
and release changes should require two-person review when the maintainer group permits it.

## Project continuity

To reduce single-maintainer abandonment risk:

- keep build/release/conformance infrastructure in the public repository;
- avoid hidden dependencies for core operation;
- document maintainer and release transfer;
- publish signed source-derived releases and reproducible instructions;
- establish multiple maintainers before enterprise production claims;
- document archival or wind-down behavior rather than silently abandoning users.
