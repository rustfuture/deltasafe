---
name: gh-pr-close
description: >-
  Pull requests (`gh pr close`): Goal + AskQuestion before PR close mutation. Mutates GitHub. Does not merge; use only when the user explicitly wants the pull request closed.
---

# GitHub: close pull request

Normative fences / full matrix: [`internal-write-gh-pr-commands`](../../../../write/gh/pr-commands/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-pr-view` first to confirm target PR state and closure rationale.
2. Use `@gh-pr-list` first when the user only gives broad PR filters.
3. Close/view command shapes: **`internal-write-gh-pr-commands`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-write-gh-pr-commands`
2. `internal-write-plan-structured-qa`
3. `internal-write-plan-skill-safety`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Return to `@gh-pr` when the user wants to reopen through a new PR workflow.
2. Use `@gh-pr-list` to verify closure inventory after mutation.

## Q&A bypass ENV

- `SKIP_QA_GH_PR_CLOSE=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- **Goal** + **AskQuestion** + **Proceed** before **Close PR** in **[`internal-write-gh-pr-commands`](../../../write/gh/pr-commands/SKILL.md)** ([`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md) **§3a**, [`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)).
- Confirm **PR number**, **repo** (`--repo` when upstream), and that closing matches user intent (superseded branch, wrong target, duplicate PR).
- Prefer **`@gh-pr-list`** / **`@gh-pr-view`** first when the PR number is unknown.

## Do not

- Close when the user meant to **merge**, **draft**, or **update** title/body—redirect to **`@gh-pr`** or the web UI.

## See also

- [`@gh-pr`](../SKILL.md)
