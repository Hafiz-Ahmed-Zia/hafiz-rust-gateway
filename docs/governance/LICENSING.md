# Licensing Decision

**Status:** Proposed; owner/legal approval required before public release

## Recommendation

Use **Apache License 2.0** for the initial repository.

Reasons:

- permissive commercial and internal adoption;
- explicit patent grant and termination terms valued by enterprise adopters;
- common in infrastructure and gateway projects;
- allows forks and continuity if original maintenance stops.

## Alternative

Dual MIT/Apache-2.0 is common in the Rust ecosystem and can simplify reuse in some Rust projects, but adds licensing presentation/maintenance and offers little product differentiation.

## Decision gate

Before adding the root license and publishing:

1. owner approves Apache-2.0 or selects the alternative;
2. contributor copyright/provenance approach is documented;
3. dependency licenses are checked;
4. trademarks and branding are clearly separate from source-code license;
5. commercial features do not withhold core security/correctness deceptively.

Until a root license exists, the repository must not be advertised as ready for third-party open-source reuse.
