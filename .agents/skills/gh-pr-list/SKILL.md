---
name: gh-pr-list
description: >-
  Pull requests (`gh pr list`): read-only pull request list for inventory; no AskQuestion by default. Shapes in internal-read-gh-pr-list; narrative hub in list-view-hub/SKILL.md.
---

# GitHub: list pull requests

Normative fences / full matrix: [`internal-read-gh-pr-list`](../../../../read/gh/pr-list/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-pr-view` when one pull request needs richer detail before further action.
2. Use this skill directly for read-only PR inventory requests.
3. List/view normative fences: **`internal-read-gh-pr-list`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-gh-pr-list`
2. `internal-write-plan-skill-safety`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Hand off to `@gh-pr` when listing should become create/edit execution.
2. Use `@gh-pr-close` only after explicit close intent is confirmed.

## Q&A bypass ENV

- `SKIP_QA_GH_PR_LIST=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **List** (and related inventory) from **[`internal-read-gh-pr-list`](../../../read/gh/pr-list/SKILL.md)** — run **only** the fenced blocks defined there.
- **Refine queries** per user intent: add **`--state`** (e.g. **`closed`** for merged+closed), **`--head`**, **`--base`**, **`--limit`**, **`--json`**, **`--repo`**, as in that internal library’s **Caller refinement** section. Optional context: **[`LIST_VIEW_HUB.md`](../../../read/gh/pr-list/list-view-hub/SKILL.md)** (hub, no duplicate fences).
- Report results in chat; **no** default confirmation for read-only listing. **Structured confirm** is required before any **`gh pr`** / **`gh api`** **mutation** (**`@gh-pr-close`**, **`@gh-pr`**) per **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)**.

## Do not

- Create, edit, merge, or close PRs in this skill.

## See also

- [`@gh-pr-view`](../view/SKILL.md)
- [`@gh-pr`](../SKILL.md)
