---
name: internal-read-git-doc-user-guide
description: >-
  Read-only: TEMPLATE.md for legacy docs/USER_GUIDE.md (migration). README-only repos fold narrative into
  internal-read-git-readme-root / root README. Callers: @git-docs. Does
  not write files.
---

# Internal: User guide template (`internal-read-git-doc-user-guide`)

**Read-only library.** Canonical **body** for **legacy** **`docs/USER_GUIDE.md`** when a repo still uses a **`docs/`** hub. **Default** for this pack: fold the same material into **root `README.md`** (**[`internal-read-git-readme-root`](../readme-root/SKILL.md)**). Full source: **`[TEMPLATE.md](./TEMPLATE.md)`**.

**Consumers:** **`@git-docs`** when maintaining **`docs/USER_GUIDE.md`** in a legacy tree; otherwise prefer **`internal-read-git-readme-root`**.

---

## Do

- **Install from scratch** (especially **macOS** **Terminal**, **raw** `brew` / `git` / `gh` commands).
- Describe **happy path** workflows (e.g. **`gh-start` → work → `gh-pr`**).
- Point **short** install blurbs at root **`README.md`**; keep **catalogs** under **`docs/*.md`** hubs (**this** pack: **`docs/git.md`**, **`docs/gh.md`**, **`docs/internal.md`**) or optional wiki-only layouts on other repos, or, in legacy repos, **`docs/reference/**`** / **`REFERENCE.md`**.
- Document **`.cursor/`** or project-local artifact dirs with a **who writes what** table.

## Do not

- Duplicate long wiki tables — link **`docs/README.md`** (**this** pack), optional **`docs/README.md`**, or **`docs/reference/**`** (legacy).

---

## Rendered path

| Layout | Path |
| --- | --- |
| **Domain docs** (**this** pack) | **`docs/README.md`** + **`docs/git.md`** / **`docs/gh.md`** / **`docs/internal.md`** — **[`internal-read-git-repo-layout`](../repo-layout/SKILL.md)** |
| Legacy hub | **`docs/USER_GUIDE.md`** |
| Variant | **`USER_GUIDE.md`** at repo root |

---

## See also

- [`internal-read-git-doc-exemplars`](../doc-exemplars/SKILL.md) — public markdown patterns + rubric
- [`internal-read-git-doc-reference`](../doc-reference/SKILL.md)
- [`internal-read-git-doc-graphs`](../doc-graphs/SKILL.md)
- [`internal-read-git-doc-wiki`](../doc-wiki/SKILL.md) — doc depth orientation
- [`@git-docs`](../../../git/docs/SKILL.md)
