---
name: internal-read-gh-issue-dedupe
description: >-
  Read-only: list/search open issues, flag near-duplicates vs one or more candidate title+body rows; no gh writes.
  Pasteable checklist: ./dedupe-checklist/SKILL.md. Callers: @gh-issues, @gh-issue-review.
---

# Internal: Issue dedupe (`internal-read-gh-issue-dedupe`)

**Read-only library.** Compare **one or more candidate** issues (each: title + summary lines) to **open** issues using **`gh`**. **No** `gh issue create`, **no** comments, **no** closes.

**Consumers:** **`@gh-issues`**, **`@gh-issue-review`** (overlap pass), optionally humans.

## Do

1. Follow **[`dedupe-checklist/SKILL.md`](./dedupe-checklist/SKILL.md)** (**§1** defers all **`gh issue list`** / **`gh search issues`** shapes to **`internal-read-gh-issue-list`**—**do not** duplicate flags here).
2. Emit a short verdict **per candidate**: **`safe_to_create`** | **`likely_duplicate #n`** | **`ambiguous`** (list candidate numbers + one-line reason each). For a **single** candidate, one verdict is enough.

**Reshape-review (optional caller: `@gh-issue-review`).** When the **candidate** text describes **refining existing issue `#N`**, treat a hit on **`#N` itself** as **self**, not a duplicate signal—only **other** numbers are **peer overlaps** or **likely duplicate** targets. Verdict labels stay the same; interpret **`likely_duplicate #m`** as “**`#m`** collides with the reshaped scope.”

## `@gh-issues` orchestration order

Use this sequence when `@gh-issues` routes create-or-update behavior.

### Single candidate (default)

1. **List / search** — run `@gh-issue-list` (or equivalent via [`internal-read-gh-issue-list`](../issue-list/SKILL.md) + [`dedupe-checklist/SKILL.md`](./dedupe-checklist/SKILL.md)) to inventory open issues before any write.
2. **Dedupe verdict** — evaluate candidate title + body summary through this skill and return `safe_to_create` | `likely_duplicate #n` | `ambiguous`.
3. **Branch**
   - `likely_duplicate #n` (or `ambiguous` resolved to a single `#n`) -> treat as update path in `@gh-issues`.
   - `safe_to_create` -> continue create path in `@gh-issues`.

### Multi-candidate (optional `@gh-issues`)

After **`@gh-issues`** has **partitioned themes** (chat and/or **[`internal-read-plan-parts`](../../plan/parts/SKILL.md)** + plan hub files):

1. **List / search once** — same inventory pass as single-candidate step 1; reuse the same open-issue set for every theme. Optionally add a focused **`gh search issues`** query per theme only when the default list is too large to compare fairly.
2. **Per-theme verdict** — for each theme, compare its working title + one-line intent to that inventory; emit `safe_to_create` | `likely_duplicate #n` | `ambiguous` **per row**. **Never** skip this for a theme that will end in **create**.
3. **Merge similar themes before publish** — when two rows point at the same **`likely_duplicate #n`** or are clearly the same root cause, default to **one** issue (merge rows in the summary table); user confirms.
4. **Aggregate** — hand off a table (theme → verdict → proposed **edit #n** or **create**) to **`@gh-issues`** for structured Q&A and one **Proceed — batch** per **`internal-write-plan-structured-qa`**.

If clustering collapses to **one** theme, use **only** the single-candidate subsection above.

Optional first pass: run `@gh-issue-review` on an existing issue when the body is weak and needs codebase + overlap analysis before deciding update vs create.

## Do not

- Mutate GitHub state.
- Duplicate **`gh issue list`** / **`gh search issues`** recipes outside **`internal-read-gh-issue-list`** — the **checklist** template links there too.

## See also

- **[`internal-read-gh-issue-list`](../issue-list/SKILL.md)** — normative list/search **`gh`** lines.
- [`@gh-issues`](../../../gh/issues/SKILL.md) · [`@gh-issue-review`](../../../gh/issues/review/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
