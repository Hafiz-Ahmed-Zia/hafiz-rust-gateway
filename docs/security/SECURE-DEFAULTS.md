# Secure Defaults Contract

**Status:** Proposed; each default requires an executable test before release

| Area | Default | Unsafe override behavior |
| --- | --- | --- |
| Listener | Loopback for generated local config; explicit bind required for external interfaces | Warning plus explicit config; production profile requires TLS/auth |
| Authentication | Required for all data-plane routes | Public route must be individually declared and audited |
| TLS upstream | Certificate and hostname verification enabled | Development-only flag, loud startup/runtime warning, excluded from production profile |
| Provider secrets | Environment/file/provider references | Inline secret rejected or redacted and discouraged according to final schema |
| Telemetry content | Prompts, responses, tool args/results, auth headers disabled | Field-level opt-in, redaction, sampling, destination warning, audit record |
| Logs | Structured metadata allowlist | No arbitrary request/response header dumping |
| Configuration | Strict schema; unknown security fields rejected | Compatibility mode cannot weaken security fields silently |
| Reload | Validate/compile completely, then atomic activate | Invalid update retains last-known-good and emits failure evidence |
| Upstreams | Trusted operator configuration; private/link-local/metadata denied by policy | Explicit CIDR/host allow with security warning and audit |
| Redirects | Disabled or same-policy revalidation | Cross-host redirect requires explicit trusted policy |
| Request/event size | Finite conservative limits | Increase is explicit and surfaced by `check`/`doctor` |
| Queues/buffers | Finite, documented capacities | No unlimited setting in production profile |
| Retries | Bounded; eligible errors before commitment only | Post-commit replay unavailable in v1 |
| Timeouts | Separate finite phase and end-to-end limits | Infinite timeout unavailable in production profile |
| Policy service failure | Fail closed | Fail-open must be route-specific, explicit, time-bounded, and audited |
| Telemetry service failure | Traffic continues within bounded buffer/drop policy | Loss/backlog must be observable |
| Plugins | No arbitrary native plugins | Future extension mechanisms capability- and resource-bounded |
| Admin/debug endpoints | Disabled or loopback/authenticated | External exposure requires explicit bind, auth, and redaction |
| Panic/debug output | No request content/secrets; backtrace off in normal release | Debug profile clearly non-production |

## Verification

- canary secrets and canary prompt content injected into tests must never appear in default output;
- configuration mutation tests verify unknown/ambiguous fields fail;
- SSRF suite covers IP literals, alternate encodings, redirects, DNS changes, and cloud metadata ranges;
- fuzz/property tests enforce parser and state bounds;
- release acceptance checks production profile cannot enable known development-only overrides;
- documentation examples use the same secure defaults as released configuration.
