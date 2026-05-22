---
name: internal-read-gh-pr-list
description: >-
  Normative gh pr list, gh pr view, and gh pr diff --stat read-only shapes. Callers add --state, --head, --base, --limit,
  --json, and --repo per goal; PR mutations live in internal-write-gh-pr-commands.
---

# Internal: GitHub pull request list / view (`internal-read-gh-pr-list`)

**Library.** **Sole** normative **`gh pr list`**, **`gh pr view`**, and read-only **`gh pr diff --stat`** fences for this pack. Consumed by **`@gh-pr-list`**, **`@gh-pr-view`**, **`@gh-pr`** (PR number lookup), and **`internal-write-gh-pr-commands`**.

**Mutations** (**`gh pr create`**, **`gh pr edit`**, **`gh pr close`**) live in **[`internal-write-gh-pr-commands`](../../write/gh/pr-commands/SKILL.md)**—never run those from this file.

## Cadence (read-only)

**Faster in inventory-only flows** — This library **only** hits GitHub for **read** list/view/diff-stat. **Parents** that are read-only (**`@gh-pr-list`**, **`@gh-pr-view`**, **`@gh-pr`** lookup, …) may use it **without** an extra **AskQuestion** from this file. **Do not** infer **Proceed** for **`gh pr close`** or **`gh pr create`/`edit`**—those use **`internal-write-gh-pr-commands`** or **`@gh-pr`** with their own confirms.

## Online changes and paths outside the repo

- **This file is read-only** — listing/viewing PRs does **not** replace **Goal + structured confirm** for any follow-on **`gh`** / API write.
- **Inventory stays lightweight** — no mandatory **Abort / Review / Proceed** stack for routine reads; mutation parents own **Proceed** per **`internal-write-plan-skill-safety`** / **`internal-write-plan-structured-qa`** **§1e** before writes.
- **Parent skills** that call **`internal-read-gh-pr-list`** and then **mutate** GitHub or **write outside the git workspace root** must apply **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** before those writes.
- **Master-confirm rule** — even with **`SKIP_QA_*`** / **`SKIP_QA_WRITE`**, follow-on high-risk writes still require session-level confirm in the owning mutation skill.

## Caller refinement (flags)

**Callers** compose the right flags for the question, for example:

- **`--state open`** | **`closed`** | **`merged`** | **`all`** (see **`gh pr list --help`** for your **`gh`** version)
- **`--head`** / **`--base`** (branch filters)
- **`--limit N`**
- **`--json field1,field2,…`**
- **`--repo owner/repo`** (**fork/upstream:** **[`internal-read-gh-repo-stream`](../repo-stream/SKILL.md)**)

## List (current repo)

Open PRs for the **current branch** (same-repo):

```bash
gh pr list --head "$(git branch --show-current)" --state open --json number,title,url --limit 20
```

All open PRs (cap output in chat):

```bash
gh pr list --state open --limit 30
```

**Closed, not merged** (optional read-only inventory):

```bash
gh pr list --state closed --limit 200 \
  --json number,title,state,mergedAt,url \
  --jq 'map(select(.mergedAt == null))'
```

Use **`--repo owner/repo`** when not using default **`gh`** context.

**Merged only** (housekeeping/reporting where merged history is needed):

```bash
gh pr list --state merged --limit 200 --json number,title,state,mergedAt,url
```

## View (single PR)

By number:

```bash
gh pr view 42 --json title,body,state,url,headRefName,baseRefName
```

Body-focused JSON (house-style hint for **`@gh-pr`**):

```bash
gh pr view 42 --json title,body
```

**GraphQL id** (for GraphQL APIs that accept pull request node ids):

```bash
gh pr view <N> --json id,number,title,state
```

Optional diff stat:

```bash
gh pr diff 42 --stat
```

## Example — PR from current branch

**Input:** “Do I already have a PR from this branch?”

```bash
BRANCH=$(git branch --show-current)
gh pr list --head "$BRANCH" --base main --state open --json number,title,url
```

**Output (sample JSON line):** `{"number":7,"title":"Add issue label playbook","url":"https://github.com/org/repo/pull/7"}`

## Example — view before edit

**Input:** “Show PR 7 title and base.”

```bash
gh pr view 7 --json number,title,baseRefName,headRefName,state
```

## See also

- **[`internal-write-gh-pr-commands`](../../write/gh/pr-commands/SKILL.md)** — **`gh pr create`**, **`gh pr edit`**, and **`gh pr close`**.
- **[`internal-read-gh-repo-stream`](../repo-stream/SKILL.md)** — **`--repo`**, head/base for **`@gh-pr`**.
- [`@gh-pr-list`](../../../gh/pr/list/SKILL.md) · [`@gh-pr-view`](../../../gh/pr/view/SKILL.md)
- **[`list-view-hub/SKILL.md`](./list-view-hub/SKILL.md)** — narrative hub (no duplicate fences).
- [`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md) · [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
