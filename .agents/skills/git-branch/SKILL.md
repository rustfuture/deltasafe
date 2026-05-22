---
name: git-branch
description: >-
  Branch hygiene workflow: inspect branches, prune stale remotes, delete merged local branches, and optionally rename current branch.
  No pull request mutations and no implicit push.
---

**Normative workflow:** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-branch-workflow).

# Branch (cleanup and hygiene)

Normative fences / full matrix: [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use after `@git-review` when you want branch cleanup on a healthy tree.
2. Use `@git-main` first if local main alignment is still pending.
3. Branch hygiene fences: **`internal-read-git-workflows`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-start` when branch hygiene should flow into new feature branch creation.
2. Use `@git-push` when cleaned branch state should be published.

## Q&A bypass ENV

- `SKIP_QA_GIT_BRANCH=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-branch-workflow)** in order.
- Preview branch impact before delete operations.
- Use structured confirmation before deleting local branches that are not obviously merged.

## Do not

- Delete the currently checked-out branch.
- Force-delete unmerged branches without explicit user confirmation.
- Push, open PRs, or run issue mutations from this skill.

## On invoke

*`@git-branch`* — If the requested action is unclear, ask whether the user wants list/prune/delete/rename first.

## Notes

- This skill complements **`@git-pull`** and **`@git-main`** by keeping branch lists clean.
- For one-off branch creation from a task, use **`@git-start`**.
