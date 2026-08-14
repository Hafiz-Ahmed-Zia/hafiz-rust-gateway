# New Ideas: Can Anything Score 90+?

> **Honest answer: No.** A 90+ requires near-perfection across ALL 9 dimensions simultaneously — massive pain, zero competitors, easy to build solo, huge grant appeal, AND strong commercial revenue. These criteria are inherently contradictory: if something has massive enterprise pain AND no competitors, venture-backed startups have already found it.

> **However**, my fresh research surfaced 3 genuinely strong ideas that score **78-82** — significantly higher than the previous best of 74. The key unlock: **August 2026 has 2-3 "perfect storm" problems that didn't exist even 6 months ago.**

---

## Why 90+ Is Structurally Impossible

| Requirement | Tension |
|---|---|
| Massive pain (18+/20) + No competitors (13+/15) | If the pain is that extreme, funded startups already exist |
| High grants (6+/7) + High commercial (4+/5) | Grant funders want "digital commons"; commercial buyers want proprietary advantage |
| Solo feasibility (9+/10) + Deep defensibility (9+/10) | If one person can build it, one person can clone it |
| 6-month proof (7+/8) + 3-5yr demand (14+/15) | If it's impressive fast, it may be a feature, not a platform |

**The realistic ceiling for a solo AI-assisted founder is ~82-85.** Here are the strongest new ideas I found.

---

## New Idea A: MCP Security Hardening Proxy
**Score: 82/100 · Confidence: High**

### The Perfect Storm (August 2026)

MCP (Model Context Protocol) has become the universal standard for connecting AI agents to tools. But its security model is fundamentally broken:

- **[VERIFIED]** Tool poisoning attacks have "high success rates across various LLM architectures" — attackers hide malicious instructions in tool metadata
- **[VERIFIED]** "Rug pull" attacks swap benign tools for malicious ones after initial approval
- **[VERIFIED]** Command injection and path traversal vulnerabilities are widespread in MCP servers
- **[VERIFIED]** OWASP published an "MCP Top 10" for 2026 — confirming this is a recognized attack surface
- **[VERIFIED]** Industry is "increasingly moving toward MCP Gateways" as enterprise infrastructure

### What It Builds

A lightweight, open-source security proxy that sits between AI agents (Claude, GPT, Cursor, etc.) and MCP servers. It:

1. **Scans tool metadata** for hidden prompt injection / poisoning instructions
2. **Pins and signs tool schemas** — detects "description drift" (rug pulls)
3. **Enforces policy-as-code** — YAML rules for allowed actions, file paths, network access
4. **Sandboxes execution** — blocks command injection, path traversal
5. **Logs everything** — immutable audit trail of every tool call
6. **Works as a drop-in proxy** — no changes to the agent or the MCP server required

### Why This Scores So High

| Criterion | Score | Rationale |
|-----------|-------|-----------|
| Pain & urgency (20) | **18** | [VERIFIED] Tool poisoning is the #1 MCP vulnerability. Every company deploying AI agents is exposed *right now*. |
| 3-5yr demand (15) | **14** | MCP adoption is exploding; agentic AI is only growing. This problem gets worse. |
| Competitor gap (15) | **10** | Competitors exist (MCP-Scan, MCP Shield, mcp-firewall) BUT they are all early-stage, fragmented, and narrowly scoped. No dominant solution yet. |
| OSS adoption (10) | **9** | Security proxy is exactly the kind of tool developers install immediately. Drop-in, no lock-in. |
| Differentiation (10) | **7** | Combined metadata scanning + schema pinning + policy enforcement + sandboxing in one proxy. Individual features exist; the unified product doesn't. |
| Solo feasibility (10) | **9** | Proxy + YAML policy engine + metadata scanner. Very tractable for AI-assisted Rust/Go development. |
| 6-month proof (8) | **7** | Demo: "We scanned 100 popular MCP servers and found poisoned tool descriptions in 23 of them" — instant virality. |
| Grants (7) | **5** | Strong fit for security-focused grants (STF, Project Glasswing/Anthropic, NSF PESOSE). |
| Commercial (5) | **3** | Enterprise "MCP gateway" with RBAC and compliance features. Clear buyer (CISO office). |

### Existing Competitors (Honest Assessment)

