---
name: internal-read-gh-pr-body-sections-section-patterns
description: >-
  Read-only: canonical PR body sections and examples. Parent internal-read-gh-pr-body-sections. No default Linked issues
  prose; optional minimal closing-keyword line only when @gh-pr §9 chose the keyword path.
---

# Canonical internal scaffold

This file is the canonical internal reference for PR section patterns.

## Issue and project linkage (with `@gh-pr`)

- **Do not** add a standing **“Linked issues”** markdown section by default. Candidate issues and the **linking ladder** ( **`gh project item-add`**, minimal **`Refs`/`Fixes`/`Closes`** line, or **UI-only**) are confirmed in **`@gh-pr`** **§9** per **[`internal-read-gh-pr-description`](../../pr-description/SKILL.md)** §6.5–7.
- **Minimal keyword line:** GitHub’s documented programmatic path for merge-time **closing** inference is **closing keywords in the PR body** (see **`internal-write-gh-pr-commands`**). At most **one** short trailing line when the user explicitly chose that path—**no** multi-paragraph issue prose in the PR description for traceability alone.
