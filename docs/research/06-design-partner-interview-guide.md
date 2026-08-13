# Design-Partner Interview Guide

**Purpose:** Discover actual work, incidents, and purchase behavior without pitching the proposed feature set.

## Target sample

- 5 AI-native startups with meaningful production inference traffic
- 5 mid-market platform teams serving multiple application teams
- 5 regulated or privacy-sensitive organizations
- Across the sample: platform engineering, SRE, security/privacy, FinOps, and engineering leadership

## Screening

1. How many AI applications, providers, self-hosted endpoints, and internal teams are in production?
2. Who owns model access and its reliability/security/cost?
3. What gateway, proxy, SDK layer, or cloud platform is used now?
4. Has the participant personally handled a related production or governance problem in the last six months?

## Core interview

1. Walk through the last time an application added or changed a model provider.
2. What broke or required unexpected manual work?
3. Describe the last provider outage, 429 storm, partial stream, timeout, or cost incident.
4. How was the incident detected, diagnosed, and resolved? How long did it take?
5. What AI request/response data may not leave the environment or enter logs?
6. What evidence does security require before approving a shared gateway?
7. How are model permissions, credentials, quotas, and spend attributed today?
8. Which parts of the current stack are most difficult to operate or upgrade?
9. What have you tried? Why was it kept, removed, or rejected?
10. What would cause you to replace the current solution in the next twelve months?
11. Who approves that change, who blocks it, and which budget pays?
12. What happens if the open-source project or vendor stops maintenance?

## Concept tests after problem discovery

Only after the participant has described real work, test these separately:

- metadata-only, no-content-by-default telemetry;
- standalone single-binary data plane;
- public compatibility and streaming conformance evidence;
- predictable no-replay-after-visible-output behavior;
- signed configuration and release provenance;
- optional enterprise control plane.

Ask: “What would this replace, what new risk would it introduce, and would you run a pilot?” Avoid asking whether the idea “sounds useful.”

## Evidence capture

- segment, role, company scale, environment;
- concrete incident/workflow;
- frequency, duration, people involved, cost/impact;
- current alternative and switching cost;
- security/procurement constraints;
- verbatim short quote with permission;
- pilot commitment strength: none, curious, technical evaluation, named owner/timeline, or written commitment.

## Pass criteria

R0 passes only if at least five participants across a coherent segment describe the same repeated costly job without being led. R4 requires at least three credible teams to test the same narrow use case with a named technical owner and agreed success measures.
