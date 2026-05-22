---
name: internal-read-git-doc-wiki
description: >-
  Read-only: light orientation for how deep docs/ can go on other repositories; this pack uses docs/README.md + docs/
  mirror only. No structural scaffold workflow in pack skills. Does not write files.
---

# Internal: Doc depth orientation (`internal-read-git-doc-wiki`)

**Read-only.** **This repository** (**cursor-skills**): documentation is **`docs/README.md`** (hub) + **`docs/`** mirroring **`skills/`**. There is **no** separate **`docs/wiki/`** profile tree checked in here.

**Other repositories** may grow richer **`docs/`** trees (architecture notes, runbooks, per-service hubs). This skill **does not** prescribe manifests, bulk scaffolds, or agent-driven “wiki regen”—those are **normal repo edits** outside this pack’s **`@git-*`** shortcuts.

**Consumers:** **`internal-read-git-repo-layout`**, **`internal-write-gh-documentation`**, **`internal-read-git-doc-reference`**, **`internal-read-git-repo-classification`** (optional hints only).

## Bootstrap hints (when starting from scratch)

1. **Pick hub depth** — Smallest useful surface is **`docs/README.md`** + links; add domain wikis (**`docs/git.md`**, **`docs/gh.md`**, …) when **`@`** skills multiply.
2. **Keep domains aligned** — Mirror **`skills/`** folder names under **`docs/`** when you want a wiki beside the pack (**`internal-read-git-repo-layout`**).
3. **API / schema docs** — Add **`docs/api/`** (or similar) only when the repo actually ships HTTP/GraphQL/proto contracts worth documenting (**`internal-read-git-discover-dependencies`** can hint at signals).
4. **Shape** — Indexes, tables, diagrams: **`internal-read-git-doc-index`**, **`internal-read-git-doc-table`**, **`internal-read-git-doc-graphs`**, **`internal-read-git-doc-explanation`**.
5. **Hub starter** — For a cursor-skills-style docs hub scaffold, start from **`internal-read-git-doc-reference/docs-hub-scaffold/SKILL.md`**.

## Do not

- Run commands or write files from this skill.
- Treat this file as a mandatory multi-folder **`docs/wiki/`** scaffold for every consumer of the pack.

## See also

- [`internal-read-git-repo-classification`](../repo-classification/SKILL.md)
- [`internal-read-git-repo-layout`](../repo-layout/SKILL.md)
- [`internal-read-git-readme-tree`](../readme-tree/SKILL.md)
- [`docs/README.md`](../../../../docs/README.md)
