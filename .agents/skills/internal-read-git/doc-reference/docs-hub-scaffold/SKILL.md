---
name: internal-read-git-doc-reference-docs-hub-scaffold
description: >-
  Read-only: docs hub README scaffold. Parent internal-read-git-doc-reference.
---

# Documentation index

**Repository:** <name> — <one-line purpose>.

Use this as a **wiki hub** starter when `docs/` is the human-facing surface and `skills/**/SKILL.md` remains normative.

---

## 📑 Contents

1. [Location map](#location-map)
2. [Browse](#browse)
3. [Common commands](#common-commands)
4. [Diagram policy](#diagram-policy)
5. [See also](#see-also)

---

## 🧩 Location map

| Location | Purpose |
| --- | --- |
| [`docs/README.md`](README.md) | Hub index and routing for human readers. |
| [`docs/git.md`](git.md) | Public/local git workflow wiki. |
| [`docs/gh.md`](gh.md) | GitHub issue/PR workflow wiki. |
| [`docs/internal.md`](internal.md) | Internal read/write library map. |
| [`skills/**/SKILL.md`](../skills/) | Source of truth for contracts and behavior. |

---

## 🧩 Browse

| Topic | Description |
| --- | --- |
| [reference/README.md](reference/README.md) | Start here — map of reference pages |
| <add more rows> | <link each `docs/reference/*.md`> |

---

## 🧩 Common commands

1. `@git-review` for verification.
2. `@git-docs` for in-place doc corrections.
3. `@git-push` for commit + publish.
4. `@gh-pr` after pull + push for PR metadata.

---

## 🧩 Diagram policy

- Diagrams are welcome in the hub and key flow pages, but avoid adding Mermaid blocks everywhere.
- Keep 0-2 diagrams per page unless the workflow is genuinely complex.
- For GitHub rendering safety, quote labels containing special characters (for example `node["@gh-pr"]`).
- For palette and scaffold examples, use [`internal-read-git-doc-graphs`](../doc-graphs/SKILL.md).

---

## 🔗 See also

- **[docs/git.md](../../../../../docs/git.md)**, **[docs/gh.md](../../../../../docs/gh.md)**, **[docs/internal.md](../../../../../docs/internal.md)** — agent skills by domain (**this** pack)
- **Pasteables** under **`docs/`**: see *Folder and file index* in [reference/README.md](reference/README.md) or your legacy [REFERENCE.md](REFERENCE.md)
