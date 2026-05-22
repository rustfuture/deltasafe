---
name: git-stash
description: >-
  Stash and restore local changes safely when you need a clean tree for other workflows. Supports push/list/show/pop/apply/drop/clear
  with explicit confirmation before destructive stash deletions. No push, no PR, no branch creation.
---

**Normative workflow:** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-stash-workflow).

# Stash (save local work temporarily)

Normative fences / full matrix: [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-commit` first when a savepoint should exist before parking edits in stash.
2. Use before clean-tree workflows when local edits should be temporarily parked.
3. Use this skill directly for stash lifecycle operations with explicit safety gates.
4. Workflow gates: **`internal-read-git-workflows`** and **`internal-write-plan-skill-safety`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`
2. `internal-write-plan-skill-safety`
3. `internal-write-plan-structured-qa`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Resume with `@git-pull`, `@git-main`, or `@git-reset` after stashing if needed.
2. Use `@git-review` after applying/pop to revalidate workspace state.

## Q&A bypass ENV

- `SKIP_QA_GIT_STASH=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-stash-workflow)** in order for stash save/inspect/restore operations.
- Use **structured confirmation** before destructive stash operations (`drop`, `clear`) per **[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)**.
- Prefer explicit stash labels tied to issue/branch context so later recovery is predictable.

## Do not

- Use this skill as a replacement for commits when work is ready to keep.
- Run **`@git-push`**, **`@gh-pr`**, or branch orchestration from here.
- Clear stashes without a preview of stash entries and user confirmation.

## On invoke

*`@git-stash`* — If the user intent is not explicit (`save`, `pop`, `apply`, `drop`, `clear`, or `list`), ask which stash action they want first.

## Notes

- This skill complements **`@git-main`**, **`@git-pull`**, and **`@git-reset`** when the working tree must be clean.
- For long-lived or shared work, prefer a normal commit on a branch over stash storage.
