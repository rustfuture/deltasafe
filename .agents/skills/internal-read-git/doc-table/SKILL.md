---
name: internal-read-git-doc-table
description: >-
  Read-only: markdown table conventions for doc reference sections (not indexes). Callers: @git-docs,
  internal-write-gh-documentation. Does not write files.
---

# Internal: Doc table rules (`internal-read-git-doc-table`)

**Read-only library.** **When** to use **markdown tables** on **`docs/`** pages: **reference** and **comparison** sections only—not **`## Contents`**.

**Default for agents** — Whenever you touch a **markdown table** in **reference** material (including root **`README.md`** and **`docs/**`** in **any** Cursor project), apply **these rules silently** as part of **`@git-docs`**, **`internal-write-gh-documentation`**, or ad-hoc doc polish—the user does **not** need to name **`internal-read-git-doc-table`** in chat.

---

## Rules

1. **Sort** rows by **first column** lexicographically.
2. After the table, **call out** the **largest** row by the chosen metric and **why** it matters.
3. Optional **summary** prose for aggregates—avoid double-counting with child pages.

## See also

- [`internal-read-git-doc-index`](../doc-index/SKILL.md)
- [`internal-read-git-doc-explanation`](../doc-explanation/SKILL.md)
