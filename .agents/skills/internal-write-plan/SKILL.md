---
name: internal-write-plan
description: >-
  Cross-cutting safety and structured Q&A libraries used by planning, git, and GitHub workflows.
---

# `skills/write/plan/`

Shared prompt-safety write libraries.

- `structured-qa` defines confirmation and option-prompt UX.
- `skill-safety` defines when confirmation is mandatory before mutating actions.

These libraries are not public workflow entrypoints; they are consumed by public skills.

- For plan-driven gh workflows, these libraries also govern final keep/delete prompts for temporary `.cursor/gh/issues/` and `.cursor/gh/pr/` artifacts.
