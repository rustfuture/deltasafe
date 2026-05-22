---
name: git-main
description: >-
  Align local main to origin/main using the linked workflow and internal working-tree align
  library. No stash. No push. Embedded in @git-start before new branch.
---

**Normative workflow:** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-main-align-workflow).

# Main (checkout `main` → fetch → match canonical remote → clean)

Normative fences / full matrix: [`internal-write-git-working-tree-align`](../../write/git/working-tree-align/SKILL.md), [`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-commit` or `@git-stash` first when local work must be preserved.
2. Use this skill directly when local main should be aligned to canonical main.
3. Main alignment fences: **`internal-read-git-workflows`** and **`internal-write-git-working-tree-align`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`
2. `internal-write-git-working-tree-align`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-start` to branch from the freshly aligned base.
2. Use `@git-push` when aligned work should be published.

## Q&A bypass ENV

- `SKIP_QA_GIT_MAIN=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Follow the linked **Normative workflow** end-to-end; it owns the operational sequence for aligning local **`main`**.
- Use **`origin/main`** as the canonical main ref for alignment in this standalone workflow.
- Use **[`internal-write-git-working-tree-align`](../../write/git/working-tree-align/SKILL.md)** for dirty-tree handling, reset/clean impact, and confirmations.
- Stay in **current workspace** repo and report whether **`main`** is aligned when done.

## Do not

- **Stash**—user commits, aborts, or confirms trash per **[`internal-write-git-working-tree-align`](../../write/git/working-tree-align/SKILL.md)**.
- Publish—**[`@git-push`](../push/SKILL.md)** only.
- Perform destructive alignment without structured confirm after showing impact (see working-tree-align).
- Reorder the linked workflow.
- Run **`@git-pull`** here—feature-branch sync is **`@git-pull`**, not **`@git-main`**.

**When to use:** (1) Standalone—clean **`main`**. (2) Inside **[`@git-start`](../start/SKILL.md)** before creating the new branch.

## Workflow Ownership

- **Order and command recipe:** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-main-align-workflow).
- **Dirty tree, destructive align, clean, optional cache pruning:** [`internal-write-git-working-tree-align`](../../write/git/working-tree-align/SKILL.md).

## On invoke

*`@git-main`* — Execute the linked workflow exactly once against **`origin/main`**. Do not publish from this skill.

## Notes

- **Canonical ref:** **`origin/main`**.
- **Feature branch** sync (merge **`main` into** that branch) → **`@git-pull`**, not **`@git-main`**.
- **`@git-start`** embeds **this** skill end-to-end.
