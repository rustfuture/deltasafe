---
name: git-revert
description: >-
  Create safe inverse commits for one or more existing commits, including merge-commit revert handling.
  Does not reset history and does not push automatically.
---

**Normative workflow:** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-revert-workflow).

# Revert (safe undo with new commits)

Normative fences / full matrix: [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-review` first when you need current failure evidence before revert.
2. Use after identifying exact commit SHAs that need safe inverse commits.
3. Revert workflow fences: **`internal-read-git-workflows`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-review` again to verify behavior after revert commits are created.
2. Use `@git-push` when reverted history is ready to publish.

## Q&A bypass ENV

- `SKIP_QA_GIT_REVERT=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-revert-workflow)** in order.
- Prefer revert for shared/published history instead of reset-based rewriting.
- Confirm target commit set and order before applying multi-commit reverts.

## Do not

- Use hard reset here; this skill is commit-based undo only.
- Push automatically; hand off to **`@git-push`** when the user wants to publish.
- Revert unknown merge commits without confirming parent selection.

## On invoke

*`@git-revert`* — Ask for commit hash/range (and merge parent when needed) if not explicitly provided.

## Notes

- For local-only discard workflows, use **`@git-reset`** instead.
- For conflict handling during revert, use the same conflict discipline as **`@git-pull`** and complete the revert sequence.
