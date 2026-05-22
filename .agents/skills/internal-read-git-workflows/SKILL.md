---
name: internal-read-git-workflows
description: >-
  Read-only in-pack git workflow references for public git skills, covering
  branch/main/pull/push/reset/stash/revert/rebase/zip guidance.
---

# In-pack git workflows

Use these sections as the in-repo workflow source for public `@git-*` skills.

High-risk steps referenced here (for example `git fetch`, `git push`, `git reset --hard`, `git clean`, or any out-of-repo write in delegated libraries) must still run behind [`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md) and [`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md). `SKIP_QA_*` / `SKIP_QA_WRITE` does not bypass session-level master confirm for those actions.

## Git start workflow

1. Derive `BRANCH` from Jira / issue / activity context.
2. Run full `@git-main` once. If `@git-main` aborts, stop `@git-start`.
3. After `@git-main` succeeds, create the feature branch from clean `main`:
   - `git checkout -b "$BRANCH"`
4. Hand off to full `@git-push` unless the user explicitly skips publish.

## Git main align workflow

1. Validate repository context:
   - `git rev-parse --is-inside-work-tree`
2. Check out `main` before fetch/reset:
   - `git checkout main`
   - If local `main` is missing, create from `origin/main`.
3. Fetch canonical refs:
   - `git fetch origin`
4. Resolve canonical main for standalone workflow:
   - `CANONICAL_MAIN_REF=origin/main`
   - `git rev-parse "$CANONICAL_MAIN_REF"` must succeed
5. If tree is clean and `HEAD == $CANONICAL_MAIN_REF`, stop.
6. Else align via `internal-write-git-working-tree-align` with:
   - `ALIGN_REF="$CANONICAL_MAIN_REF"`
   - `BRANCH=main`
   - `MODE=combined`
7. Do not `git pull` after reset/clean in this workflow.
8. Do not push in this workflow.

## Git pull workflow

1. Capture current branch; fail if detached:
   - `BRANCH=$(git branch --show-current)`
2. Fetch refs:
   - `git fetch origin`
   - If `upstream` exists, `git fetch upstream`
3. If branch has upstream tracking, merge `@{u}`; on conflict use `internal-read-git-merge-conflicts`.
4. Resolve canonical root branch via `internal-read-gh-repo-stream`:
   - `ROOT_BRANCH="$CANONICAL_MAIN_REF"` (`upstream/main` on fork, else `origin/main`)
5. Merge `"$ROOT_BRANCH"`; on conflict use `internal-read-git-merge-conflicts`.
6. Stop after merges complete; do not push in this workflow.

## Git push workflow

- **Read-only inventory** first (`git status --short`; optional **`HEAD...@{u}`** counts when upstream exists).
- Run **full `@git-commit`** **only when** that inventory shows **something to commit**; if the tree is clean, **skip** commit (still run publish when there are **unpushed commits**).
- **Publish** — **One** structured **Proceed** for branch intent + **`git push`** when the first attempt succeeds; **`origin`-aware** branch gate (**`main`** + **`origin`** → recommend new branch + later **`@gh-pr`**; no **`origin`** → single proceed on current branch). Use failure triage only when **`git push`** fails.
- **Full verify** before opening/updating a PR is **`@git-review`** inside **`@gh-pr`** (see **`@gh-pr`** fixed order), not a prerequisite of standalone **`@git-push`**.

## Git commit workflow

- Run **`@git-commit`** for local savepoints only (no review gate, no push).
- **Inventory first** — if **nothing to commit**, **no-op** with **no Q&A**.
- **Single gate** when work exists — default **`git add -A`**, confirm **`COMMIT_SUBJECT`**, **Cancel / Proceed**, then stage + **one** commit (optional **one** follow-up for ambiguous paths only).
- Use **`@git-push`** when publishing; use **`@git-review`** (or **`@gh-pr`**, which runs it) when you need a verify-backed gate before sharing work broadly.

## Git tag workflow

- **Optional publish-first** — When **`origin`** exists and the repo is **dirty**, **ahead** of **`@{u}`**, or the user asked to publish first, run **full `@git-push`** **before** **`@git-main`** + tag gates; otherwise skip with a short reason.
- Then **`@git-main`**, optional **`@git-commit`** when **`@git-push`** did not run and the tree is still dirty, **read-only tag inventory**, then **fewer Q&A gates** (publish intent → local collision → remote collision → **Proceed**) per **`@git-tag`**.
- **`@git-zip`** still hands off tag mutations only to **`@git-tag`**; export uses **`internal-write-git-zip`**.

## Git reset align workflow

- Resolve target ref, show impact, then align via reset/clean after confirm.

## Git stash workflow

- Save/list/show/apply/pop/drop with explicit confirmation for destructive steps.

## Git branch workflow

- Prune stale refs and optionally rename current branch.

## Git rebase workflow

- Rebase current branch onto target with conflict resolution.

## Git revert workflow

- Create inverse commits for selected commits (including merges when needed).

## Git zip workflow

- Export a **GitHub-style** **`.zip`** of **tracked files** at **`refs/tags/${TAG}`** via **`internal-write-git-zip`**.
- **`@git-zip`** orchestrates only: **`@git-tag`** when a tag must be created or repointed, then export; or read-only tag verification plus export when the tag already exists.
- **`@git-tag`** owns optional **publish-first `@git-push`**, sync **`main`**, label, and tag-push Q&A; **`@git-zip`** does not run tag-mutating fences from **`internal-write-git-tag`**.

## See also

- [`@git-zip`](../../../git/zip/SKILL.md)
- [`@git-tag`](../../../git/tag/SKILL.md)
- [`internal-write-git-tag`](../../../write/git/tag/SKILL.md)
- [`internal-write-git-zip`](../../../write/git/zip/SKILL.md)
- [`internal-write-git-commit`](../../../write/git/commit/SKILL.md)
- [`internal-write-git-working-tree-align`](../../../write/git/working-tree-align/SKILL.md)
