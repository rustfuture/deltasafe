---
name: git-push
description: >-
  Orchestrate publish: read-only working-tree inventory; full @git-commit only when there is something to commit;
  one publish Proceed (branch gate + push + failure triage). No @git-review or @git-docs here—use @git-review alone
  or via @gh-pr when opening/updating a PR.
---

**Normative workflows:** **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-push-workflow)**.

# Push (commit if needed → publish)

Normative fences: [`internal-write-git-commit`](../../write/git/commit/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use **`@git-pull`** first if upstream changes may not be merged locally.
2. Use when implementation changes are ready to **record and publish** (not for full verify—see **`@git-review`**).
3. **Full verify** (install, format, lint, test, §8a docs polish) is **[`@git-review`](../review/SKILL.md)**. For pull requests, **`@gh-pr`** runs **`@git-review`** before this skill in its fixed order.
4. Publish workflow fences: **`internal-read-git-workflows`** and **`internal-write-plan-skill-safety`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`
2. `internal-write-plan-skill-safety`
3. `internal-write-plan-structured-qa`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use **`@gh-pr`** after publish completes to create or update PR metadata (**`@gh-pr`** runs **`@git-pull`** → **`@git-review`** → **`@git-push`** then PR work—do not skip ahead).
2. Use **`@git-review`** when you need a full workspace health pass without opening a PR.
3. Use **`@git-docs`** when docs accuracy requires a broader post-green pass.

## Q&A bypass ENV

- `SKIP_QA_GIT_PUSH=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- **§1 (read-only):** Run a **working-tree inventory** (same spirit as **Inspect working tree** in [`internal-write-git-commit`](../../write/git/commit/SKILL.md): e.g. `git status --short`; add `git diff` / `git diff --cached` when useful). If the branch has an upstream, note **ahead/behind** (e.g. `git rev-list --left-right --count HEAD...@{u}` when `@{u}` resolves).
- Run **complete** **[`@git-commit`](../commit/SKILL.md)** (**§2**) **only when** that inventory shows **something to commit** (non-empty short status: staged, unstaged, or untracked changes). If the tree is clean, **skip §2** entirely (no **`@git-commit`** hand-off). A clean tree with **unpushed commits** still proceeds to **§3** without §2.
- **§3** — Follow publish steps in **`push-workflow`** after **one** structured **Proceed** for **branch intent + push** (plus **[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** + **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** **§3a**). Do **not** stack extra micro-confirms on top of what **`@git-commit`** already resolved; **§3b** triage runs only when **`git push`** fails.
- **§3a — Branch gate (`origin`-aware):** Read **`origin`** with **`git remote get-url origin`** (or equivalent). **If `origin` exists** and the current branch is **`main`**, **recommend** **Create + switch to a new branch** from current **`HEAD`**, then **`git push -u`**, then **[`@gh-pr`](../../gh/pr/SKILL.md)** for GitHub review. Treat **Proceed on `main`** as an **explicit, non-default** path (hotfix / maintainer override) with clear risk wording. **If `origin` is missing**, default to **Proceed on current branch** (including **`main`**) in one gate—no branch+PR pressure; publishing may be impossible or local-only; state that honestly.
- **§3b** — If push fails, run failure triage per **`structured-qa` §8** (safe-first options with explicit next actions), then continue only on a selected recovery path.

## Do not

- Run **`@git-commit`** when the §1 inventory is **clean** (nothing to stage or commit)—skip §2 instead.
- Substitute ad-hoc format/lint/test commands for **[`@git-review`](../review/SKILL.md)** when the user asked for full verify—the matrix lives there (**[`internal-write-gh-evaluate`](../../write/gh/evaluate/SKILL.md)** via **`@git-review`**).
- Merge **`main`** or author PR bodies here—**[`@git-pull`](../pull/SKILL.md)**, **[`@gh-pr`](../../gh/pr/SKILL.md)** after push. Branch creation belongs to **`@git-start`** by default; **`@git-push`** may create/switch a recovery branch only inside **§3a/§3b** push handling.
- Assume **`@git-docs`** runs inside this skill; run **[`@git-docs`](../docs/SKILL.md)** separately when documentation updates are requested.

**When another skill publishes:** **`@git-start`** → **`@git-push`** on new branch. **`@gh-pr`** → **`@git-pull`** then **`@git-review`** then **`@git-push`** then PR text. **`@git-main`** / **`@git-pull`** alone do **not** push. Verify without push: **`@git-review`** alone.

## On invoke

*`@git-push`* — Run steps **1 → 2 → 3** in order. Run the **§1 read-only inventory** first; call **`@git-commit`** only when that inventory shows work to commit. During §3, use **one** branch+publish **Proceed** when push succeeds on the first attempt; use **§3b** only on failure.

## Workflow

### 1. Working-tree inventory (read-only)

> **`git status --short`** (and short diffs if useful); optionally **`git rev-list --left-right --count HEAD...@{u}`** when upstream exists. This step does **not** run **`@git-review`**.

### 2. **`@git-commit`** only if needed

> **If** there is **nothing to commit** (clean index and working tree), **skip** **`@git-commit`** and go to **§3**. **Else** execute the **entire** **[`@git-commit`](../commit/SKILL.md)** — single add+commit gate per that skill.

### 3. Publish

*`@git-push`* — Execute publish steps in **`push-workflow`** (one **Proceed** for branch intent + **`git push`**, then scratch cleanup when applicable). Ephemeral scratch dirs: **[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** (**Exception: ephemeral scratch paths in `@git-push`**).

### 3a. Branch pre-push gate

*`@git-push`* — In §3 preamble, surface current **`BRANCH`** and whether **`origin`** exists. **With `origin` on `main`:** **AskQuestion** once: **Abort**, **Create + switch to new branch from current `HEAD` then push** (**recommended**), or **Proceed on `main`** (non-default; name the risk). **Without `origin`:** prefer a single **Proceed / Abort** for publish intent on the current branch (including **`main`**).

### 3b. Push failure recovery loop

*`@git-push`* — If push returns non-zero, do not hard-stop immediately. Run failure triage per **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** **§8** with compact context summary above the prompt (branch, remote, concise error, next-step effects). Keep safe-first options: **Abort**, **Create/switch recovery branch + push `-u`**, **Retry current push**. Offer force-like paths (for example `--force-with-lease`) only as an explicit risk option with separate confirm per **[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** triggers.

## Notes

*`@git-push`*
- Run from the repo root.
- Run **[`@git-docs`](../docs/SKILL.md)** as a separate invocation when documentation updates are requested.
- After a successful push, to open or refresh a PR → **`@gh-pr`** (full chain: **`@git-pull`** → **`@git-review`** → **`@git-push`** → PR metadata).
