---
name: internal-read-git-readme-tree
description: >-
  Read-only: optional docs/ mirror parity checklist and greenfield hints vs skills/. No tree regen or bulk README apply
  from pack skills—use @git-docs + internal-write-gh-documentation for in-place edits. Does not write files.
---

# Internal: Docs mirror notes (`internal-read-git-readme-tree`)

**Read-only.** This pack keeps **`docs/**`** as a **human-facing mirror** of **`skills/`** when you choose that layout. **Agents** do **not** run a separate “mirror regen” step after **`@git-review`**. **`@git-docs`** and **`internal-write-gh-documentation`** edit **existing** markdown **in place**; if a **`docs/.../README.md`** is missing, add it in a normal edit (same PR) using the bootstrap bullets below.

## Bootstrap (greenfield or new folders)

1. **Domain layout** — **[`internal-read-git-repo-layout`](../repo-layout/SKILL.md)** — public skills under **`skills/<domain>/<verb>/`**; shared libraries under top-level **`skills/read/<domain>/**`** vs **`skills/write/<domain>/**`** depending on whether they mutate the machine or GitHub.
2. **Mirror hub pages** — **`docs/<same-relpath>/README.md`**: short overview, **`## Contents`** as **numbered lists** only (**[`internal-read-git-doc-index`](../doc-index/SKILL.md)**), overview prose per **[`internal-read-git-doc-explanation`](../doc-explanation/SKILL.md)**.
3. **Diagrams** — Add fenced **`mermaid`** only when a diagram clarifies flow; palette and hierarchy per **[`internal-read-git-doc-graphs`](../doc-graphs/SKILL.md)**. Example scaffolds live in **`skills/read/git/doc-graphs/diagrams/*/SKILL.md`** (optional references, not required files in consuming repos).
4. **Tables** — Data tables in reference sections: sort rows, call out large rows (**[`internal-read-git-doc-table`](../doc-table/SKILL.md)**). Never use a markdown **table** for a pure index (**[`internal-read-git-doc-index`](../doc-index/SKILL.md)**).

## Optional parity checklist (human)

If this repo uses a **`docs/`** mirror: skip **`.gitignore`**d paths; spot-check that important **`skills/.../`** dirs have a sibling **`docs/.../README.md`** when that is your convention; **`## Contents`** ordered lexicographically; relative links into **`skills/`** still resolve.

## Do not

- Treat this skill as part of **`@git-review`** after green evaluate—the verify pipeline ends at **`internal-write-gh-documentation`** (**§8a**).

## See also

- [`internal-read-git-repo-layout`](../repo-layout/SKILL.md)
- [`internal-read-git-docs-mirror-bootstrap`](../docs-mirror-bootstrap/SKILL.md)
- [`internal-write-gh-documentation`](../../../write/gh/documentation/SKILL.md)
- [`@git-docs`](../../../git/docs/SKILL.md)
- [`internal-read-git-readme-root`](../readme-root/SKILL.md)
