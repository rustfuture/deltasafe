---
name: internal-read-git-doc-reference-reference-readme-scaffold
description: >-
  Read-only: root README long-form scaffold. Parent internal-read-git-doc-reference.
---

# Reference wiki

Deep dives and catalogs. **Diagrams:** use self-contained **`mermaid`** in each page; **color and spine** rules live only in **`internal-read-git-doc-graphs`** (**`SKILL.md`** + **`diagrams/*/SKILL.md`**)—do not paste authoring appendix tables into published docs.

---

## Contents

| Page | Description |
| --- | --- |
| <e.g. [skills.md](skills.md)> | <Public + internal skill summaries — link to `SKILL.md`> |
| <e.g. [pipelines.md](pipelines.md)> | <Verify / PR / context flows> |

---

## Folder and file index

Link every domain **`README.md`** under **`skills/`** (and other top-level folders) so readers can jump from wiki to subtree owners.

- **[docs/git.md](../../../../../docs/git.md)**, **[docs/gh.md](../../../../../docs/gh.md)**, **[docs/internal.md](../../../../../docs/internal.md)** (domain docs for `skills/`, **this** pack)
- <add per-domain READMEs>

---

## Template sources (internal)

| Role | Skill |
| --- | --- |
| Wiki hub | `internal-read-git-doc-reference` — child **`SKILL.md`** bodies (e.g. **`wiki-reference-scaffold`**) + repo **`docs/`** |
| User guide | `internal-read-git-doc-user-guide` |
| Diagram authoring | `internal-read-git-doc-graphs` |
| Root / folder README | `internal-read-git-readme-root`, `internal-read-git-readme-tree` |

---

## Related

- [Documentation index](../SKILL.md)
- [USER_GUIDE.md](../USER_GUIDE.md)
