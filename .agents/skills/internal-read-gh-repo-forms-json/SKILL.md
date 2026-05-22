---
name: internal-read-gh-repo-forms-json
description: >-
  Read-only: sole normative gh repo view --json shapes for issueTemplates and pullRequestTemplates. Callers own
  JSON parsing, merge rules, and AskQuestion. Does not replace internal-read-gh-repo-stream for fork/same-repo refs.
---

# Internal: `gh repo view` forms JSON (`internal-read-gh-repo-forms-json`)

**Read-only library.** **Single owner** of the **`gh repo view … --json …`** invocations used to load **GitHub-hosted** issue and pull-request templates from the API. **Callers** (**[`internal-read-gh-pr-description`](../pr-description/SKILL.md)** §5 Step A, **[`internal-read-gh-issue-description`](../issue-description/SKILL.md)** §1 Step A) **run** these lines in the shell and **parse** output in their own sections—**do not** paste duplicate **`gh`** recipes into other **`SKILL.md`** files or **`docs/**`**.

**Prerequisites:** **`$UPSTREAM`**, **`PR_TARGET_REPO`**, and same-repo vs fork semantics live in **[`internal-read-gh-repo-stream`](../repo-stream/SKILL.md)** §3—this file does **not** repeat that table.

---

## Pull request templates (`pullRequestTemplates`)

Pick the **host** the same way **`internal-read-gh-pr-description`** §5 expects (same-repo → current **`gh`** context; fork → upstream **`owner/repo`** string):

| Case | Command |
| --- | --- |
| **Same-repo** | `gh repo view --json pullRequestTemplates` |
| **Fork** | `gh repo view "$UPSTREAM" --json pullRequestTemplates` |

On GitHub Enterprise, set **`GH_HOST`** (or your **`gh`** host config) so the command hits the right server.

**Optional length probe** (still read **full** template entries when length **> 0**):

```bash
gh repo view … --json pullRequestTemplates --jq '(.pullRequestTemplates | length)'
```

Use **`…`** as **`"$UPSTREAM"`** or omit for default context—**consistent** with the table row you chose.

---

## Issue templates (`issueTemplates`)

| Case | Command |
| --- | --- |
| **Default** (templates for the **current** **`gh`** repository context—typical when filing on the checkout) | `gh repo view --json issueTemplates` |
| **Upstream host** (fork workflow: load upstream’s template set—same **`$UPSTREAM`** string as the PR table) | `gh repo view "$UPSTREAM" --json issueTemplates` |

**Optional length probe:**

```bash
gh repo view … --json issueTemplates --jq '(.issueTemplates | length)'
```

---

## Combined fetch (same host only)

When both JSON keys are needed in **one** round trip for the **same** host:

```bash
gh repo view … --json issueTemplates,pullRequestTemplates
```

**Callers** still split **PR-only** vs **issue-only** parsing and merge rules—this file supplies **commands** only.

---

## Do not

- Copy these **`bash`** fences into **`internal-read-gh-pr-description`**, **`internal-read-gh-issue-description`**, or **`docs/**`**—link here.
- Re-document **`$UPSTREAM`** / **`PR_TARGET_REPO`**—use **`internal-read-gh-repo-stream`**.

## See also

- **[`internal-read-gh-repo-stream`](../repo-stream/SKILL.md)** — variables for **`gh repo view`**
- **[`internal-read-gh-pr-description`](../pr-description/SKILL.md)** — PR template body parsing, §6 delta, §6.5–6.6 linking ladder + issue scan, §7 title/body
- **[`internal-read-gh-issue-description`](../issue-description/SKILL.md)** — issue template parsing and reshape rules
