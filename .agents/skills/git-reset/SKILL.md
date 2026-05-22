---
name: git-reset
description: >-
  Stay on current branch; fetch --all; align to $TARGET (tracking/upstream/origin) via reset --hard + git clean after confirm.
  No stash. No push. No merge main—use @git-pull. Leaf: no hand-off to other git-* / gh-*.
---

**Normative workflow:** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-reset-align-workflow).

# Reset (stay on branch → keep or trash → discover → confirm → reset → clean)

Normative fences / full matrix: [`internal-write-git-working-tree-align`](../../write/git/working-tree-align/SKILL.md), [`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-commit` first when any local work needs preservation before reset.
2. Use when hard alignment is explicitly requested and local edits can be discarded.
3. Reset/working-tree fences: **`internal-read-git-workflows`** and **`internal-write-git-working-tree-align`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`
2. `internal-write-git-working-tree-align`
3. `internal-write-plan-structured-qa`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-pull` when merge-based sync is preferred over destructive alignment.
2. Use `@git-review` to recheck workspace health after reset completes.

## Q&A bypass ENV

- `SKIP_QA_GIT_RESET=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-reset-align-workflow)** **in order** (validate tree, stay on branch, fetch, resolve **`$TARGET`**, align, verify).
- Resolve **`$TARGET`** from local refs, tracking refs, or explicit origin refs provided by the user.
- Dirty tree and **hard reset / clean**: **[`internal-write-git-working-tree-align`](../../write/git/working-tree-align/SKILL.md)** (**§1–§4**, **`MODE=split`**, **`ALIGN_REF="$TARGET"`**). Optional non-git prunes: that library **§5**. Prompts per **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)**.
- **Recommendation:** use a high-reasoning model for conflict-prone cleanup decisions.

## Do not

- **Stash**—preserve work by commit, abort, or confirmed destroy only.
- **`@git-push`**, merge **`main`** into branch—use **`@git-pull`** / **`@git-push`** for that.
- Align (**hard reset**, **clean**, container prunes) **without** user confirmation after showing impact.

**Leaf:** does **not** hand off to other **`gh-*`** skills.

## On invoke

*`@git-reset`* — **Do not** run destructive align steps without **user confirmation** after you show what will happen.

**Never** use **stash** in this skill.

## Notes

- **No stash:** preserving work is **commit** or **abort**; this skill only **aligns** to **`$TARGET`** after explicit **trash** consent when the tree was dirty.
- Do not use if merge/rebase is in progress unless resolved first.
- Solo **`@git-reset`** only aligns **current** branch to **`$TARGET`**; it never checks out another branch.
