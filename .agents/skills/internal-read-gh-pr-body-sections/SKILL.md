---
name: internal-read-gh-pr-body-sections
description: >-
  Read-only: PR title/body markdown patterns and canonical section structure. Caller: internal-read-gh-pr-description.
  Does not run gh or git.
---

# Internal: Pull request body sections (`internal-read-gh-pr-body-sections`)

**Read-only library.** **Static** PR **substance**: section headings, canonical body rules, example skeleton, title rules. Full copy-paste reference: **[`section-patterns/SKILL.md`](./section-patterns/SKILL.md)**.

**[`internal-read-gh-pr-description`](../pr-description/SKILL.md)** owns **template discovery** (**§5** — GitHub API via **`internal-read-gh-repo-forms-json`**, local files), **`git` delta** (**§6**), **API-first linking + issue hint scan** (**§6.5–6.6**), **title/body + triple pass** (**§7**), and hand-off to **`@gh-pr`** §9. **Do not** duplicate **§5** **`gh repo view`** lines here.

---

## Do

- When **`$PR_TEMPLATE_SOURCE`** is **`canonical`**, draft the body from **`section-patterns/SKILL.md`** only.
- When merging a repo or GitHub template, **map** content into those headings while keeping **TL;DR** first.

## Do not

- Run **`gh pr create`**, **`gh pr edit`**, or **AskQuestion** for PR mutation — parent **`@gh-pr`** §9.

---

## See also

- [`internal-read-gh-pr-content`](../pr-content/SKILL.md) — title/body shape (**`title-line`**, **`pr-body-skeleton`**)
- [`internal-read-gh-pr-description`](../pr-description/SKILL.md) — discovery, §6 delta, §6.5–7 linking + title/body
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md) — template pick when multiple templates
- [`@gh-pr`](../../../gh/pr/SKILL.md)
