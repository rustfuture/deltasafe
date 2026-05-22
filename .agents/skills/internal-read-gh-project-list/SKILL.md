---
name: internal-read-gh-project-list
description: >-
  Normative gh project list inventory shapes. Read-only. Callers add --owner, --closed, --limit, and --format/--jq
  flags per goal; mutation lives in internal-write-gh-project-commands.
---

# Internal: GitHub project list (`internal-read-gh-project-list`)

**Library.** **Sole** normative **`gh project list`** fences for this pack. Consumed by **`@gh-project-delete-closed`** for preview/inventory before any delete mutation.

**Mutations** (**`gh project delete`**) live in **[`internal-write-gh-project-commands`](../../../write/gh/project-commands/SKILL.md)**—never run those from this file.

## Cadence (read-only)

This library is read-only and can run without a mutation confirm. Do **not** infer that **Proceed** is satisfied for any follow-on **`gh project delete`** command.

## Auth scope

`gh project` commands require token scope **`project`**. Verify with:

```bash
gh auth status
```

If needed, add scope:

```bash
gh auth refresh -s project
```

## Caller refinement (flags)

Use the fenced shapes below as bases. Callers add flags for the task at hand:

- `--owner "@me"` or `--owner ORG_LOGIN`
- `--limit N`
- `--closed` (includes closed projects in the result set)
- `--format json` (structured preview output)
- `--jq '…'` to filter for closed-only views and compact previews

## List projects for an owner

```bash
gh project list --owner "@me" --limit 30 --format json
```

## List including closed candidates

```bash
gh project list --owner "@me" --closed --limit 100 --format json
```

## Closed-only compact preview (owner + number + title)

```bash
gh project list --owner "@me" --closed --limit 100 --format json --jq '.projects[] | select(.closed == true) | {owner: .owner.login, number: .number, title: .title}'
```

## See also

- **[`internal-read-gh-issue-projects-relationships`](../issue-projects-relationships/SKILL.md)** — attach issues to projects from `@gh-issues` (read-only playbook + discovery).
- **[`internal-write-gh-project-commands`](../../../write/gh/project-commands/SKILL.md)** — `gh project delete` command shapes.
- **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** — required before any project deletion flow.
