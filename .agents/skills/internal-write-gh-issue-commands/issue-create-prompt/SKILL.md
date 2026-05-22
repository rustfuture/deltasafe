---
name: internal-write-gh-issue-commands-issue-create-prompt
description: >-
  Read-only: gh issue create checklist and body-file patterns. Parent internal-write-gh-issue-commands.
---

# Issue create prompt

**Before mutation:** Goal + **AskQuestion** + Proceed ([`internal-write-plan-structured-qa`](../../../plan/structured-qa/SKILL.md)).

**Steps**

1. Run [`internal-read-gh-issue-dedupe`](../../../../read/gh/issue-dedupe/SKILL.md) with candidate title + body summary (read-only).
2. Draft **title** from [title line](../../../../read/gh/pr-content/title-line/SKILL.md); **body** from [issue body skeleton](../../../../read/gh/issue-description/issue-body-skeleton/SKILL.md) (and template merge from **`internal-read-gh-issue-description`**).
3. **Labels (default path):** run [`internal-read-gh-issue-labels`](../../../../read/gh/issue-labels/SKILL.md) after title/body drafting. Confirm **`gh label create`** separately when needed ([`internal-write-plan-skill-safety`](../../../plan/skill-safety/SKILL.md)). On Proceed, pass **`--label "a,b"`** to **`gh issue create`** (or **`--add-label` / `--remove-label`** on **`gh issue edit`**) when the user accepts label changes; skip labels only on explicit decline.
4. **Projects (default path for current-repo ship):** run [`internal-read-gh-issue-projects-relationships`](../../../../read/gh/issue-projects-relationships/SKILL.md) after the target **`owner/repo`** is known. Include **`--project "…"`** on **`gh issue create`** or **`--add-project "…"`** on **`gh issue edit`** in the mutation summary when there is a single unambiguous board title (or a user-supplied title); use **`gh project item-add`** from [`internal-write-gh-issue-commands`](../SKILL.md) only as a documented fallback. If discovery fails, scope is missing, or titles collide, record **skip** + reason in chat—never treat a skipped attach as success without saying so.
5. Prefer body files in `.cursor/gh/issues/` for batch creation (for example `.cursor/gh/issues/<slug>.md`).
6. On Proceed: run the agreed **`gh issue create`** / **`gh issue edit`** line from [`internal-write-gh-issue-commands`](../SKILL.md) (labels + project flags as accepted); never skip dedupe when the user asked for a **new** tracked issue.
