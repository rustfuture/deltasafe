---
name: gh-issues-close
description: >-
  GitHub issues (`gh issue close`). Goal + AskQuestion before issue close mutation. For duplicate closure, follow [CLOSE_AS_DUPLICATE.md](.).
---

# GitHub: close issue

Normative fences / full matrix: [`internal-write-gh-issue-commands`](../../../../write/gh/issue-commands/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-issue-view` to confirm target issue context before closing.
2. Use `@gh-issue-review` when closure is driven by reshape or duplicate analysis.
3. Close/delete command shapes: **`internal-write-gh-issue-commands`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-write-gh-issue-commands`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Return to `@gh-issues` when the user wants to reopen or rewrite issue scope.
2. Use `@gh-issue-list` to verify resulting issue-state inventory.

## Q&A bypass ENV

- `SKIP_QA_GH_ISSUES_CLOSE=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- **Goal** + **AskQuestion** + **Proceed** before **Close issue** in **[`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)**.
- If closing as **duplicate**, follow **[`CLOSE_AS_DUPLICATE.md`](../../../write/gh/issue-commands/close-as-duplicate/SKILL.md)** (comment + close order).

## Do not

- Close without confirmation.

## See also

- [`@gh-issues`](../SKILL.md)
