---
name: internal-read-git-git-diff-summary
description: >-
  Read-only: local git diff/log narrative for PR evidence (no GitHub API). PR delta evidence contract and narrative
  clustering for What changed. Command shapes live in delta-narrative/SKILL.md.
  Caller: internal-read-gh-pr-description §6.
---

# Internal: Git diff summary (`internal-read-git-git-diff-summary`)

**Read-only library.** **Local `git` only** (no **`gh`** / GitHub REST). **Sole owner** of the **evidence contract** and **inventory discipline** for computing the merge-aware delta from destination to **`HEAD`** for PR drafting. **Command and narrative shape:** **[`delta-narrative/SKILL.md`](./delta-narrative/SKILL.md)**.

**Consumers:** **`internal-read-gh-pr-description`** §6 (run this skill **in full** before §7); humans writing commits; **`@gh-pr`** orchestration.

---

## Do

1. **Inventory** by **theme** / subsystem, not raw file list (unless tiny branch).
2. Use **[`delta-narrative/SKILL.md`](./delta-narrative/SKILL.md)** for the delta commands after **`@git-pull`** / **`@git-push`** in the PR flow.
3. **Reuse** the same summary discipline for **commit** and **PR** to avoid drift.
4. Point agents to **`delta-narrative/SKILL.md`** for copy-paste structure and short commit messages.

## Do not

- Run PR mutations.
- Duplicate delta evidence steps inside **`internal-read-gh-pr-description`** — that skill **delegates** §6 here.

## Evidence Contract

Use the destination and **`HEAD`** values from **`@gh-pr`** / **`internal-read-gh-repo-stream`**. Output is for **drafting accuracy** and **triple pass** in **`internal-read-gh-pr-description`** §7—**do not** paste base/head identifiers, commit counts, or compare metadata into the PR body.

**Full-branch coverage (required):** The PR description must reflect the **entire** meaningful delta from destination to **`HEAD`**.

1. **Area checklist** — Build from the merge-aware file list and group paths by top-level directory / concern (e.g. **`skills/`**, **`docs/`**, **`.github/`**, **`README.md`**).
2. **Commit themes** — Cluster the branch history across the whole branch.
3. **Optional sanity checks** — Use the template when the graph is messy or the range looks surprising.
4. **Destination and head pins** — Keep these internal for accuracy; do not post them in the PR body.

**Short history** (roughly **≤ ~12** commits, one clear theme): you may **weave** commit themes into narrative bullets without pasting the log.

**Long or multi-era history:** **do not** dump the raw log. **Summarize** older phases in **1–3 tight bullets** each.

## Template

- **[`delta-narrative/SKILL.md`](./delta-narrative/SKILL.md)** — delta commands, command range notes, and commit/PR narrative shape.

## See also

- [`internal-read-gh-pr-content`](../gh/pr-content/SKILL.md)
- [`internal-read-gh-pr-description`](../gh/pr-description/SKILL.md)
- [`delta-narrative/SKILL.md`](./delta-narrative/SKILL.md)
- [`title-line`](../gh/pr-content/title-line/SKILL.md)
