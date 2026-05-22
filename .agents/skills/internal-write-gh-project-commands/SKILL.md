---
name: internal-write-gh-project-commands
description: >-
  Normative gh project delete CLI shapes (write / GitHub). List/inventory lives in internal-read-gh-project-list.
  Callers run Goal + AskQuestion + Proceed before mutations.
---

# Internal: GitHub project CLI (`internal-write-gh-project-commands`)

**Write library (GitHub / `gh`).** **`gh project`** command shapes for delete flows consumed by **`@gh-project-delete-closed`**.

**Mutations** require **Goal + AskQuestion + Proceed** in the public skill via **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)**.

**Master-confirm rule:** `SKIP_QA_*` and `SKIP_QA_WRITE` do not bypass session-level confirm for any GitHub mutation in this library.

**List / preview** lives in **[`internal-read-gh-project-list`](../../../read/gh/project-list/SKILL.md)**.

## Delete one project

Only after structured **Proceed** in **`@gh-project-delete-closed`**.

```bash
gh project delete <NUMBER> --owner "@me"
```

Use `--owner ORG_LOGIN` for organization-owned projects.

## See also

- **[`internal-read-gh-project-list`](../../../read/gh/project-list/SKILL.md)** — `gh project list` inventory fences.
- [`@gh-project-delete-closed`](../../../gh/projects/delete-closed/SKILL.md)
- [`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
