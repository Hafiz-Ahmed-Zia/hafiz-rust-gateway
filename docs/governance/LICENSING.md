# Licensing Decision

**Status:** Accepted by owner on 2026-08-14; legal review remains advisable before commercial use

## Decision

Use **Apache License 2.0** for the initial repository. The root `LICENSE` file is authoritative.

Reasons:

- permissive commercial and internal adoption;
- explicit patent grant and termination terms valued by enterprise adopters;
- common in infrastructure and gateway projects;
- allows forks and continuity if original maintenance stops.

## Alternative

Dual MIT/Apache-2.0 is common in the Rust ecosystem and can simplify reuse in some Rust projects, but adds licensing presentation/maintenance and offers little product differentiation.

## Follow-up controls

1. Contributions use the DCO process documented in `CONTRIBUTING.md`.
2. Dependency licenses are checked before release and when dependencies change.
3. Trademarks and branding remain separate from the source-code license.
4. Commercial features must not deceptively withhold core security or correctness.
