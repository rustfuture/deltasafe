---
name: gh-issues-view
description: >-
  GitHub issues (`gh issue view`): read-only single issue: URL, #n, or id; rich JSON and optional comments for planning. Hand off to @git-start
  for branch naming. Command shapes: internal-write-gh-issue-commands. No mutation.
---

# GitHub: view issue

Normative fences / full matrix: [`internal-write-gh-issue-commands`](../../../../write/gh/issue-commands/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-issue-list` first when the target issue is not known yet.
2. Use this skill directly when the user references one issue URL/number/id.
3. View JSON fences: **`internal-write-gh-issue-commands`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-write-gh-issue-commands`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@gh-issue-review` when the viewed issue needs reshaping.
2. Use `@git-start` when issue context is ready to drive implementation.

## Q&A bypass ENV

- `SKIP_QA_GH_ISSUES_VIEW=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Resolve the issue reference from **chat**: full **`https://github.com/owner/repo/issues/N`**, **`#N`**, or numeric **`N`** (default repo context unless user names **`owner/repo`**).
- Open **[`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)** and execute the **View** section **fenced blocks** for that reference; include **`comments`** in the **`--json`** field list when the user needs the thread for review or planning.
- Reply in chat with a **stable outline**: **title**, **state**, **labels**, **assignees**, **URL**, short **body** summary, optional **comment** highlights.
- When the user will branch next, paste **title** (one line) for **`@git-start`** branch naming.

## Do not

- Edit/create, close, or delete via **`@gh-issues`**, **`@gh-issue-close`**, or **`@gh-issue-delete-closed`** (bulk) as appropriate.

## See also

- [`@gh-issue-review`](../review/SKILL.md) — deep read + peer compare before **`@gh-issues`**
- [`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)
- [`@git-start`](../../../git/start/SKILL.md)
- [`WORK_PLAN.md`](../../../read/gh/issue-description/work-plan/SKILL.md) — local plan scaffold that embeds an issue
