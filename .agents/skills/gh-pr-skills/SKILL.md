---
name: gh-pr-skills
description: >-
  Pull requests (`gh pr` bodies in skill repos): skill-pack PR model: when @gh-pr-list/@gh-pr-view vs full @gh-pr; PR body should highlight skills/** paths and link
  issues. Read-oriented hand-off skill; create/update stays @gh-pr after @git-pull + @git-review + @git-push.
---

# Skill-pack: pull requests from Cursor

Normative fences / full matrix: [`internal-read-gh-pr-description`](../../../../read/gh/pr-description/SKILL.md), [`internal-write-gh-pr-commands`](../../../../write/gh/pr-commands/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-pr-list` or `@gh-pr-view` first when PR inventory or one-PR detail should precede the skill-pack narrative.
2. PR title/body/delta internals for shipping live in **`internal-read-gh-pr-description`** and **`internal-write-gh-pr-commands`** (names only; **`@gh-pr`** owns mutations).

Use this as the **mental model** when most changes live under **`skills/**`** and the **`docs/`** wikis (diagram scaffolds: **`skills/read/git/doc-graphs/diagrams/`** when relevant).

## When to use what

| Need | Skill |
| --- | --- |
| See open PRs for a branch | **`@gh-pr-list`** |
| Inspect one PR (title, body, base, diff stat) | **`@gh-pr-view`** |
| **Create** or **replace** PR title/body after branch is green and **pushed** | **`@gh-pr`** (**`@git-pull`** → **`@git-review`** → **`@git-push`** → **`internal-read-gh-pr-description`** → PR create/edit mutation) |
| Close a PR the user explicitly wants gone | **`@gh-pr-close`** |

**Do not** open or replace PRs on GitHub outside **`@gh-pr`**—that skill owns the sync + verify + publish prelude and **`$BASE_GIT`** narrative.

## PR body expectations (skill repos)

- Call out **paths under `skills/`** (especially **`SKILL.md`** files) and **what behavior changed**.
- Link **GitHub issues** filed with **`@gh-issues`** (or paste **`Fixes #n`** / **`Refs #n`** in the body when appropriate).
- Reuse **[`PR_ORCHESTRATION.md`](../../../read/gh/pr-content/pr-orchestration/SKILL.md)**, **[`BODY_SKELETON.md`](../../../read/gh/pr-content/pr-body-skeleton/SKILL.md)**, **[`internal-read-gh-pr-content`](../../../read/gh/pr-content/SKILL.md)**.

## Linked issue workflow

- Track work with **`@gh-issue-list`** / **`@gh-issue-view`**, narrow scope, then **`@gh-issues`** when filing.
- Publish branch + PR with **`@gh-pr`** so the PR description matches **`$BASE_GIT..HEAD`**.

## See also

- [`@gh-pr`](../SKILL.md)
- [`@gh-pr-list`](../list/SKILL.md) · [`@gh-pr-view`](../view/SKILL.md) · [`@gh-pr-close`](../close/SKILL.md)
- [`@gh-issues`](../../../gh/issues/SKILL.md)

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use after `@gh-pr-list` or `@gh-pr-view` to ground the PR narrative in real context.
2. Run `@git-push` first when branch evidence may be stale.

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-gh-pr-content`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Route final PR create/edit to `@gh-pr` after the narrative is ready.
2. Use `@gh-issues` when PR outcomes need linked issue updates.

## Q&A bypass ENV

- `SKIP_QA_GH_PR_SKILLS=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.
