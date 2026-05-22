---
name: internal-read-gh-issue-preflight-qa
description: >-
  Read-only question bank for @gh-issues pre-flight. Helps gather goal, examples, counterexamples, and
  verification details before dedupe and mutation.
---

# Issue pre-flight Q&A

Use this before any issue create/edit mutation.

## Gather (deducible first)

- Candidate title intent from user prompt and repository context.
- Existing related issue numbers/titles from list/search.
- Existing labels that might apply.
- Whether the user is carrying **multiple independent themes** (long multi-section paste, many unrelated bullets, or a plan hub: **`.cursor/plan.md`** / linked **`.cursor/plans/<slug>.md`**) — see **[`internal-read-plan-parts`](../../plan/parts/SKILL.md)** and **`@gh-issues`** [Multi-intent clustering](../../../gh/issues/SKILL.md#multi-intent-clustering-optional).

## Clarify (ask when ambiguous)

- Goal: what outcome should be true after completion?
- Examples: concrete "today vs desired" cases.
- Counterexamples: explicitly out-of-scope or invalid behavior.
- Constraints: compatibility, regressions to avoid, rollout risks.
- Verification: acceptance and regression checks.

## Iteration / ship gate (normative for `@gh-issues`)

After each refinement pass, once requirements and draft intent are summarized, **before** list/dedupe, the caller must offer a single structured choice (AskQuestion per [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md), safe-first when ordering matters):

- **Abort** — stop or park; do not run inventory/dedupe for publish.
- **Sharpen** — another Q&A round (optionally Cursor’s built-in Plan or chat); then re-summarize and ask again.
- **Ship** — candidate is good enough to continue the pipeline (list → dedupe → description → mutation confirm).

Repeat until **Ship** or **Abort**. Readiness hints may cite [`internal-read-plan-confidence`](../../plan/confidence/SKILL.md); keep them optional, not mandatory scoring.

## Multi-topic and plan hubs

When a **plan hub** or **long multi-topic** paste is present, optionally use structured AskQuestion (same § as the ship gate) to confirm **one combined issue** vs **multiple themes** to ship after **Ship** (so **Sharpen** can split sections in **`.cursor/plan.md`** or chat before dedupe). Default remains one narrative until the user or router opts into the multi-intent path in **`@gh-issues`**.

## AskQuestion shape

- Present a concise summary in the prompt text.
- Keep options safe-first (Abort first).
- Use one refinement outlet only.

## See also

- [`internal-read-gh-issue-spec`](../issue-spec/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
- [`internal-read-plan-parts`](../../plan/parts/SKILL.md)
- [`@gh-issues`](../../../gh/issues/SKILL.md)
