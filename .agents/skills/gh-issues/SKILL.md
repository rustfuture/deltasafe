---
name: gh-issues
description: >-
  GitHub issues (`gh issue` create/edit router). Single shipping entrypoint for issue create-or-edit. Gather requirements with an explicit Sharpen/Abort/Ship gate,
  dedupe against existing issues, ask user whether to update/create/abort, then execute through internal-write-gh-issue-commands.
  Optional multi-intent path: partition themes (including from `.cursor/plan.md` / `internal-read-plan-parts`), dedupe each theme, then one Proceed batch of ordered creates/edits.
---

**Orchestration:** [`internal-read-gh-issue-dedupe`](../../read/gh/issue-dedupe/SKILL.md) (`@gh-issues orchestration order` section).

# GitHub: issues (router)

Normative fences / full matrix: [`internal-read-gh-issue-list`](../../../read/gh/issue-list/SKILL.md), [`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md).


**Public.** Top-level router for issue shipping. This skill owns create-vs-edit routing; there is no separate public create/edit path.

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-issue-list` when you need a fresh inventory snapshot first.
2. Use `@gh-issue-view` when create-or-edit should be anchored to a specific issue context.
3. Preview only: list/dedupe hubs **`internal-read-gh-issue-list`** and **`internal-read-gh-issue-dedupe`** (Execution batch runs them in router order).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file). **Do not** run dedupe list/search until the user has chosen **Ship** on the pre-dedupe gate (or explicitly overrides).

