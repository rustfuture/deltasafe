---
name: gh-pr-view
description: >-
  Pull requests (`gh pr view`, `gh pr diff --stat`): read-only pull request view / optional diff stat for one PR; no AskQuestion by default. Shapes in internal-read-gh-pr-list.
---

# GitHub: view pull request

Normative fences / full matrix: [`internal-read-gh-pr-list`](../../../../read/gh/pr-list/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-pr-list` first when the target PR is not yet identified.
2. Use this skill directly when the user references one PR URL/number.
3. View/diff-stat fences: **`internal-read-gh-pr-list`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-gh-pr-list`
2. `internal-read-gh-repo-stream`
3. `internal-write-plan-skill-safety`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@gh-pr` when viewed context should become PR create/edit work.
2. Use `@gh-pr-close` only when explicit close intent follows the review.

## Q&A bypass ENV

- `SKIP_QA_GH_PR_VIEW=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **View** (and optional diff-stat) from **[`internal-read-gh-pr-list`](../../../read/gh/pr-list/SKILL.md)** — run **only** the fenced blocks defined there.
- When **`--repo`** is required (fork **`$UPSTREAM`**), align with **[`internal-read-gh-repo-stream`](../../../read/gh/repo-stream/SKILL.md)**.
- **Structured confirm** before any follow-on **mutation** (**`@gh-pr-close`**, **`@gh-pr`**, …) per **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)**.

## Do not

- Mutate GitHub state from this skill.

## See also

- [`@gh-pr-list`](../list/SKILL.md)
- [`@gh-pr`](../SKILL.md)
