---
name: internal-read-gh-issue-list
description: >-
  Normative gh issue list and gh search issues inventory shapes. Read-only. Callers add --state, --label, --limit, --json,
  and --repo flags per goal; mutation lives in internal-write-gh-issue-commands. Used by internal-read-gh-pr-description §6.6.
---

# Internal: GitHub issue list (`internal-read-gh-issue-list`)

**Library.** **Sole** normative **`gh issue list`** and **`gh search issues`** fences for this pack. Consumed by **`@gh-issue-list`**, **`@gh-issues`** (inventory), **`@gh-issue-pick`**, **`@gh-issue-delete-closed`** (preview), **`@gh-issue-review`** (peer list), **`internal-read-gh-issue-dedupe`**, **`internal-read-gh-pr-description`** §6.6 (optional issue validation during **`@gh-pr`**), and **[`DEDUPE_CHECKLIST.md`](../issue-dedupe/DEDUPE_CHECKLIST.md)**.

**Mutations** (**`gh issue create`/`edit`/`close`/`delete`**, …) live in **[`internal-write-gh-issue-commands`](../../write/gh/issue-commands/SKILL.md)**—never run those from this file.

## Cadence (read-only)

**Faster in inventory-only flows** — This library **only** hits GitHub for **read** list/search. **Parents** that are read-only (**`@gh-issue-list`**, **`@gh-issue-review`** peer pass, …) may use it **without** an extra **AskQuestion** from this file. **Do not** infer that **Proceed** is satisfied for a later **`gh issue`** **mutation**—that lives in **`internal-write-gh-issue-commands`** behind the right **`@gh-*`** skill.

## Online changes and paths outside the repo

- **This file is read-only** — listing/searching issues on GitHub does **not** replace **Goal + structured confirm** for any follow-on **`gh`** write.
- **Inventory stays lightweight** — no mandatory **Abort / Review / Proceed** stack for routine reads; mutation parents own **Proceed** per **`internal-write-plan-skill-safety`** / **`internal-write-plan-structured-qa`** **§1e** before writes.
- **Parent skills** that call **`internal-read-gh-issue-list`** and then **mutate** GitHub or **write outside the git workspace root** must apply **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** before those writes.
- **Master-confirm rule** — even with **`SKIP_QA_*`** / **`SKIP_QA_WRITE`**, follow-on high-risk writes still require session-level confirm in the owning mutation skill.

## Caller refinement (flags)

Use the fenced shapes below as **bases**. **Callers** add flags for the task at hand, for example:

- **`--state open`** | **`closed`** | **`all`**
- **`--label "name"`** (repeat **`--label`** if **`gh`** supports it for your version)
- **`--limit N`** (cap chat output)
- **`--json field1,field2,…`** (machine-readable inventory)
- **`--repo owner/repo`** when not using default **`gh`** context (**fork/upstream:** **[`internal-read-gh-repo-stream`](../repo-stream/SKILL.md)**)

## List / search (typical open inventory)

```bash
gh issue list --state open --limit 30
```

**Filter by label:**

```bash
gh issue list --state open --label "bug" --limit 30
```

**Search (broader)** — pass explicit **`owner/repo`** when not using default context:

```bash
gh search issues --repo owner/repo --state open --limit 30 "QUERY"
```

## List closed (inventory before bulk delete)

```bash
gh issue list --state closed --limit 200 --json number,title,state
```

## See also

- **[`internal-write-gh-issue-commands`](../issue-commands/SKILL.md)** — **`gh issue view`**, create, edit, close, delete.
- **[`internal-read-gh-repo-stream`](../repo-stream/SKILL.md)** — **`--repo`** targets.
- [`@gh-issue-list`](../../../gh/issues/list/SKILL.md) · [`@gh-issue-delete-closed`](../../../gh/issues/delete-closed/SKILL.md)
- [`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md) · [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
