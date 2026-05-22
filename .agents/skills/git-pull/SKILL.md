---
name: git-pull
description: >-
  Fetch; merge @{u} if tracked; merge canonical main (upstream/main if fork else origin/main); resolve conflicts
  via internal-read-git-merge-conflicts. Does NOT @git-review or @git-push—run publish in a separate invocation.
---

**Normative workflow (commands):** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-pull-workflow).

# Pull (merge + resolve conflicts)

Normative fences / full matrix: [`internal-read-git-merge-conflicts`](../../read/git/merge-conflicts/SKILL.md), [`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md).


**Safety / prompts** — Merge commits completing this skill follow the **`@git-pull`** carve-out in **[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** (**Exception: `@git-pull` merge commits**): no extra **AskQuestion** per merge. If the working tree is **ambiguous** before you merge, stop and use open-ended **chat** per **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** **§4**. **`@git-push`** still owns commit + push confirmation when publishing.

**Conflict resolution** — When merges conflict, execute **[`internal-read-git-merge-conflicts`](../../read/git/merge-conflicts/SKILL.md)** **in full**. Do not reimplement that playbook here.

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-commit` or `@git-stash` first if local modifications need protection.
2. Use this skill when branch tracking/main alignment merge is required.
3. Pull/merge fences: **`internal-read-git-workflows`** and **`internal-write-plan-skill-safety`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`
2. `internal-write-plan-skill-safety`
3. `internal-write-plan-structured-qa`
4. `internal-read-git-merge-conflicts`
5. `internal-read-gh-repo-stream`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-review` after pull to validate workspace health.
2. Use `@git-push` when verified updates should be published.

## Q&A bypass ENV

- `SKIP_QA_GIT_PULL=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Execute **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-pull-workflow)** **in order** (branch check, fetch, tracking merge, canonical root merge, stop).
- **`CANONICAL_MAIN_REF` / fork semantics:** **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)**.
- On conflicts: **[`internal-read-git-merge-conflicts`](../../read/git/merge-conflicts/SKILL.md)** end-to-end.

## Do not

- **`@git-push`**, **full `@git-review`**, or publishing from this skill—next step is **`@git-review`**, **`@git-push`**, or **`@gh-pr`** per the user’s goal.
- Create or update PRs—that is **`@gh-pr`**.
- Use **`@git-main`** to sync a **feature** branch—**`@git-main`** is for **local `main`** only; merging **`main` into** the current branch is **`@git-pull`**.
- Inline duplicate conflict doctrine—use **`internal-read-git-merge-conflicts`**.

## On invoke

*`@git-pull`* — Start immediately. Run the linked **pull-workflow** once; on conflicts, **`internal-read-git-merge-conflicts`**; when merges are complete, finish at workflow **§6**.

## Notes

*`@git-pull`*
- **Next steps (outside `@git-pull`):** **`@git-review`** — verify install / lint / test / build. **`@git-push`** — commit-if-needed + publish per **`internal-read-git-workflows`** push section (no bundled verify). **`@git-docs`** separately when broader doc accuracy is needed. **`@gh-pr`** — **`@git-pull`**, **`@git-review`**, **`@git-push`**, then PR title/body.
- See also: **[`internal-read-git-merge-conflicts`](../../read/git/merge-conflicts/SKILL.md)**, **[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)**, **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)**.