| Tool | What It Does | Gap |
|------|-------------|-----|
| **MCP-Scan (Invariant Labs)** | Detects tool poisoning, rug pulls | Scanner only — no runtime enforcement, no policy engine |
| **MCP Shield** | Security proxy with CVE rules | Narrow ruleset; no schema pinning or governance |
| **mcp-firewall** | Deterministic policy proxy ("iptables for MCP") | Policy enforcement only — no metadata scanning, no poisoning detection |
| **Microsoft MCP Gateway** | Enterprise reverse proxy with OAuth/RBAC | Enterprise-focused, Kubernetes-only; not developer/OSS-first |
| **agent-audit** | CLI config scanner | Static analysis only; no runtime protection |

**The gap:** Nobody combines scanning + enforcement + monitoring into a single, developer-friendly, drop-in proxy. This is exactly the pattern that made tools like Nginx, Traefik, and Envoy successful — a unified proxy that handles multiple concerns.

### Primary Users & Buyers
- **Users:** Every developer using Claude Desktop, Cursor, Windsurf, or any MCP-enabled AI tool
- **Paying buyers:** Enterprises deploying AI agents with MCP integrations; AI platform companies needing security certification

### Minimum Impressive Prototype (6 months)
- Drop-in Rust proxy between agent and MCP servers
- Metadata poisoning scanner (catches hidden instructions in tool descriptions)
- Schema pinning (alerts on tool definition changes)
- YAML policy engine (allowlist/blocklist for file paths, network, shell commands)
- Audit logging with tamper-evident records
- Public scan of 200+ popular MCP servers with vulnerability report

### Main Risks
1. **MCP protocol may evolve** to include native security — Anthropic could add signing/verification to the spec itself
2. **Fragmented competitors** could consolidate quickly
3. **Adoption friction** — developers must route traffic through a proxy
4. **MCP may not become THE standard** — if A2A or another protocol wins, the proxy loses relevance

### 30-Day Disproof Experiment
Scan 50 popular MCP servers from the community registry for tool poisoning and command injection vulnerabilities. If fewer than 5 are vulnerable, the urgency is overstated.

---

## New Idea B: AI-Code Verification Gate ("Did the AI Actually Get It Right?")
**Score: 78/100 · Confidence: High**

### The Perfect Storm

The "vibe coding" crisis has created a massive, verified problem:

- **[VERIFIED]** AI-generated PRs have 75% more logic and correctness errors than human-created ones
- **[VERIFIED]** 45% of AI-generated code contains security vulnerabilities
- **[VERIFIED]** AI generates both code AND tests, so tests pass despite bugs ("grading its own homework")
- **[VERIFIED]** Code duplication has risen up to 81% in vibe-coded codebases
- **[VERIFIED]** Mutation testing is now the "gold standard" for validating AI-generated test quality

### What It Builds

A CI/CD gate specifically designed for AI-generated code. It:

1. **Detects AI-generated changes** in a PR (via heuristics or commit metadata)
2. **Runs mutation testing** on AI-generated tests to check if they actually catch bugs
3. **Performs differential behavioral testing** — does the change actually do what the PR says?
4. **Checks for common AI anti-patterns** — tautological tests, encoded assumptions, placeholder logic
5. **Produces an evidence report** with mutation score, behavioral diff, and confidence level
6. **Blocks merge if evidence is insufficient** — like a type-checker, but for correctness

### Scoring

| Criterion | Score | Rationale |
|-----------|-------|-----------|
| Pain & urgency (20) | **17** | [VERIFIED] The "validation bottleneck" from AI-generated code is the #1 developer pain point in 2026. |
| 3-5yr demand (15) | **14** | AI code generation is only accelerating. Every team will need automated verification. |
| Competitor gap (15) | **10** | Qodo/CodiumAI, Diffblue exist but focus on test *generation*, not test *verification*. Mutation testing tools exist but aren't integrated into an "AI verification gate." |
| OSS adoption (10) | **8** | GitHub Action / CI integration makes adoption low-friction. |
| Differentiation (10) | **7** | "Mutation testing as an AI quality gate" is a specific, defensible position. |
| Solo feasibility (10) | **8** | Orchestrating existing tools (mutmut, Stryker, semgrep) into a unified gate. AI-assisted coding is perfect for this. |
| 6-month proof (8) | **6** | Demo: Run against 20 popular OSS repos' Copilot-generated PRs. Show "we caught 34 bugs that passed all existing tests." |
| Grants (7) | **4** | Moderate fit — software quality/security angle for STF/NLnet. |
| Commercial (5) | **4** | Enterprise CI/CD add-on with clear buyer (VP Eng, CISO). |

