---
name: internal-read-gh-pr-content-pr-orchestration
description: >-
  Read-only: @gh-pr orchestration prose. Parent internal-read-gh-pr-content.
---

# PR orchestration (`@gh-pr`)

**Fixed order:** **`@git-pull`** → **`@git-review`** → **`@git-push`** → list matching open PRs / resolve vars → **AskQuestion intent gate** (summary in prompt; abort/proceed/refine) → resolve `PR_NUM` → **`internal-read-gh-pr-description`** (full: §5 templates, §6 delta, **§6.5–6.6** API linking ladder + issue hint scan, **§7** title/body + triple pass) → **AskQuestion mutation gate** (summary must include **linking ladder** choice: project **`item-add`**, minimal body keywords, or UI-only, plus candidate issues) → **`gh pr create` / `gh pr edit`** → optional **`gh project item-add`** + optional **`gh pr view --json closingIssuesReferences`** per **`internal-write-gh-pr-commands`** when confirmed in that gate.

**Inventory (optional):** run **`@gh-pr-list`** / **`@gh-pr-view`** using [`internal-read-gh-pr-list`](../pr-list/SKILL.md) (and [list/view hub](../pr-list/list-view-hub/SKILL.md)) when you need open PR numbers, URLs, or a quick **`gh pr diff --stat`** before drafting with **`@gh-pr`**.

**Variables:** set **`BASE_GIT`**, **`PR_TARGET_REPO`**, etc. per [`internal-read-gh-repo-stream`](../repo-stream/SKILL.md) — do not re-derive fork rules inline.

**After publish/recovery:** `@git-push` may complete on a different branch after failure triage (for example create/switch recovery branch). Re-apply [`internal-read-gh-repo-stream`](../repo-stream/SKILL.md) before PR lookup/drafting so head/base vars reflect the current branch.

**Intent gate before drafting:** after lookup, if matches exist, ask user whether to edit one, create new, or abort. Keep the compact inventory summary inside AskQuestion prompt text and follow [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md) for safe-first options and single refinement outlet.

**Do not** run `gh pr create` before pull, review, and push succeed. **Do not** inline `npm test` / full **`@git-review`** instead of the ordered **`@git-review`** hand-off in **`@gh-pr`**—**`@git-push`** does **not** own verify.

**PR text:** [PR body skeleton](../pr-body-skeleton/SKILL.md), [title line](../title-line/SKILL.md). **Issue/project linkage:** **[`internal-read-gh-pr-description`](../pr-description/SKILL.md)** §6.5–6.6 + **`internal-write-gh-pr-commands`** (optional **`gh project item-add`**, verification JSON). **Delta git commands:** **`internal-read-git-git-diff-summary`** only (invoked from **`internal-read-gh-pr-description`**). Final create/edit still requires a separate mutation AskQuestion with summary in prompt text before the write.
