---
name: internal-write-git
description: >-
  Internal git-oriented mutating libraries for local tree alignment, commit savepoints, tag operations, and archive generation.
---

# `skills/write/git/`

Mutating git libraries used by public workflows.

- `working-tree-align` for reset/clean alignment routines.
- `commit` for local staging + commit savepoint fences.
- `tag` for local/remote tag command fences.
- `zip` for archive generation command fences.

Callers should apply structured safety/confirmation before destructive operations, online mutations, and out-of-repo writes via [`internal-write-plan-skill-safety`](../plan/skill-safety/SKILL.md) and [`internal-write-plan-structured-qa`](../plan/structured-qa/SKILL.md). `SKIP_QA_*` / `SKIP_QA_WRITE` never bypass session-level master confirm for those high-risk actions.
