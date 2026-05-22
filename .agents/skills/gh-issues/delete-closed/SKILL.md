---
name: gh-issues-delete-closed
description: >-
  GitHub issues (`gh issue delete` on closed). Preview closed issues, Goal + AskQuestion Proceed, then delete mutation per id (destructive). List shapes:
  internal-read-gh-issue-list; delete: internal-write-gh-issue-commands. Mutates GitHub.
---

# GitHub: delete closed issues (bulk)

Normative fences / full matrix: [`internal-write-gh-issue-commands`](../../../../write/gh/issue-commands/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-issue-list` to preview closed inventory and expected delete set.
2. Use `@gh-issue-view` when any candidate needs manual validation first.
3. Delete/list command shapes: **`internal-read-gh-issue-list`** and **`internal-write-gh-issue-commands`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-gh-issue-list`
2. `internal-write-gh-issue-commands`
3. `internal-write-plan-skill-safety`
4. `internal-write-plan-structured-qa`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@gh-issue-list` again to confirm post-delete state.
2. Route future issue authoring through `@gh-issues` after cleanup.

## Q&A bypass ENV

- `SKIP_QA_GH_ISSUES_DELETE_CLOSED=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

1. **Goal** — Why closed issues should be removed from GitHub (housekeeping, compliance, etc.).
2. **Inventory (read-only)** — Run **List closed** from **[`internal-read-gh-issue-list`](../../../read/gh/issue-list/SKILL.md)**. Show **count** and a **preview** (ids + titles). Warn: **irreversible** for viewers without trash restore; requires **permissions** to delete issues.
3. **AskQuestion** — **Proceed** / **Cancel** before **any** delete mutation.
4. **On Proceed only** — For each id to delete, execute **Delete one closed issue** in **`internal-write-gh-issue-commands`** (flag availability: that library’s notes). Stop on first CLI error and report.
5. **Pasteable checklist** — **[`DELETE_CLOSED_ISSUES.md`](../../../write/gh/issue-commands/delete-closed-issues/SKILL.md)** (human summary; normative list shapes **`internal-read-gh-issue-list`**; delete **`internal-write-gh-issue-commands`**).

## Do not

- Delete issues **before** user **Proceed**.
- Delete **open** issues from this skill—scope is **closed** cleanup only unless the user explicitly re-scopes in chat (then re-run **Goal** + preview).

## See also

- [`internal-read-gh-issue-list`](../../../read/gh/issue-list/SKILL.md)
- [`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)
- [`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
