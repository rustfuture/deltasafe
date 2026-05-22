---
name: gh-projects-delete-closed
description: >-
  GitHub Projects (`gh project delete` on closed): preview closed projects, Goal + AskQuestion Proceed, then delete mutation per number (destructive). List shapes:
  internal-read-gh-project-list; delete: internal-write-gh-project-commands. Mutates GitHub.
---

# GitHub: delete closed projects (bulk)

Normative fences / full matrix: [`internal-write-gh-project-commands`](../../../../write/gh/project-commands/SKILL.md).

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-projects-order-readonly` first when read-only layering would narrow which closed projects are in scope.
2. Ensure owner scope is explicit (`--owner "@me"` or org login) before any delete command.
3. Preview/list hub: **`internal-read-gh-project-list`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-gh-project-list`
2. `internal-write-gh-project-commands`
3. `internal-write-plan-skill-safety`
4. `internal-write-plan-structured-qa`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `internal-read-gh-project-list` again to confirm post-delete state.
2. Route future issue/PR work through existing `@gh-issues` and `@gh-pr` lanes.

## Q&A bypass ENV

- `SKIP_QA_GH_PROJECTS_DELETE_CLOSED=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

1. **Goal** — Why closed projects should be removed from GitHub (housekeeping, compliance, etc.).
2. **Inventory (read-only)** — Run list closed inventory from [`internal-read-gh-project-list`](../../../read/gh/project-list/SKILL.md). Show count and a compact preview (owner + number + title). Warn that deletion is irreversible and requires permissions plus `project` auth scope.
3. **AskQuestion** — **Proceed** / **Cancel** before any delete mutation.
4. **On Proceed only** — For each project number to delete, execute delete from `internal-write-gh-project-commands`. Stop on first CLI error and report.
5. **Pasteable checklist** — [`DELETE_CLOSED_PROJECTS.md`](../../../write/gh/project-commands/delete-closed-projects/SKILL.md) (human summary; normative list shapes in `internal-read-gh-project-list`; delete in `internal-write-gh-project-commands`).

## Do not

- Delete projects before user **Proceed**.
- Delete open projects from this skill—scope is closed cleanup only unless the user explicitly re-scopes in chat (then re-run Goal + preview).

## See also

- [`internal-read-gh-project-list`](../../../read/gh/project-list/SKILL.md)
- [`internal-write-gh-project-commands`](../../../write/gh/project-commands/SKILL.md)
- [`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
