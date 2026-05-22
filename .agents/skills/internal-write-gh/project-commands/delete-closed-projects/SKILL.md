---
name: internal-write-gh-project-commands-delete-closed-projects
description: >-
  Read-only: bulk delete closed projects (gh shapes). Parent internal-write-gh-project-commands.
---

# Bulk delete closed projects

**Normative CLI shapes** — **list:** [`internal-read-gh-project-list`](../../../../read/gh/project-list/SKILL.md); **delete:** [`internal-write-gh-project-commands`](../SKILL.md). **Public skill:** **`@gh-project-delete-closed`**.

**Before you run anything**

1. **Goal** — Why delete closed projects.
2. **Preview** — List projects for the selected owner and filter to closed (`closed == true`); confirm count, number, and title match intent.
3. **Structured confirm** — **Proceed** / **Cancel** per `internal-write-plan-structured-qa` + `internal-write-plan-skill-safety`.
4. **Permissions** — Deleting projects requires appropriate GitHub role and `project` auth scope; `gh` errors are authoritative.

**After Proceed:** run `gh project delete <NUMBER> --owner …` one project at a time; stop on first failure and report.

**Do not** delete open projects from this checklist unless the flow was explicitly re-scoped.
