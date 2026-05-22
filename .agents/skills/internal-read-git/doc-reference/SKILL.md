---
name: internal-read-git-doc-reference
description: >-
  Read-only: legacy monolithic docs/REFERENCE.md + docs/reference/wiki templates (migration). Primary narrative
  is docs/README.md + optional docs/ mirror per internal-read-git-repo-layout. Callers: @git-docs. Does not write files.
---

# Internal: Reference documentation template (`internal-read-git-doc-reference`)

**Read-only library.** **Current** default narrative: **`docs/README.md`** + optional **`docs/`** mirror (**[`internal-read-git-repo-layout`](../repo-layout/SKILL.md)**). **Other** repos may add a **profile** wiki per **`internal-read-git-doc-wiki`**—**cursor-skills** does not ship **`docs/wiki/`**. This skill supplies **extra** **migration / legacy** tracks when a repo still wants **`docs/reference/**`** or **`docs/REFERENCE.md`**:

1. **Legacy single file** — **[`wiki-reference-scaffold/SKILL.md`](./wiki-reference-scaffold/SKILL.md)** → **`docs/REFERENCE.md`** (monolithic catalog).
2. **Legacy split wiki** — skill-local **[`reference-readme-scaffold/SKILL.md`](./reference-readme-scaffold/SKILL.md)**, **[`docs-hub-scaffold/SKILL.md`](./docs-hub-scaffold/SKILL.md)** → hub pages under **`docs/reference/**`** (older style; prefer **`docs/README.md`** + domain indexes for greenfield).

**Consumers:** **Manual** scaffold (when user keeps or migrates **`docs/**`**), **`@git-docs`**.

---

## Do

- For **wiki layout**, copy from **`docs/`** (this pack) or your repo’s existing **`docs/`** first, then split catalogs into **`docs/reference/*.md`** when **`wiki-reference-scaffold/SKILL.md`** would exceed **`@git-docs`** size limits.
- For **legacy** repos, copy **`wiki-reference-scaffold/SKILL.md`** into **`docs/REFERENCE.md`**; replace placeholder rows with real skills.
- Keep **public** skills **before** **internal** in any catalog tables.
- Use consistent table columns (**Invoke** | **Source** | **Description**) or document your variant once per repo.

## Do not

- Run commands or edit the filesystem from this skill.

---

## Rendered paths

| Layout | Paths |
| --- | --- |
| **Profile wiki** (preferred) | Often **`docs/README.md`** alone (**minimal**); or expanded sections per **`internal-read-git-doc-wiki`** when you need deeper hubs |
| **Optional skills mirror** | **`docs/.../README.md`** (full tree when opted in) |
| Minimal root | **`README.md`** at repo root |
| Legacy wiki | **`docs/README.md`**, **`docs/reference/**`** |
| Legacy monolith | **`docs/REFERENCE.md`** |
| Flat variant | **`REFERENCE.md`** at repo root |

---

## See also

- [`internal-read-git-doc-wiki`](../doc-wiki/SKILL.md) — doc depth orientation
- [`internal-read-git-repo-layout`](../repo-layout/SKILL.md) — hub contract
- [`internal-read-git-doc-exemplars`](../doc-exemplars/SKILL.md) — markdown patterns + rubric
- [`internal-read-git-doc-graphs`](../doc-graphs/SKILL.md) — diagram authoring (not required **`GRAPHS.md`**)
- [`internal-read-git-doc-user-guide`](../doc-user-guide/SKILL.md) — **`USER_GUIDE.md`**
- [`internal-read-git-readme-root`](../readme-root/SKILL.md) — root **`README.md`**
