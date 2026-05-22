---
name: internal-read-plan
description: >-
  Internal planning read-only libraries for profile selection, turn order, and readiness checks. Consumed by
  @gh-issues pre-flight shaping and related chat workflows (not a separate public @ invoke).
---

# `skills/read/plan/`

Read-only planning libraries that support issue pre-flight and **Sharpen / Abort / Ship** shaping inside **`@gh-issues`** (and similar callers), backed by **`internal-write-plan-structured-qa`**.

- `core/` provides profile selection, turn order, and readiness orchestration.
- Child profiles/checklists keep issue-ready and generic brief guidance consistent.
- Default planning artifact is `.cursor/plan.md` with optional linked sub-plans in `.cursor/plans/`.

No git or GitHub mutations should be executed from this layer.

- `parts/` manages disjoint multi-goal plan sections by domain.
- `domain-questions/` picks high-impact clarifying questions by domain.
- `confidence/` measures how close the task is to a well-defined executable brief.
- `qa-batch/` builds structured AskQuestion batches for planning loops.
