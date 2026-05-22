---
name: internal-write-gh-issue-commands
description: >-
  Normative gh issue view, delete, edit, create, and close CLI shapes (write / GitHub). List/search inventory lives in
  internal-read-gh-issue-list. Callers run structured Q&A before mutating commands. Mutation fences grouped at end;
  execution is agent-led per parent skill.
---

# Internal: GitHub issue CLI (`internal-write-gh-issue-commands`)

**Write library (GitHub / `gh`).** **`gh issue`** command shapes for **view**, **delete**, **edit**, **create**, and **close**, consumed by **`@gh-issue-view`**, **`@gh-issue-delete-closed`** (delete only), **`@gh-issue-close`**, **`@gh-issue-review`** (view path), and **`@gh-issues`** (router owns create-or-edit). **Mutations** require **Goal + AskQuestion + Proceed** in the **public** skill via **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)**.
**Master-confirm rule:** **`SKIP_QA_*`** and **`SKIP_QA_WRITE`** do not bypass session-level confirm for any GitHub mutation in this library.

**List / search** — **[`internal-read-gh-issue-list`](../../read/gh/issue-list/SKILL.md)** (sole **`gh issue list`** / **`gh search issues`** fences).

**`gh repo view`** for **issue/PR templates** is **not** here—use **[`internal-read-gh-repo-forms-json`](../../read/gh/repo-forms-json/SKILL.md)**.

## View (single issue)

**Full extract (preferred for planning/execution):**

```bash
gh issue view <N> --json id,number,title,body,state,stateReason,url,author,assignees,labels,milestone,projectCards,closed,closedAt,createdAt,updatedAt,reactionGroups,comments
```

**Full extract with explicit repo:**

```bash
gh issue view <N> --repo owner/repo --json id,number,title,body,state,stateReason,url,author,assignees,labels,milestone,projectCards,closed,closedAt,createdAt,updatedAt,reactionGroups,comments
```

**By number** (current repo):

```bash
gh issue view <N> --json number,title,body,state,url,labels,assignees
```

**With comments** (when user needs thread for planning):

```bash
gh issue view <N> --json number,title,body,state,url,labels,assignees,comments
```

**From URL** — resolve **`owner/repo`** and **`N`** from `https://github.com/owner/repo/issues/N`, then:

```bash
gh issue view <N> --repo owner/repo --json number,title,body,state,url,labels,assignees,comments
```

**`#123` in chat** — treat as numeric **`<N>`** for default **`gh`** repo context.

## Delete one closed issue

**Only after** structured **Proceed** in **`@gh-issue-delete-closed`**. Flag availability varies by **`gh`** version—check **`gh issue delete --help`**.

```bash
gh issue delete <N> --yes
```

Use **`--repo owner/repo`** when not in default context.

## Edit issue (after Proceed)

**Only after** structured **Proceed** in **`@gh-issues`** when the chosen path is edit. Compose flags from **[`internal-read-gh-issue-description`](../../read/gh/issue-description/SKILL.md)** output and **[`internal-read-gh-issue-labels`](../../read/gh/issue-labels/SKILL.md)** when labels change. Multi-intent **Proceed — batch** may include **multiple** `gh issue edit` lines (and/or creates) in one confirmed batch—in the summarized order.

```bash
gh issue edit <N> --title "New title"
```

```bash
gh issue edit <N> --body-file /path/to/body.md
```

```bash
gh issue edit <N> --add-label "bug" --remove-label "triage"
```

## Create issue (after Proceed)

**Only after** structured **Proceed** in **`@gh-issues`** when the chosen path is create. Full checklist and body file patterns: **[`issue-create-prompt/SKILL.md`](./issue-create-prompt/SKILL.md)**. When **`@gh-issues`** multi-intent batching applies, **several** `gh issue create` (and/or `gh issue edit`) commands may run **in sequence** after **one** parent Proceed, in the order fixed in that Proceed summary.

```bash
gh issue create --title "…" --body-file /path/to/body.md
```

```bash
gh issue create --title "…" --body-file /path/to/body.md --label "bug,area:skills"
```

## Project attach (same Proceed batch as create/edit)

**Plan + summarize** before **`gh`** using **[`internal-read-gh-issue-projects-relationships`](../../read/gh/issue-projects-relationships/SKILL.md)** (discovery, auth, ambiguity, GHES fallbacks). **Additive** only: do not skip structured Q&A / Proceed; if attach is skipped after a failed command, say so explicitly (not a silent success).

**Create** — add by **project title** (requires **`project`** scope; see **`gh issue create --help`**):

```bash
gh issue create --title "…" --body-file /path/to/body.md --project "Project title"
```

```bash
gh issue create --title "…" --body-file /path/to/body.md --label "bug,area:skills" --project "Project title" --repo owner/repo
```

**Edit** — when the issue exists but is missing from a board (**`gh issue edit --help`** lists **`--add-project`**):

```bash
gh issue edit <N> --add-project "Project title"
```

```bash
gh issue edit <N> --add-project "Project title" --repo owner/repo
```

**Fallback** — add an existing issue URL to a project by **number** (when title-based attach is unsuitable or failed once with an unambiguous **`PROJECT_NUMBER`**):

```bash
gh project item-add <PROJECT_NUMBER> --owner OWNER_LOGIN --url https://github.com/owner/repo/issues/<N>
```

**Link project to repo** (maintainer setup; only when the user explicitly asked—may be a separate Proceed):

```bash
gh project link <PROJECT_NUMBER> --owner OWNER_LOGIN --repo owner/repo
```

## Close issue (after Proceed)

**Only after** structured **Proceed** in **`@gh-issue-close`**.

```bash
gh issue close <N> --comment "…"
```

## See also

- **[`internal-read-gh-issue-projects-relationships`](../../read/gh/issue-projects-relationships/SKILL.md)** — project discovery + issue relationship playbook (read-only).
- **[`internal-read-gh-issue-list`](../../read/gh/issue-list/SKILL.md)** — **`gh issue list`** / **`gh search issues`** (read-only inventory).
- **[`internal-write-gh-pr-commands`](../pr-commands/SKILL.md)** — **`gh pr create`**, **`gh pr edit`**, and **`gh pr close`** command shapes.
- **[`internal-read-gh-repo-forms-json`](../../read/gh/repo-forms-json/SKILL.md)** — **`gh repo view --json`** for template metadata.
- [`@gh-issue-review`](../../../gh/issues/review/SKILL.md)
- [`@gh-issue-view`](../../../gh/issues/view/SKILL.md)
- [`@gh-issue-list`](../../../gh/issues/list/SKILL.md)
- [`@gh-issue-pick`](../../../gh/issues/pick/SKILL.md)
- [`@gh-issue-delete-closed`](../../../gh/issues/delete-closed/SKILL.md)
- [`@gh-issues`](../../../gh/issues/SKILL.md) · [`@gh-issue-close`](../../../gh/issues/close/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
