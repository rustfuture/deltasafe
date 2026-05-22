---
name: git-rebase
description: >-
  Rebase current branch onto canonical main (or another target) with conflict resolution via internal-read-git-merge-conflicts.
  No push in this skill; publish separately with @git-push after review.
---

**Normative workflow:** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-rebase-workflow).

# Rebase (current branch onto target)

Normative fences / full matrix: [`internal-read-git-merge-conflicts`](../../read/git/merge-conflicts/SKILL.md), [`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-pull` first when canonical main should be merged before rewriting history.
2. Use after preserving local work if rewrite risk requires a safety savepoint.
3. Use this skill when rebasing current branch onto canonical main/target is desired.
4. Rebase/conflict fences: **`internal-read-git-workflows`** and **`internal-read-git-merge-conflicts`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`
2. `internal-read-git-merge-conflicts`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-review` after rebase to validate workspace health.
2. Use `@git-push` for publish flow once rebase results are confirmed.

## Q&A bypass ENV

- `SKIP_QA_GIT_REBASE=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-rebase-workflow)** in order.
- Use **`origin/main`** as the default target when rebasing onto repository main.
- If conflicts occur, run **[`internal-read-git-merge-conflicts`](../../read/git/merge-conflicts/SKILL.md)** as the conflict playbook and continue rebase until complete.

## Do not

- Rebase `main` itself in this skill flow; use **`@git-main`** to align local `main`.
- Push rewritten history from this skill; publishing belongs to **`@git-push`** after user confirmation.
- Mix merge and rebase strategies in one invocation unless the user explicitly requests it.

## On invoke

*`@git-rebase`* — Determine target (`canonical main` by default). If unclear, ask for the target branch/ref before running workflow steps.

## Notes

- After a successful rebase, run **`@git-review`** before publishing, or use **`@gh-pr`** (which runs **`@git-review`** before **`@git-push`**).
- Rebase rewrites commits; confirm intent when branch history is already shared.
