---
name: internal-read-gh-pr-list-list-view-hub
description: >-
  Read-only: PR list/view narrative hub (no duplicate gh fences). Parent internal-read-gh-pr-list.
---

# PR list / view hub

**Normative command fences** live in [`internal-read-gh-pr-list`](./SKILL.md). **PR mutations** (`gh pr create`, `gh pr edit`, `gh pr close`) live in [`internal-write-gh-pr-commands`](../../../write/gh/pr-commands/SKILL.md).

**Fork / upstream:** set **`--repo`** when viewing or listing against **`$UPSTREAM`** per [`internal-read-gh-repo-stream`](../repo-stream/SKILL.md).

| Task | Skill | Internal shapes |
| --- | --- | --- |
| Inventory / pick a PR | **`@gh-pr-list`** | **`internal-read-gh-pr-list`** — add **`--state`**, **`--head`**, **`--base`**, **`--limit`**, **`--json`**, **`--repo`** per caller |
| Inspect one PR | **`@gh-pr-view`** | **`internal-read-gh-pr-list`** **View** / **`gh pr diff --stat`** |
| Close a PR | **`@gh-pr-close`** | **`internal-write-gh-pr-commands`** **Close PR** (after **Proceed**) |

**Do not** use this hub to **replace** **`@gh-pr`** for **`gh pr create`** / **`gh pr edit`**—those stay behind **`@git-pull`** + **`@git-review`** + **`@git-push`** + **`internal-read-gh-pr-description`**.

When `@gh-pr` finds non-empty matching inventory, it may run an intent AskQuestion (edit existing vs create new vs abort) before drafting; command fences still stay in **`internal-read-gh-pr-list`**.

**Safety:** Any **online mutation** or **write outside the git workspace root** requires **Goal + structured confirm** per **`internal-write-plan-skill-safety`** before execution.
