---
name: internal-read-gh-pr-content
description: >-
  Read-only: canonical PR title + body shape; child skills title-line/ and pr-body-skeleton/. Orchestration stays in
  internal-read-gh-pr-description + internal-read-gh-pr-body-sections. Callers: @gh-pr. Does not write files.
---

# Internal: PR content shape (`internal-read-gh-pr-content`)

**Read-only library.** **What** the **title** and **body** should look like. **Child skills:**

- **[`title-line/SKILL.md`](./title-line/SKILL.md)** — PR and issue **titles**
- **[`pr-body-skeleton/SKILL.md`](./pr-body-skeleton/SKILL.md)** — PR body markdown shape

**Template discovery:** **[`internal-read-gh-pr-description`](../pr-description/SKILL.md)** §5 (orchestration + local globs); **`gh repo view --json`** lines: **[`internal-read-gh-repo-forms-json`](../repo-forms-json/SKILL.md)**. **`git` delta commands:** **[`internal-read-git-git-diff-summary`](../../git/git-diff-summary/SKILL.md)** (§6 hand-off). **API-first linking + issue hint scan:** **`internal-read-gh-pr-description`** §6.5–6.6. **Full section prose:** **[`internal-read-gh-pr-body-sections/section-patterns/SKILL.md`](../pr-body-sections/section-patterns/SKILL.md)**.

---

## Ownership

| Topic | Owner |
| --- | --- |
| Title line rules + body section order | **This skill** + **[`title-line/SKILL.md`](./title-line/SKILL.md)** + **`pr-body-sections/section-patterns`** |
| **`gh repo view --json`** (templates API) | **`internal-read-gh-repo-forms-json`** |
| Local template globs + merge / reshape | **`internal-read-gh-pr-description`** §5 |
| **`git log` / `git diff`** vs **`$BASE_GIT`** | **`internal-read-git-git-diff-summary`** (§6 of **`internal-read-gh-pr-description`**) |
| API-first linking ladder + issue hint scan (read-only) | **`internal-read-gh-pr-description`** §6.5–6.6 |
| Title/body prose assembly + triple pass | **`internal-read-gh-pr-description`** §7 |
| Example markdown shape only | **[`pr-body-skeleton/SKILL.md`](./pr-body-skeleton/SKILL.md)** |

## See also

- [`internal-read-git-git-diff-summary`](../../git/git-diff-summary/SKILL.md)
- [`@gh-pr`](../../../gh/pr/SKILL.md)
