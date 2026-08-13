# Protocol and Provider Compatibility

**Status:** Proposed

## Compatibility tiers

| Tier | Meaning | Required evidence |
| --- | --- | --- |
| Native | Maintained provider adapter with intentional normalization | Offline fixtures, differential tests, live canary tests, published deviations |
| Compatible | Tested OpenAI-compatible subset | Endpoint-specific fixture suite and deviations |
| Pass-through | Gateway applies transport/auth/policy with minimal interpretation | Header/body safety tests and provider-owned semantics |
| Experimental | Incomplete or unstable | Clearly versioned, excluded from stability promise |

## Endpoint scope proposal

### Phase 0

- OpenAI-style `POST /v1/chat/completions`, streaming and non-streaming
- Narrow OpenAI Responses API subset sufficient to exercise typed streaming events
- Generic OpenAI-compatible upstream
- Health/readiness endpoints outside the provider API namespace

### Phase 1 candidate

- Expand Responses API coverage based on official event fixtures
- Anthropic native Messages adapter if evidence supports it
- Embeddings only if required by design partners

### Deferred

- Realtime WebSocket/WebRTC
- image, audio, video, fine-tuning, batch, files
- MCP and A2A
- provider-hosted tools whose semantics cannot be preserved

## Matrix dimensions

Every provider/endpoint entry must cover:

- authentication and base URL;
- request fields and rejected/ignored fields;
- model aliases;
- non-streaming response;
- streaming event types and ordering;
- tool/function calls;
- structured output;
- usage/token accounting and whether authoritative;
- error mapping and rate-limit hints;
- cancellation behavior;
- idempotency/replay behavior;
- maximum body/event sizes;
- provider-specific headers;
- tested version/date and live-test frequency.

## Versioning rule

Provider behavior can change independently. The compatibility matrix is release-versioned. A green test from an older provider/API version is not permanent proof.

## Honesty rule

“OpenAI compatible” always names the endpoint and subset. It never implies full compatibility across every OpenAI API, stream event, tool, or modality.
