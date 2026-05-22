---
name: internal-read-git-doc-explanation
description: >-
  Read-only: how to write overview/explanation sections for docs mirror pages from SKILL.md. Callers: @git-docs,
  internal-write-gh-documentation. Does not write files.
---

# Internal: Doc explanation rules (`internal-read-git-doc-explanation`)

**Read-only library.** **Normative guidance** for **Overview** / narrative blocks on **`docs/<domain>/.../README.md`** or optional **`docs/.../README.md`** mirror pages.

---

## Rules

1. Derive **what / when / boundaries** from **`SKILL.md`** frontmatter **`description`** and **Do** / **Do not** sections.
2. **Link** delegates (**`internal-*`**, sibling public skills) with **one line why**.
3. Avoid speculative roadmaps—describe **current** behavior.

## See also

- [`internal-read-git-doc-index`](../doc-index/SKILL.md)
- [`internal-read-git-doc-graphs`](../doc-graphs/SKILL.md) — when to add **mermaid** + palette
- [`internal-read-git-doc-index`](../doc-index/SKILL.md) — pair narrative sections with stable page navigation.
