# Security Policy

Hafiz Rust Gateway is pre-alpha research software. It must not be treated as a production
security boundary yet.

## Supported versions

There are no supported production releases. Security fixes currently target the `main` branch.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability. Use GitHub private vulnerability
reporting once the repository is public. Until that channel exists, contact the repository owner
privately and include:

- affected revision;
- minimal reproduction without real credentials or customer data;
- expected and observed behavior;
- likely impact and preconditions;
- whether public disclosure has already occurred.

The project will acknowledge a complete report within five business days where possible. Response
and remediation timelines are targets, not an SLA, until a staffed security process exists.

## Scope

High-priority reports include authentication or policy bypass, credential/content leakage, SSRF,
request smuggling, cross-tenant exposure, unsafe retry after commitment, unbounded resource use,
and release or dependency compromise.

Reports about unsupported production deployment, social engineering, denial of service requiring
unrealistic local privileges, or findings without a reproducible security boundary may be closed as
out of scope.

## Safe research

Use synthetic data and dedicated low-privilege credentials. Do not test third-party providers or
deployments without authorization. Never include secrets, prompts, responses, personal data, or
active exploit payloads in public artifacts.
