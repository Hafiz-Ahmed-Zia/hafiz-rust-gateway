# 1. Executive verdict

**Pursue a Proof-Carrying Change Verification Engine for AI-assisted software development. Do not spend the next six months building Hafiz as a broad general-purpose AI gateway.**

The winning concept scores **88/100**. That is the highest score I can justify. **No opportunity reached 90/100**, because every serious market considered has either strong incumbents, a difficult adoption bottleneck, or an unresolved technical oracle problem.

**Hafiz Rust Gateway scores 78/100 and ranks #10 out of 11 total candidates.** Its underlying reliability thesis is valid, but the current product envelope is too broad and too crowded. Rust, low latency, self-hosting, retries, fallbacks, routing, and OpenAI-compatible APIs are no longer meaningful differentiation by themselves.

The strongest current market signal is the widening gap between **code generation throughput and trustworthy verification**:

* DORA’s 2025 research reported that 90% of surveyed technology professionals used AI at work and more than 80% perceived productivity gains, yet 30% reported little or no trust in AI-generated output. DORA’s broader conclusion was that AI amplifies the strengths and weaknesses of the surrounding engineering system rather than repairing weak controls. Sources: [[https://dora.dev/dora-report-2025/](https://dora.dev/dora-report-2025/)](https://dora.dev/dora-report-2025/) and [[https://dora.dev/ai/gen-ai-report/](https://dora.dev/ai/gen-ai-report/)](https://dora.dev/ai/gen-ai-report/). ([[Dora](https://dora.dev/dora-report-2025/)][1])
* METR’s randomized study found that experienced open-source developers working in familiar repositories took **19% longer** with early-2025 AI tools, despite expecting AI to accelerate them. That does not prove all AI coding is slower; it shows that generation speed does not automatically produce end-to-end engineering speed. Source: [[https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/)](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/). ([[METR](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/)][2])
* A recent security benchmark reported a large separation between functional correctness and security correctness in AI-generated code. That work is a **preprint**, not settled industry evidence, but it reinforces the need for executable verification rather than another language-model review comment. Source: [[https://arxiv.org/html/2512.03262](https://arxiv.org/html/2512.03262)](https://arxiv.org/html/2512.03262). ([[arXiv](https://arxiv.org/html/2512.03262)][3])
* On August 12, 2026, Reuters reported that CodeRabbit raised $143 million at a $1.5 billion valuation and was processing roughly two million reviews weekly for 17,000 customers. Those figures validate willingness to spend on AI-era code assurance, although they do not prove that CodeRabbit or the broader review category has solved behavioral verification. Source: [[https://www.reuters.com/technology/ai-code-review-platform-coderabbit-valued-15-billion-latest-funding-round-2026-08-12/](https://www.reuters.com/technology/ai-code-review-platform-coderabbit-valued-15-billion-latest-funding-round-2026-08-12/)](https://www.reuters.com/technology/ai-code-review-platform-coderabbit-valued-15-billion-latest-funding-round-2026-08-12/). ([[Reuters](https://www.reuters.com/technology/ai-code-review-platform-coderabbit-valued-15-billion-latest-funding-round-2026-08-12/)][4])

The proposed winner is **not another AI code reviewer**. Its core claim would be:

> For a given change, repository state, environment and declared policy, this signed bundle proves exactly which checks were executed, what behavior was compared, which mutations were killed, which properties were exercised, and what counterexamples were found.

That is deliberately narrower than “this code is correct.” Tests cannot generally prove complete program correctness. The product must sell **reproducible evidence**, not mathematical certainty.

---

## Evidence discipline

I use four evidence classes throughout:

* **Verified fact:** official specifications, public documentation, public repository issues, security advisories or reputable reporting.
* **Vendor or association claim:** a product claim, commissioned survey, vendor incident analysis or vendor-supplied metric.
* **Estimate:** my scoring, timetable, cost, adoption and execution judgement.
* **Hypothesis:** the proposed unmet gap, product wedge or willingness-to-pay thesis that still requires validation.

The scores are decision tools, not scientific measurements. A three-point difference is meaningful; a one-point difference usually is not.

---

# 2. Ranked table: top 10 new opportunities

|   Rank | Project opportunity                                                  | Urgency /20 | 3–5 yr demand /15 | Gap /15 | OSS /10 | Defensibility /10 | Solo feasibility /10 | 6-mo proof /8 | Funding /7 | Business /5 | **Total** | Confidence  |
| -----: | -------------------------------------------------------------------- | ----------: | ----------------: | ------: | ------: | ----------------: | -------------------: | ------------: | ---------: | ----------: | --------: | ----------- |
|  **1** | **Proof-Carrying Change Verification Engine**                        |          18 |                15 |      12 |       9 |                 8 |                    7 |             8 |          6 |           5 |    **88** | Medium-high |
|  **2** | **Cross-System Non-Human Identity and Delegation Attack-Path Graph** |          17 |                15 |      13 |       9 |                 9 |                    6 |             7 |          6 |           5 |    **87** | Medium      |
|  **3** | **Postgres Migration Safety and Workload Replay Lab**                |          17 |                14 |      13 |       9 |                 8 |                    8 |             8 |          4 |           5 |    **86** | High        |
|  **4** | **Agent Action Safety Kernel: Semantic Transactions for Tool Calls** |          16 |                15 |      13 |       9 |                 9 |                    6 |             7 |          6 |           4 |    **85** | Medium      |
|  **5** | **Deterministic Agent Flight Recorder, Replay and Fault Injection**  |          16 |                15 |      11 |       9 |                 8 |                    7 |             8 |          6 |           4 |    **84** | Medium      |
|  **6** | **Post-Quantum Cryptography Migration Workbench**                    |          14 |                15 |      12 |       9 |                 8 |                    7 |             7 |          7 |           4 |    **83** | Medium-high |
|  **7** | **Runtime Exposure and VEX Evidence Engine**                         |          17 |                13 |      11 |       9 |                 7 |                    7 |             7 |          7 |           4 |    **82** | Medium      |
|  **8** | **AI Provider Conformance and Failure-Semantics Lab**                |          15 |                14 |      10 |       9 |                 8 |                    8 |             8 |          5 |           4 |    **81** | High        |
|  **9** | **AI Artifact Provenance and License-Evidence Compiler**             |          13 |                15 |      11 |      10 |                 8 |                    7 |             7 |          6 |           3 |    **80** | Medium-low  |
| **10** | **Local-First Sync Correctness and Schema-Evolution Lab**            |          12 |                13 |      12 |      10 |                 9 |                    7 |             8 |          4 |           2 |    **77** | Medium-low  |

## Categories deliberately rejected

A generic AI gateway did not make the top 10 because LiteLLM, Envoy AI Gateway, Portkey and TensorZero already cover much of the basic gateway surface. LiteLLM is also moving performance-sensitive routing, retry and streaming paths toward Rust, directly weakening “implemented in Rust” as a moat. Sources: [[https://docs.litellm.ai/docs/routing](https://docs.litellm.ai/docs/routing)](https://docs.litellm.ai/docs/routing), [[https://docs.litellm.ai/blog/litellm-rust-launch](https://docs.litellm.ai/blog/litellm-rust-launch)](https://docs.litellm.ai/blog/litellm-rust-launch), [[https://aigateway.envoyproxy.io/](https://aigateway.envoyproxy.io/)](https://aigateway.envoyproxy.io/). ([[LiteLLM](https://docs.litellm.ai/docs/routing)][5])

Generic agent observability is already becoming standardized through OpenTelemetry’s GenAI semantic conventions, while Phoenix, AgentOps and commercial observability platforms occupy the trace-and-evaluation layer. The remaining opportunity is deterministic reproduction, not another trace dashboard. Sources: [[https://opentelemetry.io/blog/2026/genai-observability/](https://opentelemetry.io/blog/2026/genai-observability/)](https://opentelemetry.io/blog/2026/genai-observability/) and [[https://arize.com/docs/phoenix](https://arize.com/docs/phoenix)](https://arize.com/docs/phoenix). ([[OpenTelemetry](https://opentelemetry.io/blog/2026/genai-observability/)][6])

A generic software-supply-chain graph would collide with GUAC. A generic compliance dashboard would collide with OSCAL ecosystems, Prowler and existing compliance automation. A generic agent sandbox would enter a heavily funded field including E2B and several cloud platforms. The opportunities retained below sit **beside** those incumbents and produce evidence they can ingest, rather than rebuilding their complete platforms. ([[guac](https://guac.sh/)][7])

---

# 3. Hafiz Rust Gateway’s comparative rank

## Score

| Criterion                         |      Score |
| --------------------------------- | ---------: |
| Current problem intensity         |      16/20 |
| Future demand                     |      14/15 |
| Unresolved incumbent gap          |       8/15 |
| Open-source adoption potential    |       9/10 |
| Differentiation and defensibility |       7/10 |
| Solo-founder feasibility          |       8/10 |
| Six-month demonstration           |        8/8 |
| Grant and sponsorship appeal      |        4/7 |
| Commercial sustainability         |        4/5 |
| **Total**                         | **78/100** |

**Overall rank: #10 of 11 candidates.**

Nine new opportunities rank above it. The local-first correctness laboratory ranks slightly below it, although that one-point difference is not decisive.

## What remains genuinely valuable in Hafiz

The reliability problem is real. Public LiteLLM issues describe difficult edge cases involving retries, fallback cycles, malformed tool arguments, midstream failures and continuation behavior. One July 2026 issue reported a retry/fallback cycle that produced 79,858 attempts and terminated a pod; that is a public user report, not independently verified evidence, but it demonstrates the class of failure Hafiz targets. Sources:

* [[https://github.com/BerriAI/litellm/issues/35303](https://github.com/BerriAI/litellm/issues/35303)](https://github.com/BerriAI/litellm/issues/35303)
* [[https://github.com/BerriAI/litellm/issues/22296](https://github.com/BerriAI/litellm/issues/22296)](https://github.com/BerriAI/litellm/issues/22296)
* [[https://github.com/BerriAI/litellm/issues/18229](https://github.com/BerriAI/litellm/issues/18229)](https://github.com/BerriAI/litellm/issues/18229) ([[GitHub](https://github.com/berriai/litellm/issues/35303)][8])

A 2026 preprint on verified tool calls also identifies non-atomic failures—such as a timeout after an external side effect—as a distinct reliability problem and shows that verify-before-retry logic can reduce duplicate actions. This is preliminary research, but it supports Hafiz’s emphasis on retry correctness. Source: [[https://arxiv.org/abs/2608.02645](https://arxiv.org/abs/2608.02645)](https://arxiv.org/abs/2608.02645). ([[arXiv](https://arxiv.org/html/2608.02645v1?utm_source=chatgpt.com)][9])

## What is not defensible

* **Rust is not the moat.** LiteLLM is moving relevant hot paths toward Rust. Envoy already provides a mature high-performance proxy foundation. ([[LiteLLM](https://docs.litellm.ai/blog/litellm-rust-launch)][10])
* **Generic routing is not the moat.** Weighted routing, health-based routing, retries, fallbacks, budgets and provider abstraction are rapidly becoming table stakes.
* **No mandatory database is a deployment advantage, not a market category.** Customers needing audit, accounting, policy history or analytics will eventually need durable state somewhere.
* **Low latency is necessary but insufficient.** A gateway that saves several milliseconds but requires an organization to replace an existing production traffic layer has weak adoption leverage.
* **Provider capability matrices are copyable.** A continuously maintained, executable failure corpus is harder to copy than a static capability table.
* **Routing receipts are interesting only when an external policy, auditor or incident workflow consumes them.** A cryptographic receipt without a verifier or policy ecosystem is merely structured logging.

## Decision

**Continue the broad gateway? No.**

**Narrow it? Yes.**

**Pivot within the domain? Yes: turn it into the #8 opportunity, an AI Provider Conformance and Failure-Semantics Lab.**

**Contribute to incumbents? Yes.** Upstream adapters and failing test cases into LiteLLM, Envoy AI Gateway and TensorZero. The goal should be to become the neutral test standard those projects run, not another proxy they must fight.

**Completely switch projects? Yes, unless the narrowed conformance project passes a strict validation gate.** The recommended primary project remains the Proof-Carrying Change Verification Engine.

## Evidence that would reverse this judgement

Hafiz would move from approximately 78 toward 86–90 if, within a limited validation period, it achieved most of the following:

1. Three independent gateway or agent-framework projects integrate the conformance suite into CI.
2. The suite reproduces at least 20 meaningful cross-provider divergences across eight provider families, including at least five that can cause duplicate side effects, corrupted streams, incorrect billing or silent tool-call failure.
3. At least one gateway maintainer accepts an upstream fix based on a Hafiz test.
4. Two production teams commit to paying for continuous private-provider testing or managed conformance infrastructure.
5. The Rust reference data plane demonstrates material tail-latency or memory benefits without compromising failure semantics.
6. Routing receipts are consumed by a real audit, incident-response or compliance workflow rather than only displayed in a dashboard.

Without those results, the gateway thesis is technically respectable but commercially undifferentiated.

## Grant and sponsored model-access attractiveness

In its generic gateway form, Hafiz is only **moderately attractive**, approximately **5/10**, for open-source AI-development grants. As a neutral security and failure-conformance project, it rises to roughly **7/10**.

OpenAI’s defensive cybersecurity programs offer API credits for security work and critical open-source infrastructure; Anthropic’s Project Glasswing advertises substantial usage credits and support for open-source security; the Linux Foundation announced $12.5 million in 2026 grant funding for open-source security work. These are genuine funding pools, but they do not imply Hafiz would be selected. Sources:

* [[https://openai.com/index/trusted-access-for-cyber/](https://openai.com/index/trusted-access-for-cyber/)](https://openai.com/index/trusted-access-for-cyber/)
* [[https://openai.com/index/openai-cybersecurity-grant-program/](https://openai.com/index/openai-cybersecurity-grant-program/)](https://openai.com/index/openai-cybersecurity-grant-program/)
* [[https://www.anthropic.com/project-glasswing](https://www.anthropic.com/project-glasswing)](https://www.anthropic.com/project-glasswing)
* [[https://www.linuxfoundation.org/press/linux-foundation-announces-12.5-million-in-grant-funding-from-leading-organizations-to-advance-open-source-security](https://www.linuxfoundation.org/press/linux-foundation-announces-12.5-million-in-grant-funding-from-leading-organizations-to-advance-open-source-security)](https://www.linuxfoundation.org/press/linux-foundation-announces-12.5-million-in-grant-funding-from-leading-organizations-to-advance-open-source-security) ([[OpenAI](https://openai.com/index/trusted-access-for-cyber/)][11])

Sponsored model access is plausible because provider access is necessary for conformance testing. The conflict is obvious: a provider may not wish to fund a project that publishes its failures. An embargoed responsible-disclosure process, followed by public normalized results, would improve the proposition.

### Repository proof required before applying

A grant application made before these exist would be premature:

1. An executable specification containing at least 80–100 versioned scenarios.
2. Public results across at least six provider families and several local OpenAI-compatible servers.
3. Property-based tests for retry, fallback, cancellation and stream-state transitions.
4. A fault-injection proxy covering disconnects, delayed responses, malformed chunks, duplicated chunks, partial tool calls and ambiguous post-dispatch timeouts.
5. At least three responsibly disclosed, reproducible provider or gateway failures.
6. Benchmarks against LiteLLM and Envoy AI Gateway using identical workloads.
7. An explicit threat model, `SECURITY.md`, fuzzing, reproducible releases and signed provenance.
8. At least one independent integration or upstream pull request accepted by an incumbent.

---

# 4. Detailed analysis of the top three opportunities

## #1 — Proof-Carrying Change Verification Engine — 88/100

### Concept

An open-source local and CI execution engine that converts a code change into a **signed, reproducible change-evidence bundle**.

The bundle would describe:

* The exact source revision and diff.
* The affected symbols, interfaces, data contracts and runtime paths.
* The existing tests selected and executed.
* Mutations introduced into changed behavior and whether tests killed them.
* Property-based, fuzz and differential tests executed.
* Security and performance checks executed.
* Deterministic seeds, containers, toolchain hashes and network policy.
* Any minimized counterexamples.
* The identity of the runner and the signature covering the evidence.

SLSA, in-toto and Sigstore already provide standards and tools for provenance and attestations. In-toto even defines a test-result predicate. What is missing is an opinionated engine that plans and produces strong behavioral evidence for a source-code change. Sources:

* [[https://slsa.dev/spec/draft/build-provenance](https://slsa.dev/spec/draft/build-provenance)](https://slsa.dev/spec/draft/build-provenance)
* [[https://in-toto.io/](https://in-toto.io/)](https://in-toto.io/)
* [[https://in-toto.io/attestation/test-result/](https://in-toto.io/attestation/test-result/)](https://in-toto.io/attestation/test-result/)
* [[https://docs.sigstore.dev/cosign/signing/other_types/](https://docs.sigstore.dev/cosign/signing/other_types/)](https://docs.sigstore.dev/cosign/signing/other_types/) ([[SLSA](https://slsa.dev/spec/draft/build-provenance)][12])

### Exact users and buyers

**Initial users:** open-source maintainers, senior engineers, platform teams and teams merging substantial AI-generated code.

**Economic buyers:** heads of engineering, developer-platform teams, application-security leaders, regulated-product engineering leaders and companies purchasing AI coding tools at scale.

### Painful job being solved

The recurring job is not “review this code.” It is:

> Decide whether this change is safe enough to merge, using evidence that can be reproduced later.

Existing AI review tools primarily produce findings, suggestions and summaries. Conventional CI produces pass/fail results for whatever tests the repository already contains. Static analyzers detect known classes of defects. None automatically proves that the tests meaningfully exercised the changed behavior or that the change preserved important old behavior.

Reuters-reported funding for CodeRabbit demonstrates substantial spending in the review category. CodeRabbit and Qodo document broad review, security and integration capabilities, but their public product surfaces remain principally review and governance systems rather than deterministic behavioral attestation engines. Sources: [[https://docs.coderabbit.ai/](https://docs.coderabbit.ai/)](https://docs.coderabbit.ai/), [[https://docs.coderabbit.ai/security-agent](https://docs.coderabbit.ai/security-agent)](https://docs.coderabbit.ai/security-agent), [[https://www.qodo.ai/](https://www.qodo.ai/)](https://www.qodo.ai/). ([[CodeRabbit](https://docs.coderabbit.ai/)][13])

### Why open source matters

The runner needs access to source code, tests, build systems, internal schemas and sometimes production-like data. Enterprises will reasonably demand self-hosting, transparent check logic and reproducible results.

Open source also enables language-specific communities to build check adapters. A closed company cannot credibly maintain deep test, build, fuzzing and mutation semantics across every ecosystem alone.

### Proposed technical wedge

The core asset is a **change evidence graph**, not an LLM reviewer.

1. **Change impact graph:** diff → symbols → callers → contracts → tests → runtime entrypoints.
2. **Risk classifier:** identify authentication, authorization, serialization, migrations, concurrency, money movement, public APIs and irreversible operations.
3. **Evidence planner:** choose checks under an explicit time and compute budget.
4. **Hermetic execution:** pinned environments, deterministic seeds, controlled egress and captured toolchain hashes.
5. **Mutation testing focused on changed behavior:** demonstrate whether the tests can distinguish the proposed logic from plausible incorrect variants.
6. **Differential execution:** run old and new revisions against the same inputs and identify undeclared behavioral changes.
7. **Property and fuzz testing:** LLMs may propose properties and generators, but executable checks—not the LLM’s opinion—decide success.
8. **Signed attestation:** in-toto-compatible test result plus provenance and evidence references, signed through Sigstore or an enterprise key system.

The defensible asset eventually becomes the corpus connecting **change pattern → relevant invariants → effective checks → historical counterexamples**.

### Minimum impressive prototype

Support one ecosystem initially—preferably TypeScript—and demonstrate the following on 30–50 public changes:

1. A pull request passes its existing tests and an AI review.
2. The engine maps the changed functions to affected behavior.
3. Mutation or differential testing exposes a missing case.
4. The tool generates a minimized reproduction.
5. A signed evidence bundle can be verified locally.
6. The entire result appears as a GitHub Check, not another wall of review comments.

A successful demonstration should include several seeded faults and at least a few historical bugs reconstructed from public fixes.

### Credible six-month deliverable

* TypeScript and Python support.
* GitHub Action and local CLI.
* Containerized execution with deterministic configuration.
* Changed-code mutation testing.
* Differential HTTP/API and CLI testing.
* Adapters for `pytest`, Jest/Vitest and common static analyzers.
* Policy profiles for ordinary, security-sensitive and high-risk changes.
* In-toto/Sigstore evidence bundles.
* Public benchmark containing approximately 300 labelled changes.
* Caching and evidence-budget controls.
* Plugin SDK for additional languages and check engines.
* A deliberately simple web viewer; no large dashboard platform.

### Competitors and substitutes

CodeRabbit, Qodo, GitHub Copilot review, CodeQL, Semgrep, Sonar, Snyk, mutation-testing tools such as Stryker, fuzzing tools, test-impact tools, conventional CI and human review.

The strategy is to **compose** these tools and produce evidence about their execution, not replace every analyzer.

### Commercial model

The open-source edition should include the complete local runner, evidence format, policy engine, CLI and core adapters.

Paid layers can include:

* Hosted execution fleets and caching.
* Enterprise policy management.
* Long-term evidence retention.
* Private runners and isolated environments.
* Organization-wide risk analytics.
* Audit integrations and support.
* Curated high-assurance policy packs.

That preserves a non-crippled open-source edition while selling operational scale.

### Biggest risks

1. **The test-oracle problem:** generated tests can encode the same misunderstanding as the implementation.
2. **Compute overhead:** mutation, fuzz and differential testing can multiply CI cost.
3. **Noise:** a merge-blocking tool with false positives will be disabled.
4. **Cross-language scope:** build systems and runtime semantics can overwhelm a solo maintainer.
5. **Incumbent copying:** well-funded reviewers can add mutation and test-generation features.
6. **Overclaiming:** the word “proof” can create legal and technical backlash.

The mitigation is strict wording: the tool proves that declared evidence was produced under declared conditions. It does not prove complete correctness.

### Smallest experiment that could disprove it

Run the prototype against:

* 25 historical bug-fix changes reconstructed before the fix.
* 25 seeded AI-authored changes.
* Five active open-source repositories.
* Baselines consisting of current CI plus one leading AI review tool.

Kill or radically narrow the idea if:

* It catches fewer than 20% additional meaningful defects.
* The false-block rate exceeds 5%.
* Added execution cost is consistently more than twice existing CI without clear value.
* Fewer than three of ten maintainers keep the integration enabled after a two-week trial.
* Most detections depend on unverifiable LLM judgement rather than executable counterexamples.

### Path toward 90+

It crosses 90 only after demonstrating that it catches at least 20–25% of important defects missed by ordinary CI and review, at acceptable cost, while several maintainers voluntarily make the evidence gate mandatory.

**Confidence:** medium-high. The pain and budget are verified; the behavioral-evidence wedge remains a hypothesis.

---

## #2 — Cross-System Non-Human Identity and Delegation Attack-Path Graph — 87/100

### Concept

An open-source graph engine that continuously discovers and connects:

* Service accounts and workload identities.
* GitHub Actions and other CI identities.
* Cloud roles and OIDC trust relationships.
* Kubernetes service accounts.
* Secrets and secret-manager access.
* OAuth applications and third-party integrations.
* API keys and long-lived credentials.
* AI-agent identities and delegated human authority.
* Owners, sponsors, last use, expiry and offboarding state.
* Effective permissions and reachable assets.

Its central query would be:

> Through all direct, delegated and transitive relationships, what can this machine or agent identity actually reach, and why?

### Exact users and buyers

**Users:** cloud-security engineers, IAM teams, platform engineers, application-security teams, red teams and incident responders.

**Buyers:** CISOs, heads of cloud security, identity-security leaders and large engineering organizations.

### Painful job being solved

Organizations increasingly have identities that are not employees: CI workflows, service principals, SaaS integrations, bots, agents and short-lived workload credentials.

OWASP published a Non-Human Identities Top 10 covering issues such as improper offboarding, secret leakage, overprivilege, insecure authentication and environment isolation. Source: [[https://owasp.org/www-project-non-human-identities-top-10/2025/top-10-2025/](https://owasp.org/www-project-non-human-identities-top-10/2025/top-10-2025/)](https://owasp.org/www-project-non-human-identities-top-10/2025/top-10-2025/). ([[OWASP Foundation](https://owasp.org/www-project-non-human-identities-top-10/2025/top-10-2025/)][14])

A Cloud Security Alliance survey announced in March 2026 reported that 68% of responding organizations could not clearly distinguish AI-agent actions from human actions. This is an **association survey claim**, not an independently audited market census, but it indicates substantial governance anxiety. Source: [[https://cloudsecurityalliance.org/press-releases/2026/03/24/more-than-two-thirds-of-organizations-cannot-clearly-distinguish-ai-agent-from-human-actions](https://cloudsecurityalliance.org/press-releases/2026/03/24/more-than-two-thirds-of-organizations-cannot-clearly-distinguish-ai-agent-from-human-actions)](https://cloudsecurityalliance.org/press-releases/2026/03/24/more-than-two-thirds-of-organizations-cannot-clearly-distinguish-ai-agent-from-human-actions). ([[Cloud Security Alliance](https://cloudsecurityalliance.org/press-releases/2026/03/24/more-than-two-thirds-of-organizations-cannot-clearly-distinguish-ai-agent-from-human-actions)][15])

Security incidents reinforce the risk class. StepSecurity’s analysis of the `tj-actions/changed-files` compromise described maliciously modified GitHub Action tags and exposed CI secrets across a widely used dependency. Palo Alto Unit 42’s Salesloft Drift analysis described stolen OAuth tokens used to access customer Salesforce environments. These are vendor-authored incident analyses, but both describe concrete cross-system identity chains. Sources:

* [[https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised)](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised)
* [[https://unit42.paloaltonetworks.com/third-party-supply-chain-token-management/](https://unit42.paloaltonetworks.com/third-party-supply-chain-token-management/)](https://unit42.paloaltonetworks.com/third-party-supply-chain-token-management/) ([[StepSecurity](https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised)][16])

### Why existing solutions are insufficient

* SPIFFE/SPIRE issues and manages workload identities but does not attempt to provide complete cross-SaaS ownership, delegation and attack-path analysis. Source: [[https://spiffe.io/docs/latest/spiffe/concepts/](https://spiffe.io/docs/latest/spiffe/concepts/)](https://spiffe.io/docs/latest/spiffe/concepts/). ([[Spiffe](https://spiffe.io/docs/latest/spiffe/concepts/)][17])
* AWS IAM Access Analyzer is strong inside AWS and uses automated reasoning, but it is not a universal cross-cloud, CI, SaaS and agent identity graph. Source: [[https://aws.amazon.com/iam/access-analyzer/features/](https://aws.amazon.com/iam/access-analyzer/features/)](https://aws.amazon.com/iam/access-analyzer/features/). ([[Amazon Web Services, Inc.](https://aws.amazon.com/iam/access-analyzer/features/)][18])
* BloodHound is the most dangerous incumbent. SpecterOps added GitHub attack-path analysis to BloodHound Enterprise in 2026 and has been expanding its graph beyond traditional Active Directory. That validates demand but compresses the opportunity window. Source: [[https://specterops.io/blog/2026/03/18/introducing-attack-path-analysis-for-github-in-bloodhound-enterprise/](https://specterops.io/blog/2026/03/18/introducing-attack-path-analysis-for-github-in-bloodhound-enterprise/)](https://specterops.io/blog/2026/03/18/introducing-attack-path-analysis-for-github-in-bloodhound-enterprise/). ([[SpecterOps](https://specterops.io/blog/2026/03/18/introducing-attack-path-analysis-for-github-in-bloodhound-enterprise/)][19])

The proposed gap is an openly governed, connector-neutral graph designed specifically for **non-human delegation, actual use, lifecycle state and AI-agent authority**, rather than a generic directory or CSPM.

### Why open source matters

The complete identity graph is one of an organization’s most sensitive datasets. Self-hosting and transparent inference rules are strong adoption requirements.

Every edge should retain provenance: which API response, configuration or policy generated the relationship. A user must be able to challenge a path rather than trust a black-box “critical risk” score.

### Proposed technical wedge

1. Canonical identity and delegation schema.
2. Edge-level provenance and confidence.
3. Temporal snapshots and “what changed?” queries.
4. Explicit ownership, sponsor and offboarding state.
5. Effective-authority computation across cloud, Kubernetes, CI and SaaS.
6. Reachability proofs showing every policy and trust transition.
7. Actual-use overlays from audit logs.
8. Least-privilege remediation as a generated, reviewable policy patch.
9. Agent execution budgets and human-delegation chains.

### Minimum impressive prototype

Connect GitHub, AWS and Kubernetes.

Demonstrate a path in which:

1. A GitHub Actions workflow can mint an OIDC token.
2. The token can assume an AWS role.
3. The role can reach a secret, KMS key or deployment target.
4. Repository permissions or third-party Action control make the path exploitable.
5. The tool emits the full evidence chain and a minimally disruptive remediation.

That demonstration is stronger than a large dashboard filled with generic findings.

### Credible six-month deliverable

* GitHub, AWS, Kubernetes and Vault connectors.
* Versioned open graph schema.
* Incremental graph ingestion and snapshots.
* Attack-path query engine.
* Ownership and stale-identity policies.
* Effective permission and last-used overlays.
* CLI, API and minimal graph explorer.
* Remediation PR generation.
* Export into BloodHound OpenGraph or another open interchange format where feasible.

### Competitors and substitutes

BloodHound Enterprise/Community Edition, cloud-native IAM analyzers, CNAPP platforms, NHI security vendors, SPIRE, manual IAM reviews and custom graph queries.

### Commercial model

Open-source schema, graph engine, core connectors and queries; paid managed ingestion, enterprise SaaS connectors, high-scale history, RBAC, alerting, remediation workflow and support.

### Biggest risks

* Connector maintenance can consume the entire company.
* Semantic mistakes in effective-permission computation create dangerous false assurance.
* Enterprise design partners may refuse access to realistic identity data.
* BloodHound can expand faster than a solo founder.
* Sales cycles are long and require security credibility.

### Smallest disproof experiment

Obtain restricted read-only access to ten design-partner environments or realistic exported configurations.

Stop if:

* The tool cannot find at least three meaningful paths missed by existing cloud/IAM tools.
* Most reported paths are false or operationally irrelevant.
* Fewer than two organizations agree to recurring scans.
* Connector normalization requires bespoke consulting for every deployment.

### Path toward 90+

It becomes a 90+ opportunity if three or four connectors repeatedly expose cross-system paths that incumbent cloud tools miss and at least two security teams treat the graph as an operational control rather than an occasional audit.

**Confidence:** medium. Market severity is strong; solo-founder connector economics are the central weakness.

---

## #3 — Postgres Migration Safety and Workload Replay Lab — 86/100

### Concept

A Postgres-first “flight simulator” for schema, index, configuration and database-version changes.

The system captures a privacy-preserving representation of production workload, applies a proposed change to an isolated database and then replays the workload while testing:

* Lock acquisition and lock queues.
* Old and new application compatibility.
* Query-plan changes.
* Tail-latency regressions.
* Cardinality and row-count effects.
* Data transformation correctness.
* Connection and transaction behavior.
* Rollback feasibility.
* Replication and failover implications.

### Exact users and buyers

**Users:** backend engineers, database engineers, SREs and platform teams.

**Buyers:** engineering leaders, database-platform teams, infrastructure companies and organizations operating material Postgres workloads.

### Painful job being solved

Database changes recur constantly and can fail despite syntactically correct migration files.

PostgreSQL’s own documentation states that many `ALTER TABLE` forms acquire `ACCESS EXCLUSIVE` locks unless otherwise noted. PostgreSQL’s locking documentation explains that this lock mode conflicts with all other table-level modes. Sources:

* [[https://www.postgresql.org/docs/current/sql-altertable.html](https://www.postgresql.org/docs/current/sql-altertable.html)](https://www.postgresql.org/docs/current/sql-altertable.html)
* [[https://www.postgresql.org/docs/current/explicit-locking.html](https://www.postgresql.org/docs/current/explicit-locking.html)](https://www.postgresql.org/docs/current/explicit-locking.html) ([[PostgreSQL](https://www.postgresql.org/docs/current/sql-altertable.html)][20])

Xata’s engineering analysis illustrates an especially dangerous case: an exclusive-lock request waiting in a queue can cause later otherwise-compatible work to queue behind it. That source is vendor-authored but describes standard Postgres lock behavior. Source: [[https://xata.io/blog/migrations-and-exclusive-locks](https://xata.io/blog/migrations-and-exclusive-locks)](https://xata.io/blog/migrations-and-exclusive-locks). ([[Xata](https://xata.io/blog/migrations-and-exclusive-locks)][21])

### Why existing solutions are insufficient

Tools such as Flyway, Liquibase, Atlas and Bytebase manage migration definitions and workflow. `pgroll` provides reversible, zero-downtime migrations while supporting multiple schema versions, which is a strong solution to part of the problem. Source: [[https://pgroll.com/](https://pgroll.com/)](https://pgroll.com/). ([[pgroll](https://pgroll.com/)][22])

The unresolved gap is **pre-deployment empirical evidence**:

* What happens under the organization’s actual concurrency pattern?
* Does the old application still work while the new schema is introduced?
* Does a query plan change under realistic data distribution?
* Can rollback actually restore behavior?
* Does a seemingly harmless DDL request form a production lock queue?

Most teams answer these questions with staging, manual load tests or custom scripts.

### Why open source matters

Database schemas, query traces and workload shapes are sensitive. A self-hosted engine is easier to adopt than mandatory upload to a third party.

Postgres extensions, ORMs, migration frameworks and deployment systems also require an open adapter ecosystem.

### Proposed technical wedge

1. Capture normalized query templates through `pg_stat_statements` and optional proxy sampling.
2. Preserve transaction grouping, concurrency and arrival distributions without retaining raw sensitive values.
3. Produce a representative subset or synthetic distribution-aware dataset.
4. Apply the migration or upgrade inside an isolated environment.
5. Replay old and new application revisions concurrently.
6. Measure lock waits, blocked statements, plans, latency distributions and result differences.
7. Attempt declared rollback and verify post-rollback invariants.
8. Emit a machine-readable and human-readable preflight evidence report.

### Minimum impressive prototype

Provide a Docker-based demonstration containing:

* A sizeable Postgres dataset.
* A realistic concurrent read/write workload.
* A migration that passes SQL linting.
* A lock-queue failure or severe plan regression.
* A report predicting the failure before deployment.
* A safer migration alternative whose replay passes.

That can become an immediately understandable public demo.

### Credible six-month deliverable

* Postgres-only scope.
* Capture through `pg_stat_statements`, logs and an optional lightweight proxy.
* Query normalization and parameter anonymization.
* Workload replay with controllable concurrency.
* Lock and deadlock analysis.
* Query-plan and latency comparison.
* Old/new application compatibility runner.
* Migration and rollback verification.
* GitHub Action and CLI.
* Integration with `pgroll`, Atlas or plain SQL rather than replacing them.
* Evidence reports suitable for CI gates.

### Competitors and substitutes

`pgroll`, Atlas, Bytebase, Flyway, Liquibase, pganalyze, database branching products, cloud blue/green systems, custom staging environments and manual load testing.

### Commercial model

The full local capture and replay engine remains open source. Paid offerings can provide managed isolated environments, large dataset cloning, retained baselines, cloud-database integrations, fleet policy and support.

### Biggest risks

* Captured workloads may not represent production peaks.
* Data distribution can matter more than query text.
* Anonymization can destroy important statistical properties.
* A passing simulation may create false confidence.
* Large replays can be expensive.
* Cloud-specific replication behavior can be difficult to reproduce locally.

### Smallest disproof experiment

Test 20 previously executed migration changes from at least five teams.

Stop or narrow if:

* The tool finds no meaningful issue beyond static migration linting in at least 15% of cases.
* Teams refuse workload capture even with local-only anonymization.
* Replay results do not correlate with known production behavior.
* The setup requires more engineering effort than creating a custom staging test.

### Path toward 90+

It reaches 90 if real teams repeatedly discover lock, plan or compatibility failures before deployment and the tool can be inserted into existing migration pipelines without requiring a database-platform replacement.

**Confidence:** high. It has the cleanest product boundary and easiest commercial explanation among the top three, but lower grant appeal than the security-heavy opportunities.

---

# 5. The remaining seven opportunities

## #4 — Agent Action Safety Kernel — 85/100

**Concept, users and job:** An open runtime around mutating agent tool calls that supplies semantic transaction envelopes: intent, authorization budget, idempotency key, preconditions, postconditions, verify-before-retry, delayed commit where possible, compensation and signed receipts. Users are agent-platform teams and SaaS companies exposing write-capable tools; buyers are engineering, security and automation leaders.

MCP’s July 28, 2026 specification moved toward a stateless core and strengthened authorization, but transport and authorization standards do not create universal transaction or compensation semantics for arbitrary external tools. Sources: [[https://modelcontextprotocol.io/specification/2026-07-28](https://modelcontextprotocol.io/specification/2026-07-28)](https://modelcontextprotocol.io/specification/2026-07-28) and [[https://blog.modelcontextprotocol.io/posts/2026-07-28/](https://blog.modelcontextprotocol.io/posts/2026-07-28/)](https://blog.modelcontextprotocol.io/posts/2026-07-28/). ([[Model Context Protocol Blog](https://blog.modelcontextprotocol.io/posts/2026-07-28/?utm_source=chatgpt.com)][23])

OWASP identifies tool misuse, identity abuse and unexpected code execution among major agentic risks. Recent preprints on verify-before-retry, Atomix and Cordon indicate active research into non-atomic failures and semantic transactions. These papers are preliminary evidence, not production-market proof. Sources:

* [[https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/](https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/)](https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/)
* [[https://arxiv.org/abs/2608.02645](https://arxiv.org/abs/2608.02645)](https://arxiv.org/abs/2608.02645)
* [[https://arxiv.org/abs/2602.14849](https://arxiv.org/abs/2602.14849)](https://arxiv.org/abs/2602.14849) ([[OWASP Gen AI Security Project](https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/?utm_source=chatgpt.com)][24])

**Gap, OSS and wedge:** Temporal and workflow engines provide durable orchestration, while MCP gateways provide transport and policy. Neither gives arbitrary tools magical exactly-once execution. The wedge is an open semantic contract: before retrying, verify external state; before executing, check authorization budget; after ambiguous failure, reconcile; for irreversible actions, require explicit commit rules.

**Prototype:** Wrap five simulated MCP tools—email, payment, ticket creation, deployment and deletion—then inject failures after dispatch but before response. Show that a vanilla retrying agent duplicates effects while the safety kernel reconciles state.

**Six months:** TypeScript and Python SDKs, durable journal, policy engine, fault injector, postcondition interface, compensation patterns, OpenTelemetry emission and ten reference adapters.

**Competitors:** Temporal, LangGraph, MCP gateways, human approval layers and emerging transactional-agent research.

**Biggest risk:** Many external systems cannot provide reliable postcondition queries or compensation. The product must promise bounded semantic safety, not universal exactly-once execution.

**Falsifier:** If fewer than half of high-risk actions in ten real agent workflows can express useful verification or compensation without bespoke integration work, stop.

**Commercial path:** Open runtime and contract; paid managed audit, enterprise policy, adapter certification and high-availability journal.

**90-point condition:** A common agent framework or major tool provider adopts the contract and real incident testing demonstrates a large reduction in duplicate or unauthorized actions.

---

## #5 — Deterministic Agent Flight Recorder, Replay and Fault Injection — 84/100

**Concept, users and job:** Record model responses, tool boundaries, state transitions, time, randomness and external dependencies so an agent failure can be deterministically replayed, minimized and fault-injected in CI. Users are agent developers, platform engineers and SREs.

OpenTelemetry now standardizes GenAI telemetry, and Phoenix provides open-source traces, evaluations and experiments. A Haystack feature request explicitly asks for first-class deterministic pipeline recording and replay because teams otherwise maintain brittle mocks. Sources:

* [[https://opentelemetry.io/blog/2026/genai-observability/](https://opentelemetry.io/blog/2026/genai-observability/)](https://opentelemetry.io/blog/2026/genai-observability/)
* [[https://arize.com/docs/phoenix](https://arize.com/docs/phoenix)](https://arize.com/docs/phoenix)
* [[https://github.com/deepset-ai/haystack/issues/11836](https://github.com/deepset-ai/haystack/issues/11836)](https://github.com/deepset-ai/haystack/issues/11836) ([[OpenTelemetry](https://opentelemetry.io/blog/2026/genai-observability/)][6])

**Gap, OSS and wedge:** Trace storage is not replay. The wedge is a portable boundary cassette, deterministic environment controls, secret-aware redaction, model/tool substitution and systematic injection of 429s, malformed JSON, partial streams, duplicated calls, delayed visibility and ambiguous timeouts.

**Prototype:** Record one MCP- or LangGraph-based workflow, replay it without external API calls, then substitute a new model or inject a timeout and show the precise state divergence.

**Six months:** Two framework adapters plus MCP, encrypted cassettes, replay CLI, failure minimizer, fault library, CI integration and trace-to-reproduction export.

**Competitors:** Phoenix, Langfuse, AgentOps, Datadog and numerous small emerging replay projects.

**Biggest risks:** Exact replay can be impossible when hidden provider state matters; framework churn is severe; the product can degenerate into another trace viewer.

**Falsifier:** If fewer than 70% of collected production-like failures can be reproduced sufficiently to debug them, or users consume only the trace UI, stop.

**Commercial path:** Open recorder and replay engine; paid secure storage, fleet management, private cassettes, collaboration and large-scale replay.

**90-point condition:** It becomes the standard reproduction artifact attached to agent bug reports across multiple frameworks.

---

## #6 — Post-Quantum Cryptography Migration Workbench — 83/100

**Concept, users and job:** A migration workbench—not merely a scanner—that inventories cryptography across source, binaries, dependencies, certificates, TLS, KMS and services; maps it to data-retention requirements; and tests hybrid or post-quantum replacements for compatibility and performance.

NIST has finalized initial PQC standards and is urging organizations to begin migration planning. The NCCoE migration project emphasizes discovery and inventory of quantum-vulnerable cryptography across hardware, software and services. Sources:

* [[https://csrc.nist.gov/projects/post-quantum-cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)](https://csrc.nist.gov/projects/post-quantum-cryptography)
* [[https://www.nist.gov/pqc](https://www.nist.gov/pqc)](https://www.nist.gov/pqc)
* [[https://www.nccoe.nist.gov/applied-cryptography/migration-to-pqc](https://www.nccoe.nist.gov/applied-cryptography/migration-to-pqc)](https://www.nccoe.nist.gov/applied-cryptography/migration-to-pqc) ([[NIST Computer Security Resource Center](https://csrc.nist.gov/projects/post-quantum-cryptography)][25])

**Users and buyers:** security architecture, cryptography, compliance and procurement teams in regulated industries, governments, critical infrastructure and vendors with long-lived data.

**Gap and OSS:** Scanner projects already exist. The missing layer is migration execution: purpose-aware inventory, dependency ownership, replacement experiments, protocol interoperability, performance baselines, drift monitoring and evidence. Vendor neutrality and transparent detection make OSS important.

**Prototype:** Scan a repository, container and TLS endpoint; emit a cryptographic bill of materials; identify a vulnerable algorithm path; then run a hybrid/PQC replacement experiment and report size, latency and compatibility effects.

**Six months:** source/container/TLS/certificate inventory, CBOM export, migration policy, OpenSSL-compatible experiments, CI drift detection and one cloud-KMS integration.

**Competitors:** cryptographic inventory scanners, consulting firms, vendor migration products and manual spreadsheets.

**Biggest risks:** Budget may remain delayed despite future deadlines; standards and implementation guidance continue evolving; enterprise sales are slow.

**Falsifier:** Interview ten regulated organizations. Postpone if none has an inventory owner, budget or migration milestone before 2028.

**Commercial path:** Open discovery and experiment engine; paid compliance packs, managed inventory, procurement integration and support.

**90-point condition:** Procurement or regulatory frameworks make machine-readable cryptographic inventory and migration evidence mandatory.

---

## #7 — Runtime Exposure and VEX Evidence Engine — 82/100

**Concept, users and job:** Correlate SBOM and vulnerability data with signed runtime and test evidence showing which packages, modules or functions were actually loaded or exercised. Produce an evidence-backed VEX recommendation and feed it into GUAC or existing vulnerability systems.

GUAC already aggregates supply-chain metadata into a graph, while CISA supports SBOM and VEX as machine-readable mechanisms. The opportunity is evidence production, not another graph. Sources:

* [[https://guac.sh/](https://guac.sh/)](https://guac.sh/)
* [[https://www.cisa.gov/topics/information-communications-technology-supply-chain-security/sbom](https://www.cisa.gov/topics/information-communications-technology-supply-chain-security/sbom)](https://www.cisa.gov/topics/information-communications-technology-supply-chain-security/sbom)
* [[https://www.cisa.gov/resources-tools/resources/minimum-requirements-vulnerability-exploitability-exchange-vex](https://www.cisa.gov/resources-tools/resources/minimum-requirements-vulnerability-exploitability-exchange-vex)](https://www.cisa.gov/resources-tools/resources/minimum-requirements-vulnerability-exploitability-exchange-vex) ([[guac](https://guac.sh/)][7])

**Users and buyers:** product-security teams, vulnerability-management teams, container-platform teams and software vendors.

**Gap and OSS:** Package presence is a weak proxy for exploitability, but declaring “not affected” without evidence is dangerous. The wedge is signed runtime evidence with explicit scope and confidence, produced through eBPF and language-runtime hooks.

**Prototype:** For a Node or Python service, map an SBOM to actually loaded modules during integration tests and a staged workload; emit an OpenVEX-compatible recommendation containing the evidence hash and coverage limitations.

**Six months:** Linux container collector, Node and Python runtime adapters, SBOM correlation, GUAC ingestion, CI mode, Kubernetes deployment and signed evidence.

**Competitors:** Trivy, Grype, GUAC, runtime-security vendors, reachability scanners and manual VEX production.

**Biggest risk:** False negatives can be catastrophic. “Not observed” must never automatically mean “not exploitable.”

**Falsifier:** Seed known vulnerable execution paths. Stop if coverage cannot bound false negatives tightly enough for users to act on the evidence.

**Commercial path:** Open collectors and evidence format; paid fleet management, enterprise correlation, retained evidence and support.

**90-point condition:** An OpenSSF, GUAC or major scanner ecosystem adopts the evidence format and security teams demonstrate large backlog reduction without unacceptable misses.

---

## #8 — AI Provider Conformance and Failure-Semantics Lab — 81/100

**Concept, users and job:** A neutral executable specification and chaos laboratory for model APIs covering streaming, cancellation, tool calls, malformed outputs, usage accounting, error classification, retries, fallback safety, capability claims and partial failures.

This is intentionally **not a gateway**.

LiteLLM, Envoy AI Gateway, Portkey and TensorZero provide abstraction and routing. Public LiteLLM issues demonstrate that failure semantics remain difficult and provider-specific. ([[LiteLLM](https://docs.litellm.ai/docs/routing)][5])

**Users and buyers:** gateway maintainers, agent-platform teams, provider SDK teams, model hosts and enterprises using multiple providers.

**Gap and OSS:** Every gateway privately rediscovers provider behavior. A neutral public corpus, fault proxy and versioned expected-behavior specification could become shared infrastructure. Openness is essential for vendor accountability.

**Prototype:** Run 30 scenarios against six providers or local servers: pre-stream error, post-first-token failure, malformed tool arguments, cancellation, delayed usage, duplicate chunks, ambiguous timeout and retry-after handling. Publish reproducible results.

**Six months:** 100 scenarios, eight provider families, local-server adapters, fault proxy, CI integration, versioned results and an optional minimal Rust reference data plane.

**Competitors:** existing gateways, provider SDK test suites and internal platform conformance systems.

**Biggest risks:** Providers may change faster than tests can be maintained; vendors may dislike public scoring; incumbents can absorb the suite; API costs accumulate.

**Falsifier:** Stop if five gateway or provider maintainers decline integration and the first 30 days reveal no important divergence beyond documented differences.

**Commercial path:** Open specification and runner; paid continuous private-provider matrix, enterprise endpoints, historical drift, alerts and support.

**90-point condition:** The suite becomes required CI for at least two major gateways and providers use it before API releases.

---

## #9 — AI Artifact Provenance and License-Evidence Compiler — 80/100

**Concept, users and job:** Resolve dataset → base model → fine-tune → adapter → quantization → packaged application lineage, verify hashes and signatures, preserve notices and produce machine-readable provenance and obligation evidence.

SPDX 3 includes AI and dataset profiles. OpenSSF’s Model Signing specification addresses model origin and integrity. Sources:

* [[https://spdx.github.io/spdx-spec/v3.0.1/conformance/](https://spdx.github.io/spdx-spec/v3.0.1/conformance/)](https://spdx.github.io/spdx-spec/v3.0.1/conformance/)
* [[https://spdx.dev/learn/areas-of-interest/ai/](https://spdx.dev/learn/areas-of-interest/ai/)](https://spdx.dev/learn/areas-of-interest/ai/)
* [[https://openssf.org/blog/2025/06/25/an-introduction-to-the-openssf-model-signing-oms-specification/](https://openssf.org/blog/2025/06/25/an-introduction-to-the-openssf-model-signing-oms-specification/)](https://openssf.org/blog/2025/06/25/an-introduction-to-the-openssf-model-signing-oms-specification/) ([[SPDX](https://spdx.github.io/spdx-spec/v3.0.1/conformance/)][26])

A 2026 preprint analysing large numbers of dataset-to-model-to-application chains reported widespread missing license and attribution information. That is preliminary research and should not be treated as a legal conclusion. Source: [[https://arxiv.org/abs/2602.08816](https://arxiv.org/abs/2602.08816)](https://arxiv.org/abs/2602.08816). ([[arXiv](https://arxiv.org/abs/2602.08816)][27])

**Users and buyers:** model publishers, AI-platform teams, open-source program offices, legal teams and enterprises distributing AI products.

**Gap and OSS:** Model cards and registry metadata are inconsistent. The wedge is a content-addressed lineage resolver plus evidence compiler that exports SPDX AI/Dataset data and model-signing attestations. Open standards and transparent resolution rules are mandatory.

**Prototype:** Resolve one model family with several fine-tunes, adapters and quantizations across Git, Hugging Face and OCI. Identify broken lineage, missing notices and unsigned artifacts.

**Six months:** Git/Hugging Face/OCI connectors, lineage graph, SPDX export, OpenSSF Model Signing verification, policy rules, notice bundle and CI integration.

**Competitors:** model cards, registry metadata, generic SBOM/licence scanners and manual legal review.

**Biggest risks:** Source metadata is incomplete; licence interpretation is jurisdiction-specific; the product must not impersonate legal counsel; willingness to pay is uncertain.

**Falsifier:** Stop if public metadata cannot reconstruct useful lineage for at least 70% of a representative model sample or if OSPO/legal teams will not use the output operationally.

**Commercial path:** Open compiler and formats; paid enterprise policy, private registries, approval workflow and audit support.

**90-point condition:** Model registries, procurement systems or regulation begin requiring verifiable AI-artifact lineage and obligations.

---

## #10 — Local-First Sync Correctness and Schema-Evolution Lab — 77/100

**Concept, users and job:** A deterministic simulator and counterexample minimizer for local-first and CRDT systems under partitions, duplicate or reordered messages, long-offline clients, schema changes, authorization revocation, clock skew and key rotation.

Ink & Switch’s local-first work explains the model’s privacy, ownership and offline advantages. Academic work identifies schema evolution as an unresolved challenge, while existing sync engines document assumptions particular to their own protocols. Sources:

* [[https://www.inkandswitch.com/essay/local-first/](https://www.inkandswitch.com/essay/local-first/)](https://www.inkandswitch.com/essay/local-first/)
* [[https://arxiv.org/abs/2309.11406](https://arxiv.org/abs/2309.11406)](https://arxiv.org/abs/2309.11406)
* [[https://docs.rs/automerge/latest/automerge/sync/index.html](https://docs.rs/automerge/latest/automerge/sync/index.html)](https://docs.rs/automerge/latest/automerge/sync/index.html)
* [[https://github.com/electric-sql/electric](https://github.com/electric-sql/electric)](https://github.com/electric-sql/electric) ([[Ink & Switch](https://www.inkandswitch.com/essay/local-first/)][28])

**Users and buyers:** local-first framework maintainers, collaborative-application teams and privacy-oriented product companies.

**Gap and OSS:** Frameworks have their own tests, but there is no widely adopted cross-engine scenario language for offline duration, schema migration, authorization change and encryption state. Transparent counterexamples make OSS essential.

**Prototype:** Run Automerge or another CRDT through generated network schedules, introduce a schema transition and an authorization revocation, then shrink a divergence into a minimal reproducible trace.

**Six months:** deterministic scheduler, scenario DSL, state visualizer, counterexample minimizer, schema/auth/key-rotation scenarios and two engine adapters.

**Competitors:** framework-specific test suites, bespoke Jepsen-style testing and manual chaos tests.

**Biggest risks:** The paying market may be too small; semantics differ across frameworks; maintainers may prefer internal tests.

**Falsifier:** Stop if five framework maintainers provide neither real failure cases nor willingness to run the tool in CI.

**Commercial path:** Open simulator; paid distributed fuzzing infrastructure and enterprise support. This is the weakest business model in the top 10.

**90-point condition:** Local-first architecture becomes a major enterprise application pattern and multiple frameworks adopt a common correctness test format.

---

# 6. Direct comparison: winner versus Hafiz Rust Gateway

| Dimension                   | Proof-Carrying Change Verification                                                | Hafiz Rust Gateway                                                              | Verdict                                                   |
| --------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | --------------------------------------------------------- |
| **Recurring pain**          | Every risky or AI-authored code change requires merge confidence                  | Only organizations using multiple model providers need the full gateway problem | Winner has broader recurrence                             |
| **Initial user**            | Maintainer or team can add a GitHub Action                                        | Platform team must insert a proxy into a production hot path                    | Winner has much lower adoption friction                   |
| **Incumbent pressure**      | Review category is crowded, but executable evidence remains fragmented            | Gateway category is crowded and incumbents are rapidly improving                | Both crowded; Hafiz is worse                              |
| **Technical wedge**         | Change evidence graph, mutation/differential testing, signed verification bundles | Correct failure state machine, conformance corpus, routing receipts             | Winner has a larger expandable evidence layer             |
| **Open-source reason**      | Source privacy, reproducibility and language adapters                             | Self-hosting, trust and provider adapters                                       | Strong for both                                           |
| **Early public proof**      | Benchmark showing defects missed by CI and review                                 | Provider failure matrix and latency/failure benchmark                           | Both demonstrable; winner’s value is easier to understand |
| **Replacement requirement** | Additive to existing CI and reviewers                                             | Frequently replaces or intercepts an existing gateway path                      | Winner is materially easier to deploy                     |
| **Defensibility**           | Historical change/evidence/counterexample corpus plus plugin ecosystem            | Provider corpus; proxy implementation itself is copyable                        | Winner                                                    |
| **Grant appeal**            | AI safety, software supply-chain security and OSS assurance                       | Moderate unless narrowed to conformance/security                                | Winner                                                    |
| **Commercial path**         | Hosted runners, policies, evidence retention, enterprise fleet                    | Managed gateway, support, analytics                                             | Both credible                                             |
| **Primary technical risk**  | Test oracle, compute and false positives                                          | Provider churn, streaming semantics, incumbent absorption                       | Different; neither trivial                                |
| **Score**                   | **88/100**                                                                        | **78/100**                                                                      | **Winner by 10 points**                                   |

The central strategic distinction is this:

**Hafiz asks a company to replace infrastructure. The winner asks it to add evidence.**

Additive tools generally have an easier first adoption path than inline infrastructure, especially for an unknown open-source project.

---

# 7. One recommended project to pursue

## Build the Proof-Carrying Change Verification Engine

The project should begin with this positioning:

> Open-source, self-hosted verification infrastructure that turns software changes—especially AI-authored changes—into reproducible, signed behavioral evidence.

The project should **not** begin as:

* An autonomous reviewer.
* A chatbot.
* A static-analysis replacement.
* A general CI platform.
* A formal-verification system.
* A giant multi-language framework.
* A dashboard-heavy enterprise product.

Its first battle is narrow:

> Can changed-code mutation and differential verification expose important defects that normal CI and AI review miss, and can it express the result as a reproducible signed artifact?

For founder-specific execution, this is superior to the NHI graph because it can be developed and validated against public repositories without enterprise credentials. It is superior to the Postgres lab in upside and grant appeal, although the Postgres project has a cleaner technical boundary and may have a higher probability of becoming a useful paid product.

Assuming existing Hafiz code is still early, preserve only reusable components such as fault injection, deterministic recording, state-machine testing and receipt generation. Do not let sunk cost dictate the primary project.

---

# 8. Practical first 30-day validation plan

## Days 1–3: Freeze the claim

Choose **TypeScript only** for the first prototype.

Define a compact evidence schema:

* Repository and revision.
* Diff and affected symbols.
* Environment hashes.
* Commands executed.
* Tests selected.
* Mutations generated and killed.
* Differential inputs and outputs.
* Counterexamples.
* Tool versions.
* Signature.

Explicitly state that the artifact proves execution of declared checks, not full correctness.

Create a short RFC and architecture document. Do not build a web UI.

## Days 4–7: Build the benchmark before the product

Construct an initial corpus of 30 changes:

* Ten historical JavaScript/TypeScript bugs reconstructed from public fixes.
* Ten intentionally seeded defects.
* Ten AI-generated changes created from realistic repository tasks.

Include authentication, validation, serialization, concurrency, caching, API compatibility and error-handling cases.

For every case, record whether ordinary tests detect the fault.

The benchmark is not marketing decoration. It is the product’s truth mechanism.

## Days 8–14: Build CLI version 0

Implement:

1. Git diff and changed-symbol extraction.
2. Mapping from changed symbols to test files.
3. Hermetic container execution.
4. Existing targeted tests.
5. Changed-code mutation testing through an existing engine such as Stryker.
6. Evidence JSON.
7. Reproducible seeds and command hashes.

Do not use an LLM in the critical decision path yet.

## Days 15–20: Add differential verification

Run old and new revisions against identical generated or recorded inputs.

Start with one surface:

* HTTP handlers, or
* exported functions, or
* CLI commands.

Generate a clear declaration of expected versus unexpected differences.

Add counterexample minimization where practical.

## Days 21–24: Add signed evidence and GitHub integration

Use an in-toto-compatible test-result attestation and sign it with Sigstore/Cosign. GitHub already supports artifact attestations and offline verification, providing a usable substrate. Sources:

* [[https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations)](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations)
* [[https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations/verifying-attestations-offline](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations/verifying-attestations-offline)](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations/verifying-attestations-offline) ([[GitHub Docs](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations)][29])

Create one GitHub Check containing:

* Risk summary.
* Evidence status.
* Killed/surviving mutations.
* Behavior differences.
* Reproduction command.
* Verifiable attestation.

## Days 25–27: Run five external repository trials

Recruit five maintainers or small engineering teams.

Do not ask, “Do you like the idea?” Ask them to install it on an actual branch and observe:

* Setup time.
* Added runtime.
* Meaningful findings.
* False blocks.
* Whether they rerun it voluntarily.
* Whether they would make it required.

## Days 28–30: Publish the result and make a kill decision

Publish:

* Benchmark repository.
* Exact methodology.
* Baseline results.
* False-positive and false-negative analysis.
* Compute overhead.
* Five complete evidence bundles.
* Limitations.

### Go/no-go thresholds

Continue only if most of these hold:

* At least **20% additional meaningful defect detection** over existing tests.
* At least one finding missed by both ordinary CI and a leading AI reviewer.
* False-block rate below **5%**.
* Typical runtime below **1.5× existing CI**, or a clear budgeted mode that maintains value.
* Three of five trial users keep the integration.
* At least two users say they would make it a required check.
* Most findings are supported by executable counterexamples rather than LLM judgement.

If defect detection is valuable only for dependency upgrades, narrow to dependency-update verification. If it is valuable mainly for database changes, pivot to the Postgres Migration Safety Lab. If it produces mostly style advice, kill it.

---

# 9. Six-month open-source roadmap

## Month 1 — Evidence before architecture

* TypeScript CLI.
* Initial benchmark.
* Changed-code mutation testing.
* Differential verification.
* Signed evidence artifact.
* Five design partners.
* Public go/no-go report.

**Exit criterion:** demonstrated additional defect detection with acceptable noise and cost.

## Month 2 — Stable core and GitHub workflow

* Formal evidence schema version 0.1.
* GitHub Action and Checks integration.
* Hermetic runner.
* Cache keyed by source, environment and check configuration.
* Risk-based check planner.
* Security and privacy model.
* `SECURITY.md`, contribution guide and governance rules.

**Exit criterion:** ten repositories running it repeatedly.

## Month 3 — Python and property testing

* Python adapter.
* `pytest` and property-testing integration.
* Fast-check/Hypothesis adapters.
* LLM-assisted property proposals, clearly separated from executable validation.
* Counterexample minimization.
* Policy profiles for ordinary and security-sensitive changes.

**Exit criterion:** benchmark demonstrates value in two ecosystems without doubling false positives.

## Month 4 — Evidence quality

* Changed-path coverage model.
* Mutation-quality metrics.
* API/CLI/HTTP differential adapters.
* Performance budget checks.
* Network egress controls.
* in-toto/Sigstore verification command.
* Reproducible OCI-packaged runner.

**Exit criterion:** another system can independently verify the evidence bundle.

## Month 5 — Plugin and enterprise foundations

* Plugin SDK.
* Semgrep and CodeQL result ingestion.
* GitLab CI support if demanded.
* Self-hosted worker coordinator.
* Organization policies stored as code.
* Evidence retention format and export.
* Role-based policy hooks without putting core verification behind a paywall.

**Exit criterion:** external contributors can add a check engine without modifying the core.

## Month 6 — Public release and credible benchmark

* Version 0.1 stable release.
* At least 300 labelled benchmark changes.
* Transparent comparison against baseline CI and AI review.
* Twenty active repositories.
* Three independent external contributors.
* Two public case studies.
* Grant applications backed by results rather than a concept document.
* Hosted beta only after the open-source runner is credible.

### Six-month scope that must remain out

* Full multi-language support.
* Replacing GitHub Actions or CI.
* Formal verification of arbitrary programs.
* A proprietary LLM.
* Automated merge approval without explicit policy.
* A massive enterprise dashboard.
* Compliance certifications.
* Self-hosted Kubernetes control-plane complexity before demand exists.

---

# 10. Explicit reasons this recommendation could still be wrong

1. **The oracle problem may be fatal.** Mutation and differential testing reveal discrepancies, but determining intended behavior remains difficult. Generated tests can reinforce the same mistaken assumption as generated code.

2. **Code-review incumbents can copy the visible features.** CodeRabbit, Qodo, GitHub, Semgrep or Sonar could add mutation, targeted test generation and signed evidence. The project needs a neutral format, benchmark and plugin ecosystem before that happens.

3. **CI cost may overwhelm value.** Developers tolerate review comments more easily than a check adding ten or twenty minutes to every pull request.

4. **The term “proof-carrying” may create backlash.** Security engineers may correctly reject any suggestion that passing finite tests proves correctness. Marketing must remain technically disciplined.

5. **Developers may not want another blocking gate.** An accurate tool can still fail commercially if it slows merges or requires extensive policy configuration.

6. **Language and build-system fragmentation may destroy solo-founder economics.** Supporting TypeScript well is feasible; supporting every language, monorepo and build system is not.

7. **AI models may internalize stronger self-testing.** Coding agents could eventually generate, run and interpret the same checks automatically. The defence is that independent evidence generation and attestation remain valuable even when an agent proposes the tests.

8. **The CodeRabbit funding signal may favour integrated review, not evidence infrastructure.** Buyers may prefer one broad vendor rather than a separate verification engine.

9. **The NHI opportunity may actually be larger.** I have discounted it because a solo founder lacks enterprise identity data and connector capacity. A strong security design partner could reverse the ranking.

10. **The Postgres opportunity may be the better company.** It has a clearer user, a narrower system boundary and more obvious willingness to pay. It lost only because it has less strategic upside across the entire AI-assisted development market.

11. **Public benchmark performance may not transfer to proprietary repositories.** Open-source projects often have different tests, architecture and operational constraints from enterprise systems.

12. **Signed bad evidence is still bad evidence.** Attestation establishes provenance and integrity; it does not make weak tests strong. The system must expose evidence quality, coverage and limitations rather than treating a signature as correctness.

---

# Final decision

**Primary project:** Proof-Carrying Change Verification Engine.

**Hafiz decision:** stop broad gateway expansion. Extract its failure-state-machine, fault-injection and receipt concepts into a small public conformance project. Continue that effort only if outside maintainers adopt it quickly.

**Capital allocation:** spend the next 30 days proving that executable change evidence catches defects ordinary CI and AI review miss. Do not spend those 30 days polishing branding, dashboards, routing features or enterprise infrastructure.

The winner is not yet a 9/10 opportunity. It is the strongest **88/100 hypothesis** currently supported by market demand, open-source logic, public demonstrability and solo-founder execution constraints.

[1]: https://dora.dev/dora-report-2025/ "https://dora.dev/dora-report-2025/"
[2]: https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/ "https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/"
[3]: https://arxiv.org/html/2512.03262 "https://arxiv.org/html/2512.03262"
[4]: https://www.reuters.com/technology/ai-code-review-platform-coderabbit-valued-15-billion-latest-funding-round-2026-08-12/ "https://www.reuters.com/technology/ai-code-review-platform-coderabbit-valued-15-billion-latest-funding-round-2026-08-12/"
[5]: https://docs.litellm.ai/docs/routing "https://docs.litellm.ai/docs/routing"
[6]: https://opentelemetry.io/blog/2026/genai-observability/ "https://opentelemetry.io/blog/2026/genai-observability/"
[7]: https://guac.sh/ "https://guac.sh/"
[8]: https://github.com/berriai/litellm/issues/35303 "https://github.com/berriai/litellm/issues/35303"
[9]: https://arxiv.org/html/2608.02645v1?utm_source=chatgpt.com "Verified Tool Calls Improve LLM Agent Reliability Under ..."
[10]: https://docs.litellm.ai/blog/litellm-rust-launch "https://docs.litellm.ai/blog/litellm-rust-launch"
[11]: https://openai.com/index/trusted-access-for-cyber/ "https://openai.com/index/trusted-access-for-cyber/"
[12]: https://slsa.dev/spec/draft/build-provenance "https://slsa.dev/spec/draft/build-provenance"
[13]: https://docs.coderabbit.ai/ "https://docs.coderabbit.ai/"
[14]: https://owasp.org/www-project-non-human-identities-top-10/2025/top-10-2025/ "https://owasp.org/www-project-non-human-identities-top-10/2025/top-10-2025/"
[15]: https://cloudsecurityalliance.org/press-releases/2026/03/24/more-than-two-thirds-of-organizations-cannot-clearly-distinguish-ai-agent-from-human-actions "https://cloudsecurityalliance.org/press-releases/2026/03/24/more-than-two-thirds-of-organizations-cannot-clearly-distinguish-ai-agent-from-human-actions"
[16]: https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised "https://www.stepsecurity.io/blog/harden-runner-detection-tj-actions-changed-files-action-is-compromised"
[17]: https://spiffe.io/docs/latest/spiffe/concepts/ "https://spiffe.io/docs/latest/spiffe/concepts/"
[18]: https://aws.amazon.com/iam/access-analyzer/features/ "https://aws.amazon.com/iam/access-analyzer/features/"
[19]: https://specterops.io/blog/2026/03/18/introducing-attack-path-analysis-for-github-in-bloodhound-enterprise/ "https://specterops.io/blog/2026/03/18/introducing-attack-path-analysis-for-github-in-bloodhound-enterprise/"
[20]: https://www.postgresql.org/docs/current/sql-altertable.html "https://www.postgresql.org/docs/current/sql-altertable.html"
[21]: https://xata.io/blog/migrations-and-exclusive-locks "https://xata.io/blog/migrations-and-exclusive-locks"
[22]: https://pgroll.com/ "https://pgroll.com/"
[23]: https://blog.modelcontextprotocol.io/posts/2026-07-28/?utm_source=chatgpt.com "The 2026-07-28 Specification"
[24]: https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/?utm_source=chatgpt.com "OWASP Top 10 for Agentic Applications for 2026"
[25]: https://csrc.nist.gov/projects/post-quantum-cryptography "https://csrc.nist.gov/projects/post-quantum-cryptography"
[26]: https://spdx.github.io/spdx-spec/v3.0.1/conformance/ "https://spdx.github.io/spdx-spec/v3.0.1/conformance/"
[27]: https://arxiv.org/abs/2602.08816 "https://arxiv.org/abs/2602.08816"
[28]: https://www.inkandswitch.com/essay/local-first/ "https://www.inkandswitch.com/essay/local-first/"
[29]: https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations "https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations"
