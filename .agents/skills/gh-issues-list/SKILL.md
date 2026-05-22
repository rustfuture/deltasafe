---
name: gh-issues-list
description: >-
  GitHub issues (`gh issue list`, `gh search issues`): read-only list/search for inventory; no AskQuestion by default. Shapes in internal-read-gh-issue-list + dedupe-checklist.
---

# GitHub: list / search issues

Normative fences / full matrix: [`internal-read-gh-issue-list`](../../../../read/gh/issue-list/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-issue-view` first when list filters depend on one known issue.
2. Use this skill directly when the user asks for issue inventory only.
3. List/search normative fences: **`internal-read-gh-issue-list`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-gh-issue-list`
2. `internal-write-plan-skill-safety`
3. `internal-read-gh-issue-dedupe`
4. `internal-write-gh-issue-commands`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Hand off to `@gh-issues` when listing should become create/edit execution.
2. Use `@gh-issue-pick` when the user should choose the next issue to work on.

## Q&A bypass ENV

- `SKIP_QA_GH_ISSUES_LIST=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **List / search** (and inventory checklist shapes) from **[`internal-read-gh-issue-list`](../../../read/gh/issue-list/SKILL.md)** together with **[`DEDUPE_CHECKLIST.md`](../../../read/gh/issue-dedupe/dedupe-checklist/SKILL.md)**.
- **Refine queries** in chat or per user intent: add **`--state`**, **`--label`**, **`--limit`**, **`--json`**, **`--repo`**, or a **`gh search issues`** query string as described in that internal library’s **Caller refinement** section.
- Report results in chat; **no** default confirmation for read-only listing. **Structured confirm** (**Goal + AskQuestion + Proceed**) is still required before any follow-on **`gh issue`** **mutation**—use **`@gh-issues`**, **`@gh-issue-close`**, or **`@gh-issue-delete-closed`** ([`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)).

## Do not

- Create, edit, or close issues in this skill.

## See also

- [`@gh-issues`](../SKILL.md) — list → dedupe → create or update
- [`internal-read-gh-issue-dedupe`](../../../read/gh/issue-dedupe/SKILL.md)
- [`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md) — view / create / edit / close / delete only
