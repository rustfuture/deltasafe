---
name: internal-write-gh-issue-commands-delete-closed-issues
description: >-
  Read-only: bulk delete closed issues (gh shapes). Parent internal-write-gh-issue-commands.
---

# Bulk delete closed issues

**Normative CLI shapes** — **list:** [`internal-read-gh-issue-list`](../../../read/gh/issue-list/SKILL.md); **delete:** [`internal-write-gh-issue-commands`](./SKILL.md). **Public skill:** **`@gh-issue-delete-closed`**.

**Before you run anything**

1. **Goal** — Why delete closed issues.
2. **Preview** — List closed issues (ids + titles); confirm count matches intent.
3. **Structured confirm** — **Proceed** / **Cancel** per **`internal-write-plan-structured-qa`** + **`internal-write-plan-skill-safety`**.
4. **Permissions** — Deleting issues requires appropriate **GitHub** role; **`gh`** errors are authoritative.

**After Proceed:** Run **`gh issue delete <n> --yes`** (or per **`gh issue delete --help`**) one issue at a time; stop on failure.

**Do not** delete **open** issues from this checklist unless the flow was explicitly re-scoped.
