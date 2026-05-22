---
name: git-start
description: >-
  Full @git-main → new branch from derived name → full @git-push. Branch naming from Jira/activity/issue number in chat;
  use @gh-issue-view separately for live GitHub issue titles. Does NOT reimplement @git-main; does NOT open PR (@gh-pr).
  User may skip push if explicit.
---

**Normative workflow:** [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-start-workflow).

# Start (main → new branch from a task)

Normative fences / full matrix: [`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md), [`internal-write-git-working-tree-align`](../../write/git/working-tree-align/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use after issue context is clear (for example from `@gh-issue-view`).
2. Use `@git-main` first if you only need standalone main alignment.
3. Start-branch workflow fences: **`internal-read-git-workflows`** and **`internal-write-plan-structured-qa`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-workflows`
2. `internal-write-plan-structured-qa`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-push` publish flow for subsequent commit/push orchestration.
2. Use `@gh-pr` when the branch is ready for pull request creation/edit.

## Q&A bypass ENV

- `SKIP_QA_GIT_START=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Derive **`$BRANCH`** from Jira key, GitHub issue, or kebab-case activity.
- Run **full** **[`@git-main`](../main/SKILL.md)** then create the branch and run **full** **[`@git-push`](../push/SKILL.md)** on the new branch—**order and branch-creation recipe** in **`start-workflow`**, not reimplemented here.
- **Default:** publish via **`@git-push`** unless the user explicitly cancels publish.

## Do not

- Reimplement **`@git-main`** (fetch / reset / clean)—delegate to that skill.
- Open or update a PR—use **`@gh-pr`** when ready.
- Skip **`@git-push`** without user saying so (default is publish).

**Expected context (one of):**
- **Jira** — Ticket key (e.g. `TIS-503`, `PROJ-123`) or link. Branch name: lowercase with hyphen, e.g. `tis-503`, `proj-123`, or `tis-503-short-description` if a short description is given.
- **GitHub issue** — Issue number or URL (e.g. `#42`, `https://github.com/owner/repo/issues/42`). Branch name: `42` or `42-short-slug` (e.g. `42-fix-login`) from the issue title if available.
- **Activity / instruction** — Short phrase (e.g. "add user login", "refactor auth"). Branch name: **kebab-case** slug, e.g. `add-user-login`, `refactor-auth`. No spaces; keep it short.

## On invoke

*`@git-start`* — Execute **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-start-workflow)** end-to-end. If the user did not give a ticket, issue, or activity, ask: "What's the Jira ticket, GitHub issue, or activity for this branch?" If you are on **`main`** with **uncommitted changes**, **`@git-main`** will **not** stash: user must **commit**, **abort**, or **explicitly confirm trash** inside **`@git-main`**. Prefer **commit** before **`@git-start`** if they need to keep the work.

## Branch name examples

- **Jira** — input `TIS-503` → branch `tis-503`
- **Jira + short description** — input `TIS-503 add login` → branch `tis-503-add-login`
- **GitHub issue** — input `#42` → branch `42` or `42-fix-login` (from issue title when available)
- **GitHub issue URL** — e.g. `…/issues/17` → branch `17` or `17-refactor-api`
- **Activity phrase** — `add user login` → `add-user-login`; `Fix bug in parser` → `fix-bug-in-parser`

## Notes

*`@git-start`*
- Run from the repo root. Prerequisites: **`git`** only. Optional: **`@gh-issue-view`** before this skill if the branch name should reflect the live GitHub issue title.
- **Do not** run **`@git-main`** again in other skills when the user asked for **`@git-start`**—**`@git-start`** already includes it.
- **`@git-main`** aligns local **`main`** to **`origin/main`** for this workflow.
- Open-ended prompts (missing ticket, invalid branch name) follow **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)**.
- To **only** refresh **local** **`main`** without a new branch, invoke **`@git-main`** alone—not **`@git-start`**. To publish **`main`**, run **`@git-push`** after **`@git-main`**.
- For a PR: **`@gh-pr`** (**`@git-pull`** → **`@git-review`** → **`@git-push`** → PR text). For **verify only** (no push): **full** **`@git-review`**.
