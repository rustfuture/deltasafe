---
name: internal-write-gh-pr-commands
description: >-
  Normative gh pr create/edit/close shapes; optional gh project item-add for PR/issue URLs; read-only gh pr view JSON
  for closingIssuesReferences. PR list/view lives in internal-read-gh-pr-list. Callers run structured Q&A before mutations.
---

# Internal: GitHub pull request mutations (`internal-write-gh-pr-commands`)

**Write library (GitHub / `gh` / API).** **`gh pr create`**, **`gh pr edit`**, and **`gh pr close`** shapes consumed by **`@gh-pr`** and **`@gh-pr-close`**. **Mutations** require **Goal + AskQuestion + Proceed** in the **public** skill via **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)**.
**Master-confirm rule:** **`SKIP_QA_*`** and **`SKIP_QA_WRITE`** do not bypass session-level confirm for any GitHub mutation in this library.

**List / view / diff stat** — **[`internal-read-gh-pr-list`](../../read/gh/pr-list/SKILL.md)** (sole read-only **`gh pr list`** / **`gh pr view`** / **`gh pr diff --stat`** fences).

**`gh repo view`** for **PR templates** is **not** here—use **[`internal-read-gh-repo-forms-json`](../../read/gh/repo-forms-json/SKILL.md)**.

## Platform note (read before Proceed)

- GitHub does not provide issue-parity hard-delete behavior for pull requests in common `gh` setups.
- PR archive APIs are admin-scoped and are not exposed as a packaged bulk mutation skill in this pack.
- For cleanup policy work on closed PRs, use GitHub UI and repository/org governance outside this library.
- **PR ↔ issue (Development / closing semantics):** There is **no** official `gh` flag to set the web **Development** sidebar link without **closing keywords in the PR body** or **manual UI**; platform gap is tracked as [`cli/cli#11405`](https://github.com/cli/cli/issues/11405). **`gh pr view --json closingIssuesReferences`** is **read-only** and reflects what GitHub already inferred (often from body keywords). See **[`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md)** §6.5–6.6 and [GitHub docs: linking PRs to issues](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue).

## Create PR (after Proceed)

**Only for** **`@gh-pr`** **§9**, after the final structured **Proceed** there. Resolve `--repo`, `--base`, and `--head` through **`internal-read-gh-repo-stream`**. When the **§9** summary chose the **keyword path**, put **`Refs #n` / `Fixes #n` / `Closes #n`** in the PR body text (or body file)—there are **no** separate `gh pr create` flags for that linkage. Otherwise omit keyword lines and use **Projects** and/or UI per **[`internal-read-gh-issue-projects-relationships`](../../read/gh/issue-projects-relationships/SKILL.md)**.

```bash
gh pr create --repo owner/repo --base main --head feature-branch --title "Title" --body-file /path/to/body.md
```

## Edit PR (after Proceed)

**Only for** **`@gh-pr`** **§9**, after the final structured **Proceed** there. Editing replaces title/body with regenerated content; keep issue keyword lines in the body **only when** the user confirmed that path (same rules as create).

```bash
gh pr edit <N> --repo owner/repo --title "Title" --body-file /path/to/body.md
```

## Optional: add PR and issues to a project (same Proceed batch as `@gh-pr` §9)

When **`@gh-pr`** **§9** confirmed **project attach**, run **`gh project item-add`** once per URL **after** the PR number is known (create path: parse **`gh pr create`** output or run **`gh pr view --json url,number`** for the new PR). Use the **destination** repo URL shape for both PR and issues: **`https://github.com/OWNER/REPO/pull/<N>`** and **`https://github.com/OWNER/REPO/issues/<N>`** where **`OWNER/REPO`** matches **`PR_TARGET_REPO`** from **`internal-read-gh-repo-stream`**. Discovery, **`project`** token scope, and owner disambiguation: **[`internal-read-gh-issue-projects-relationships`](../../read/gh/issue-projects-relationships/SKILL.md)** + **[`internal-read-gh-project-list`](../../read/gh/project-list/SKILL.md)**.

```bash
gh project item-add <PROJECT_NUMBER> --owner OWNER_LOGIN --url https://github.com/owner/repo/pull/<PR_NUMBER>
```

```bash
gh project item-add <PROJECT_NUMBER> --owner OWNER_LOGIN --url https://github.com/owner/repo/issues/<ISSUE_NUMBER>
```

**Fork:** **`owner/repo`** in URLs is the **upstream** destination (same as **`PR_TARGET_REPO`**), not the fork’s **`github.com/FORK_OWNER/…`** issue namespace unless the issue genuinely lives on the fork.

## Read-only: verify linked issues metadata (after create/edit)

```bash
gh pr view <N> --repo owner/repo --json closingIssuesReferences,number,url,title
```

## Close PR (after Proceed)

**Only for** **`@gh-pr-close`**, after **Goal** + **AskQuestion** + **Proceed**.

```bash
gh pr close <N>
```

Use **`--repo owner/upstream`** when closing against the fork’s upstream repo. Add **`--comment "…"`** when the user supplied a closing note.

## See also

- **[`internal-read-gh-pr-list`](../../read/gh/pr-list/SKILL.md)** — read-only **`gh pr list`** / **`view`** / **`diff --stat`**.
- **[`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md)** — §6.5–7 linking + body rules.
- **[`internal-read-gh-issue-projects-relationships`](../../read/gh/issue-projects-relationships/SKILL.md)** — project discovery + PR/issue **`item-add`** playbook.
- **[`internal-write-gh-issue-commands`](../issue-commands/SKILL.md)** — **`gh issue`** view/create/edit/close/delete; issue **`item-add`** fallback fences.
- **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)** — **`--repo`** targets for forks/upstream.
- [`@gh-pr-close`](../../../gh/pr/close/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
- [`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)