1. `internal-read-gh-issue-spec`
2. `internal-read-gh-issue-preflight-qa` with **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** — iterate **Sharpen / Abort / Ship** until **Ship** or **Abort**
3. **Dedupe and description path (branch on intent count after clustering)** — see **`internal-read-gh-issue-dedupe`** for single- vs multi-candidate orchestration and **[`internal-read-plan-parts`](../../read/plan/parts/SKILL.md)** when themes come from a plan hub:
   - **Single-theme (default):** `internal-read-gh-issue-dedupe` → `internal-read-gh-issue-description` → `internal-read-gh-issue-labels` → `internal-read-gh-issue-projects-relationships` for **one** candidate.
   - **Multi-intent (optional):** follow **[Multi-intent clustering](#multi-intent-clustering-optional)**; then run `internal-read-gh-issue-description`, `internal-read-gh-issue-labels`, and `internal-read-gh-issue-projects-relationships` **per theme** that is not deferred, before one mutation summary.

## Multi-intent clustering (optional)

Enter only when useful—avoid extra steps for a clearly single-topic request.

**When to use:** after **Ship**, if **any** of: the user asks to split into multiple issues; the pasted request has **clearly disjoint** workstreams (unrelated bullets/sections); or a plan hub exists (`.cursor/plan.md` and/or linked `.cursor/plans/<slug>.md`) with **≥2** disjoint parts per **`internal-read-plan-parts`**.

**Steps:**

1. **Partition / cluster** — Prefer plan section boundaries when files exist; otherwise group bullets by shared root cause. Merge themes that are the same underlying work (issue #41: two bullets, one root cause → **one** issue).
2. **Inventory once** — `internal-read-gh-issue-list` (and checklist in **`internal-read-gh-issue-dedupe`**) over open issues; reuse that inventory for every theme. Optionally add targeted `gh search issues` per theme only when the open list is too large to scan fairly.
3. **Per-theme dedupe** — For each theme, working title + one-line intent vs inventory → `safe_to_create` | `likely_duplicate #n` | `ambiguous` (**never** skip dedupe for a theme that will **create**).
4. **Summarize** — Table: Theme id → overlap verdict → proposed action (**edit #n** / **create** / **defer**). Safe default: offer to **merge** near-duplicate themes before publishing.
5. **Structured Q&A** — **`internal-write-plan-structured-qa`**: **Abort**, **Defer selected themes** (park for a later `@gh-issues`), or **Proceed — batch** with the full ordered list of `gh issue create` / `gh issue edit` summarized above the prompt (§1d multi-issue batch).
6. **Prepare artifacts** — Draft bodies under **`.cursor/gh/issues/`** with distinct names (for example `auth-oauth.md`, `docs-readme.md`); labels and project attach per theme, all folded into the **same** final mutation summary.

**Non-regression:** If clustering yields **one** theme, follow the single-theme branch only.

## Fixture (verification)

**Input (chat):** six bullets—(1–2) fix login timeout and refresh-token retry (same auth flow), (3) add CSV export for the dashboard, (4–5) tweak README install path and cross-link docs hub (same docs theme), (6) unrelated: migrate CI from Travis to GitHub Actions.

**Expected clustering:** Theme **A** auth: bullets 1–2 → **one** issue. Theme **B** export: bullet 3 → **one** issue. Theme **C** docs: bullets 4–5 → **one** issue. Theme **D** CI: bullet 6 → **one** issue (four issues), **unless** inventory shows an open issue already covering CI—then **edit #n** for that row only. If the user chooses to merge docs + README into one existing **#42**, table updates to **edit #42** for theme C and fewer creates.

**Dedupe behavior:** Each theme row gets a verdict against the **same** list; no theme ships **create** without a prior dedupe pass for that row.

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Route approved implementation work to `@git-start` once issue scope is settled.
2. Use `@gh-pr` when issue outcomes are ready to be tied to a shipping PR.

## Q&A bypass ENV

- `SKIP_QA_GH_ISSUES=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Gather pre-flight requirements first (goal, concrete examples, counterexamples, scope boundaries, verification intent) using **[`internal-read-gh-issue-spec`](../../read/gh/issue-spec/SKILL.md)** and **[`internal-read-gh-issue-preflight-qa`](../../read/gh/issue-preflight-qa/SKILL.md)**.
- When multiple themes are in play, partition using **[`internal-read-plan-parts`](../../read/plan/parts/SKILL.md)** and the **[Multi-intent clustering](#multi-intent-clustering-optional)** steps before dedupe-heavy work.
- Run issue inventory/search via **`internal-read-gh-issue-list`** shapes and compare candidate intent using **[`internal-read-gh-issue-dedupe`](../../read/gh/issue-dedupe/SKILL.md)**.
- Present overlap choices with structured Q&A (edit existing issue, create new issue, or abort) using **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)**.
- Build title/body through **[`internal-read-gh-issue-description`](../../read/gh/issue-description/SKILL.md)** and labels through **[`internal-read-gh-issue-labels`](../../read/gh/issue-labels/SKILL.md)**.
- Treat labels as a default part of the mutation plan: summarize accepted label adds/removals in the final AskQuestion mutation summary (skip only when the user explicitly declines labels).
- Plan **project attach** for the **issue target repo** using **[`internal-read-gh-issue-projects-relationships`](../../read/gh/issue-projects-relationships/SKILL.md)**: default to adding the issue to an owner-scoped project when discovery yields one clear title; use structured AskQuestion when several boards match; on scope errors or missing projects, state a **visible skip** (never silent success).
- Apply **[`internal-write-gh-issue-commands`](../../write/gh/issue-commands/SKILL.md)** after Goal + AskQuestion + Proceed: **one structured Proceed batch** may run **several** `gh issue create` / `gh issue edit` lines in a fixed order (multi-intent path) or a **single** create/edit (default). **Additive** **`gh project item-add`** stays in the **same** Proceed batch when that library’s fallback fences apply. If the user declines batching, defer extra themes to a follow-up **`@gh-issues`** invocation.
- After each pre-flight summary, run the **Sharpen / Abort / Ship** gate (structured AskQuestion per **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** and **[`internal-read-gh-issue-preflight-qa`](../../read/gh/issue-preflight-qa/SKILL.md)**): **Sharpen** (another Q&A round, optionally Cursor’s built-in Plan or chat), **Abort** (stop or park), **Ship** (continue toward list/dedupe and create/edit). Repeat until **Ship** or **Abort**. Optionally surface readiness using **[`internal-read-plan-confidence`](../../read/plan/confidence/SKILL.md)**; do not require numeric scoring.
- Route open-ended exploration to **Cursor’s built-in Plan** (product UI) or chat; route draft tightening through the same **Sharpen** loop when **`.cursor/plan.md`**, linked **`.cursor/plans/<slug>.md`**, or another artifact needs refinement. Use **`internal-read-plan-parts`** to align plan sections with themes before the multi-intent path. Route implementation delivery to **`@git-start`** / **`@gh-pr`** after issue scope is settled (see [Planning and issues](../../../README.md#planning-and-issues) in the root README).

## Do not

- Do not skip pre-flight requirement gathering before dedupe and mutation.
- Do not run direct `gh issue create/edit` instructions in this public skill.
- Do not open a new issue before dedupe for that theme unless the user explicitly overrides after seeing overlap evidence.

## On invoke

1. Run **[`internal-read-gh-issue-spec`](../../read/gh/issue-spec/SKILL.md)** and **[`internal-read-gh-issue-preflight-qa`](../../read/gh/issue-preflight-qa/SKILL.md)** gathering; after each refinement pass, present **Sharpen / Abort / Ship** (safe-first order when the UI respects option ordering). On **Sharpen**, continue clarifying (optional Cursor Plan or chat). On **Abort**, stop without list/dedupe. On **Ship**, continue with a stable candidate summary (even when the user already pasted a full spec—**Ship** still allows an optional **Sharpen** pass for labels or edge cases).
2. Decide **single-theme vs multi-intent** per **[Multi-intent clustering](#multi-intent-clustering-optional)**. If multi-intent, partition/cluster, then run **one** inventory + per-theme **[`internal-read-gh-issue-dedupe`](../../read/gh/issue-dedupe/SKILL.md)** verdicts and present the summary table + structured choices (**Proceed — batch**, **Abort**, defer themes as offered). If single-theme, run inventory + dedupe for the single candidate.
3. Ask the user to confirm actions: **Edit existing #n**, **Create new issue**, **multi-row table + Proceed — batch**, or **Abort** (and per-theme **defer** when using the multi-intent table).
4. Prepare description/labels (per theme in the multi case), include label intent and **project attach** (title, **`item-add`**, or explicit skip + reason) in the mutation summary, and run final mutation confirmation per **`internal-write-plan-structured-qa`** §1d (including multi-issue batch when applicable).
5. Execute the agreed command(s) via **`internal-write-gh-issue-commands`** in the summarized order, then any **same-batch** project fallback commands documented there when Proceed included them.

## See also

- [`@gh-issue-review`](review/SKILL.md) · [`@gh-issue-list`](list/SKILL.md) · [`@gh-issue-view`](view/SKILL.md)
- [Planning and issues (root README)](../../../README.md#planning-and-issues)
- [`internal-read-plan-parts`](../../read/plan/parts/SKILL.md) · [`internal-read-gh-issue-dedupe`](../../read/gh/issue-dedupe/SKILL.md) · [`internal-read-gh-issue-description`](../../read/gh/issue-description/SKILL.md) · [`internal-read-gh-issue-projects-relationships`](../../read/gh/issue-projects-relationships/SKILL.md)
- [`@gh-pr`](../pr/SKILL.md)

- When producing many issue drafts before mutation, store body files under `.cursor/gh/issues/` and reference them during `gh issue create --body-file`.
