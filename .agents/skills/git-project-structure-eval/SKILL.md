---
name: git-project-structure-eval
description: >-
  Public router for repository structure/readiness evaluation. Runs a read-only internal batch and returns posture + prioritized themes.
---

# `@git-project-structure-eval` (read-only posture snapshot)

Normative fences / full matrix: [`internal-read-git-repo-classification`](../../read/git/repo-classification/SKILL.md).


**Public.** **Goal:** evaluate the current repository structure/readiness in one pass and return a planning-grade posture report without running installs/tests or mutating git/GitHub.

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-review` first when posture should include fresh verification context.
2. Use **Cursor Plan** or chat first when evaluation is only one input to a larger planning loop.
3. Repo posture batch: **`internal-read-git-repo-classification`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-repo-classification`
2. `internal-read-git-discover-dependencies` (optional, when available/relevant)
3. `internal-read-git-project-structure-eval`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use **Cursor Plan** or **`@gh-issues`** Sharpen rounds to resolve ambiguity in prioritized themes.
2. Use **`@git-start`** once a stable brief or issue anchor is approved.

## Q&A bypass ENV

- `SKIP_QA_GIT_PROJECT_STRUCTURE_EVAL=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Keep this skill read-only and orchestration-only.
- Return the full output contract from `internal-read-git-project-structure-eval`.

## Do not

- Do not run installs/tests here.
- Do not mutate git or GitHub from this skill.
- Do not duplicate runnable command fences in this public file.