### Main Risks
1. **Mutation testing is slow** — could be a dealbreaker for large codebases
2. **"Just a CI action" problem** — similar to original #1 candidate's risk
3. **Qodo/CodiumAI could pivot** into verification (they're close)
4. **LLM providers may build this in** — Copilot/Cursor adding native verification

### 30-Day Disproof Experiment
Run mutmut on 10 repos' AI-generated PRs. If mutation scores are consistently >80% (meaning AI tests are already good), the value proposition collapses.

---

## New Idea C: Agent Budget Enforcer & Cost Firewall
**Score: 72/100 · Confidence: Medium-High**

### The Problem
- **[VERIFIED]** AI agents enter infinite loops costing $30-$100+ per task before humans notice
- **[VERIFIED]** Agentic AI costs scale 5x-30x compared to chatbot interactions
- **[VERIFIED]** Most agent frameworks lack built-in "circuit breakers"

### Why It Scores Lower
- **[VERIFIED]** LiteLLM already has budget enforcement built in
- **[VERIFIED]** AgentBudget is an open-source library for this exact problem
- **[VERIFIED]** Langfuse provides cost attribution and tracing
- The gap is narrower than it appears — existing tools cover 70%+ of the need

### Verdict
Real problem, but existing tools (LiteLLM, AgentBudget, Langfuse) already provide reasonable solutions. The remaining gap is too small for a new project to differentiate on.

---

## Updated Rankings: All Ideas Including New Ones

| Rank | Project | Score | Change |
|------|---------|-------|--------|
| **1** | **🆕 MCP Security Hardening Proxy** | **82** | **NEW** |
| **2** | **🆕 AI-Code Verification Gate** | **78** | **NEW** |
| 3 | AI Provider Conformance Lab (prev #8) | 74 | — |
| 4 | 🆕 Agent Budget Enforcer | 72 | NEW |
| 5 | Deterministic Agent Flight Recorder (prev #5) | 71 | — |
| 6 | Runtime Exposure & VEX Evidence Engine (prev #7) | 68 | — |
| 7 | Agent Action Safety Kernel (prev #4) | 66 | — |

---

## Head-to-Head: New #1 vs Old #1

| Dimension | MCP Security Proxy (82) | AI Conformance Lab (74) |
|-----------|------------------------|------------------------|
| **Pain RIGHT NOW** | 🔴 Critical — active exploits, OWASP Top 10 published | 🟡 Annoying — provider inconsistencies cause bugs |
| **Who cares?** | Every AI agent user + every CISO | AI platform engineers only |
| **Competitors** | 5+ early-stage tools (fragmented) | Zero direct competitors |
| **Grant fit** | Excellent (security infra) | Weak (not "infrastructure") |
| **Commercial path** | Clear (enterprise MCP gateway) | Unclear (consulting?) |
| **Solo 6-month demo** | Scan 200 MCP servers, find 50+ vulns → instant press | Test 6 providers, find 47 inconsistencies → HN front page |
| **Risk of being a "feature"** | Medium — Anthropic could add security to MCP spec | High — any gateway could add conformance tests |
| **Defensibility** | Medium — unified proxy is more than a feature | Low — "just a test suite" |

### Verdict

**MCP Security Proxy wins on almost every dimension** except competitor density (it has more early competitors than the Conformance Lab). But its competitors are fragmented and narrowly scoped, leaving a clear gap for a unified, developer-friendly solution.

---

## Final Updated Recommendation

> [!IMPORTANT]
> **Build the MCP Security Hardening Proxy.**
> 
> It scores 82/100 — the highest achievable for a solo founder in August 2026. The timing is perfect: MCP is the new universal standard, tool poisoning is an active threat, OWASP just published the MCP Top 10, enterprises are scrambling for governance, and no unified solution exists.
> 
> The previous recommendation (#8 Conformance Lab, 74) remains a strong fallback if you want lower risk and fewer competitors.

### Why 90+ Remains Impossible

Even the MCP Security Proxy loses points because:
- Competitors exist (not zero-gap): -5 points
- Commercial sustainability unclear for OSS (will enterprises pay?): -2 points  
- Protocol could change (Anthropic owns MCP spec): -1 point
- Not a "platform" — it's an infrastructure tool: -2 points

**These are structural limitations of being a solo founder building open-source.** A VC-funded team with 10 engineers, enterprise sales, and $20M would score higher on commercial/defensibility, but that's not the scenario.
