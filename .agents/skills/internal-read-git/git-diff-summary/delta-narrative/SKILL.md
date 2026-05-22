---
name: internal-read-git-git-diff-summary-delta-narrative
description: >-
  Read-only: git log/diff recipes and PR/commit narrative shape; optional full commit-body log for issue keywords.
  Parent internal-read-git-git-diff-summary.
---

# Git diff summary (commit message or PR prep)

Reusable shape for summarizing **`git diff`** or **`$BASE_GIT..HEAD`** before writing a **commit message** or **PR body**. **Normative PR delta commands** live here; **`internal-read-git-git-diff-summary`** owns when and how agents use the evidence.

---

## Steps (human or agent)

1. **Inventory** — List changed **areas** (top-level dirs or subsystems), not every file, unless the branch is tiny. Run **`git diff --name-status "$BASE_GIT...HEAD"`** and group by concern.
2. **Themes** — From **`git log --oneline "$BASE_GIT..HEAD"`**, group into **2–5 bullets** (what readers must know).
3. **Risk** — One line: **merge risk**, **behavior change**, or **none**.
4. **Omit** — Noise renames, generated lockfile-only churn unless that *is* the story.

---

## Shell recipes (PR delta)

Use **`$BASE_GIT`** and **`HEAD`**. **Two-dot** for commits, **three-dot** for file list.

```bash
git log --oneline "$BASE_GIT..HEAD"
```

```bash
git diff --name-status "$BASE_GIT...HEAD"
```

Optional:

```bash
git merge-base --is-ancestor "$BASE_GIT" HEAD
git log --reverse --oneline "$BASE_GIT..HEAD"
```

### Issue keywords in commit range (PR / `@gh-pr`)

For **`internal-read-gh-pr-description`** §6.6, scan **full** commit messages on the branch (not only **`git log -1`**) for **`Fixes #n`**, **`Refs #n`**, **`Closes #n`**, **`Resolves #n`**, and bare **`#nnn`** mentions. Use a **two-dot** range so merges behave as expected for your branch topology.

```bash
git log --pretty=format:'---%n%s%n%n%b' "$BASE_GIT..HEAD"
```

When the log is long, you may **cap** output for chat (for example **`--max-count=30`**) but **state the cap** if older commits were omitted.

---

## Commit message skeleton (short)

```
<area>: <imperative outcome>

- <bullet tied to diff>
- <bullet>

Refs: #n
```

Use **50–72** char first line when possible; body wraps at ~72.

---

## Hand-off

- **PR / issue title:** [`title-line`](../../gh/pr-content/title-line/SKILL.md).
- **PR body:** [`pr-body-skeleton`](../../gh/pr-content/pr-body-skeleton/SKILL.md).
- **Orchestration:** [`internal-read-git-git-diff-summary`](../SKILL.md), [`internal-read-gh-pr-description`](../../gh/pr-description/SKILL.md).
