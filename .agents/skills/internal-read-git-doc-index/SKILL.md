---
name: internal-read-git-doc-index
description: >-
  Read-only: rules for doc page indexes (numbered lists, lexicographic order, no tables in Contents). Callers:
  @git-docs, internal-write-gh-documentation. Does not write files.
---

# Internal: Doc index rules (`internal-read-git-doc-index`)

**Read-only library.** **Normative rules** for **navigation** sections on **`docs/**`** pages.

**Consumers:** **`@git-docs`**, **`internal-write-gh-documentation`**, **`internal-read-git-readme-tree`** (bootstrap text).

**Default for agents** — When you add or fix **`## Contents`** (or similar nav blocks) in **`README.md`** / **`docs/**`** in **any** project, apply **these rules silently** as part of **`@git-docs`** or **`internal-write-gh-documentation`**—the user does **not** need to name **`internal-read-git-doc-index`** in chat.

---

## Rules

1. **`## Contents`** (and **See also** when it is pure navigation): **numbered nested lists** only.
2. **Lexicographic** ordering of children (stable convention per repo).
3. **No markdown tables** for directory listings—GitHub file view is already tabular.
4. Link to **source** **`SKILL.md`** where the doc mirrors a skill folder.

## Do not

- Duplicate full skeleton bodies here—reuse this file's rules and sibling doc libraries instead.

## See also

- [`internal-read-git-doc-explanation`](../doc-explanation/SKILL.md)
- [`internal-read-git-repo-layout`](../repo-layout/SKILL.md)
- [`docs/README.md`](../../../../docs/README.md)
