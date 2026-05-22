---
name: internal-read-gh-issue-projects-relationships
description: >-
  Read-only: discover repo-scoped GitHub Projects for issue and PR mutation, normative gh shapes for attaching issues and
  pull requests to projects, and issue relationship playbook (body links, duplicates, parent/sub-issues). Callers add
  --repo and owner flags per target; mutations live in internal-write-gh-issue-commands and internal-write-gh-pr-commands.
---

# Internal: Issue projects + relationships (`internal-read-gh-issue-projects-relationships`)

**Read-only.** Consumed by **`@gh-issues`** (plan + mutation summary), **`@gh-pr`** (via **`internal-read-gh-pr-description`** §6.5–6.6), **[`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)** (normative attach fences), and **[`internal-write-gh-pr-commands`](../../../write/gh/pr-commands/SKILL.md)** (PR **`item-add`**). **No `gh` mutations** in this file.

**Project list shapes** — **[`internal-read-gh-project-list`](../project-list/SKILL.md)** (auth + `gh project list` bases).

**Fork / PR target repo** — resolve **`owner/repo`** for issues the same way **`@gh-issues`** resolves the mutation target (see **[`internal-read-gh-repo-stream`](../repo-stream/SKILL.md)** when the issue is filed on **`$PR_TARGET_REPO`** rather than the fork root). **Project discovery `--owner`** must match the **login that owns the repository** where the issue is created (first segment of **`nameWithOwner`** for that repo).

**Pull requests (`@gh-pr`)** — use the same **`PR_TARGET_REPO`** for **`https://github.com/OWNER/REPO/pull/<N>`** URLs passed to **`gh project item-add --url`**. Run **`item-add` for the PR** after **`gh pr create`** (or **`gh pr edit`**) returns a PR number; then **`item-add` per confirmed issue** URL on that same **`OWNER/REPO`**. Do not substitute the fork’s **`github.com/FORK_OWNER/repo`** host in URLs when issues and PR are tracked on the **upstream** destination unless the issue genuinely lives on the fork.

---

## 1. Auth and capability probes

**`gh issue create`**, **`gh issue edit --add-project`**, **`gh project list`**, and **`gh project item-add`** need the **`project`** scope for project features. **`gh auth status`** shows current scopes.

If a command fails with missing **`project`** (or GraphQL mentions **`read:project`** / **`project`** scope), refresh:

```bash
gh auth refresh -s project
```

**GitHub Enterprise Server** may lack Projects v2 features or ship older **`gh`** flags. When **`gh issue create --help`** does not list **`--project`**, or **`gh project list`** errors with an unsupported API message, **stop** automated attach, record the error in chat, and fall back to **web UI** (Issues + Projects) or an operator-run attach.

---

## 2. Discover projects for the issue target repo

1. Resolve **`TARGET_REPO`** — `owner/repo` passed to **`gh issue create` / `gh issue edit`** (default: current **`gh`** context; else **`--repo owner/repo`**).
2. Load owner login:

```bash
gh repo view --json nameWithOwner --repo owner/repo
```

Use **`owner.login`** from **`gh repo view --json owner,nameWithOwner --repo owner/repo`** when you need the owner object explicitly.

3. List candidate projects for that owner (open projects first; add **`--closed`** only when grooming closed boards):

```bash
gh project list --owner OWNER_LOGIN --limit 50 --format json
```

4. **Choose a project title** for **`--project` / `--add-project`**:
   - **One unambiguous title** in the list → use it in the mutation summary and in **[`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)** fences.
   - **Several titles match** the user’s intent (or duplicate titles) → **structured AskQuestion** in **`@gh-issues`**: pick one project, **skip** attach for this run, or **abort** until clarified.
   - **No projects** or **`gh project list` fails** after auth → **skip** project attach; state **why** (no boards, scope, or GHE limitation)—not a silent success.

The CLI does not always expose “this repo’s linked project” as a single field without GraphQL. Treat **owner-scoped listing + user disambiguation** as the supported pack workflow unless the user supplies an exact **`--project "Title"`** override.

---

## 3. Relationship playbook (issues)

### 3.1 Cross-links in the body

Use **`#123`** (same repo) or full **`https://github.com/owner/repo/issues/N`** (cross-repo) for **human-readable** context, release notes, and search. Body links **do not** replace GitHub duplicate closure or project membership—they document intent only.

### 3.2 Duplicate vs “mentioned in body”

To close a duplicate with correct GitHub semantics, use **`@gh-issue-close`** and **[`close-as-duplicate`](../../../write/gh/issue-commands/close-as-duplicate/SKILL.md)**: comment pointing at the canonical issue, then **`gh issue close`**. Do not rely on body text alone to mark duplicates.

### 3.3 Parent / sub-issue / “blocked by”

Product support (sub-issues, dependency graphs, **blocked by**) varies by **github.com** vs **GHES** and ship channel. If **`gh`** has no stable sub-issue flags for your version, use **web UI** or API where your token allows, and keep a **body fallback** (“Parent: #12”, “Blocked by: #34”) so agents stay consistent when automation is unavailable.

### 3.4 Interaction with **`@gh-issue-close`**

Relationship edits that **close** or **reopen** issues stay in **`@gh-issue-close`** and **`internal-write-gh-issue-commands`** close paths. **`@gh-issues`** owns **create/edit** content and **additive** project attach—not bulk close/reopen.

---

## 4. Pull requests and projects (`@gh-pr`)

**Goal:** Put the **PR** and any **confirmed issues** on the same GitHub Project board using supported **`gh project item-add`** calls (see **[`internal-write-gh-pr-commands`](../../../write/gh/pr-commands/SKILL.md)**).

1. Resolve **`PR_TARGET_REPO`** as **`owner/repo`** on the **destination** (upstream when **`STREAM_MODE=fork`**) per **`internal-read-gh-repo-stream`** §3.
2. Resolve **`PROJECT_NUMBER`** and **`OWNER_LOGIN`** the same way as §2 of this file (**`gh project list`**, user override, or structured pick).
3. After the PR exists, add the PR, then each issue:

```text
https://github.com/OWNER/REPO/pull/<PR_NUMBER>
https://github.com/OWNER/REPO/issues/<ISSUE_NUMBER>
```

4. **Auth:** same **`project`** scope and GHES caveats as §1.

**Do not** confuse this with the PR **Development** sidebar link—**`item-add`** is **project membership**, not a substitute for closing keywords when merge-time close semantics are required (see §6.5 of **`internal-read-gh-pr-description`**).

---

## See also

- **[`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)** — create, edit, project attach, close.
- **[`internal-write-gh-pr-commands`](../../../write/gh/pr-commands/SKILL.md)** — PR create/edit + optional **`item-add`** for PR URLs.
- **[`internal-read-gh-pr-description`](../pr-description/SKILL.md)** — §6.5–6.6 linking ladder + issue hint scan.
- **[`internal-read-gh-project-list`](../project-list/SKILL.md)** — `gh project list` inventory.
- **[`internal-read-gh-repo-stream`](../repo-stream/SKILL.md)** — **`$PR_TARGET_REPO`** / fork vs same-repo.
- **[`@gh-issues`](../../../gh/issues/SKILL.md)** · **[`@gh-issue-close`](../../../gh/issues/close/SKILL.md)** · **[`@gh-pr`](../../../gh/pr/SKILL.md)**
