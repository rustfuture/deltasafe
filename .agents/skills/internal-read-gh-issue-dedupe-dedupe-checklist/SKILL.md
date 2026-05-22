---
name: internal-read-gh-issue-dedupe-dedupe-checklist
description: >-
  Read-only: issue dedupe checklist (defers list shapes to internal-read-gh-issue-list). Parent internal-read-gh-issue-dedupe.
---

# Issue dedupe checklist

Used by **`internal-read-gh-issue-dedupe`** and **`@gh-issues`** / **`@gh-issue-review`**.

1. **List / search open issues** — use **only** the **List / search** and **Search** sections of [`internal-read-gh-issue-list`](../issue-list/SKILL.md) (`gh issue list`, `gh search issues`, flags, limits). **Do not** invent alternate **`gh`** shapes in this checklist.
2. **Single candidate (default)** — Compare **titles** and first lines of **bodies** for near-duplicates against that inventory; note issue numbers. Output: **safe to create** / **likely duplicate #n** / **ambiguous — user must choose** (no **`gh`** writes in this phase).
3. **Multi-candidate (`@gh-issues` multi-intent only)** — (a) **Theme list** — stable ids, working title, one-line intent per theme (after clustering in the router). (b) **Single inventory** — reuse the issue set from step 1 for every theme; do **not** repeat a full list pass unless a theme needs an extra targeted **`gh search issues`** (large backlog). (c) **Per-theme compare** — same title/body skim as step 2, independently per row. (d) **Aggregate user choice** — table: theme → verdict → proposed **edit #n** or **create**; flag rows that should **merge** before mutation (same **`likely_duplicate #n`** or same root cause). No **`gh`** writes in this phase.
4. **`@gh-issue-review`** — same list/compare mechanics for **reshape** context; when reviewing **`#N`**, ignore **self** on **`#N`** when interpreting overlaps (see **`internal-read-gh-issue-dedupe`** reshape note).
