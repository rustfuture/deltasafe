---
name: internal-read-git-repo-layout
description: >-
  Read-only: domain-driven skills/, docs/README.md + optional docs/ mirror (this pack), diagram examples under
  skills/read/git/doc-graphs/diagrams/*/SKILL.md, minimal root README. Consumed by @git-docs (read-only alignment); not @git-review.
---

# Internal: Repo layout (domain-driven)

**Read-only.** Canonical **shape** for repositories that use this skill pack: **`skills/`** (workflows), **`docs/README.md`** + optional **`docs/...`** beside **`skills/`** (per-folder **`README.md`** and workflow markdown), **`skills/read/git/doc-graphs/diagrams/*/SKILL.md`** (optional mermaid examples), and a **minimal** root **`README.md`**. **Does not** run commands or verify—**[`@git-docs`](../../../git/docs/SKILL.md)** aligns **existing** markdown to this contract (**in place** only).

**Consumers:** **`@git-docs`** (post-green **content** sync on **existing** **`.md`** only—**no** structural file ops from that skill). **Single source** for layout rules—do not duplicate these tables in other skills.

## Do

- Serve as the **single** read-only contract for **`skills/`** + documentation hub shape; callers **read** it and **link** here instead of copying prose.

## Do not

- Run commands, create files, or verify—**[`@git-docs`](../../../git/docs/SKILL.md)** applies **in-place** doc alignment only.

---

## 1. Skills (`skills/`)

Treat **`skills/`** as a **single aggregate** of agent workflows, partitioned by **domain** (bounded context):

| Domain | Typical path | Group together |
| --- | --- | --- |
| **GitHub (`gh`)** | `skills/gh/...` | **`pr`**, **`issues/*`** — **`@gh-*`** only; local git lives under **`skills/git/`** |
| **Cross-cutting internal** | `skills/read/<domain>/**`, `skills/write/<domain>/**` | **`read/gh`**: **`gh`** inventory and PR/issue form helpers (issues, PRs, **`repo-stream`**, **`repo-forms-json`**). **`read/git`**: verify prep (**`discover-dependencies`**, **`configuration`**), doc/markdown rules (**`doc-*`**, **`readme-*`**, **`repo-layout`**, **`repo-classification`**, **`doc-wiki`**), **`git-diff-summary`**, **`merge-conflicts`**. **`write/gh`**: install, evaluate, documentation, issue/pr **command** fences. **`write/git`**: zip, tag, working-tree align. **`write/plan`**: safety + structured Q&A. |
| **Public wikis + scaffolds** | **`docs/git.md`**, **`docs/gh.md`**, **`internal-read-git-doc-graphs`** `diagrams/` … | **Not** invocable skills—**`.md`** only (diagrams use **fenced `mermaid`** inside those files). |

**Rules**

1. **One skill per folder** — each leaf is `.../<name>/SKILL.md` with YAML **`name`** + **`description`** matching the pack’s conventions.
2. **Similar verbs, same parent** — e.g. all `gh-*` public skills live under **`skills/gh/`**, not scattered at `skills/` root.
3. **Public vs library** — user-invoked flows stay shallow (`skills/<domain>/<verb>`); shared libraries and policies live under `skills/read/<domain>` and `skills/write/<domain>`.
4. **Names mirror behavior** — folder names stay short and match the skill `name` prefix (`git-docs` → `skills/git/docs/`).

When **moving** skill folders, update **relative links** in the same change set; **`internal-read-git-repo-layout`** stays the single source for layout rules.

---

## 2. Narrative wiki + optional mirror under `docs/`

**Mirror:** when you use one, **`docs/<relpath>/README.md`** may sit beside each **`skills/`** directory; parity expectations are a **repo convention**, not an automated pack step—see **`internal-read-git-readme-tree`** for a short checklist.

**Root `README.md`:** minimal—title, purpose, install pointer, link **`docs/README.md`** (**`internal-read-git-readme-root`**).

**`skills/`:** one **`SKILL.md`** per leaf; with a mirror, avoid duplicate prose in **`skills/**/README.md`**—use **`docs/...`** or **`SKILL.md`** only.

**Policy vs bodies:** “when/how” in **`internal-read-git-doc-*`**, **`internal-read-gh-pr-content`**, **`internal-read-git-git-diff-summary`**, **`internal-read-git-doc-graphs`**; long checklist/skeleton/scaffold bodies as **sibling `.md`** files next to the owning **`internal-*`** library; **`docs/`** wikis stay **short** indexes over **`skills/`**.

**Hub scaffold:** use **`internal-read-git-doc-reference/docs-hub-scaffold/SKILL.md`** as the baseline for emoji headings, numbered `Contents`, and structural comparison tables in new/reshaped docs hubs.

**Exemplars:** **`internal-read-git-doc-exemplars`**.

---

## 3. What this is not

- **Not** **`@git-review`** — verify + **light** polish of **existing** paths only.
- **Not** **`internal-write-gh-documentation`** in place of this file — that skill **edits** after green tests; **layout policy** stays here only.

---

## See also

- [`internal-read-git-doc-wiki`](../doc-wiki/SKILL.md) — how deep **`docs/`** might go elsewhere
- [`internal-read-git-readme-tree`](../readme-tree/SKILL.md) — mirror bootstrap + optional parity checklist
- [`@git-docs`](../../../git/docs/SKILL.md) — post-green doc accuracy (**existing** files **in place**)
- [`internal-write-gh-documentation`](../../write/gh/documentation/SKILL.md) — §8a polish existing docs after green evaluate
- [`docs/README.md`](../../../../docs/README.md) — documentation hub (**this** pack)

## 4. Naming hygiene

- Avoid nested segments repeating the parent (`foo/foo/`). For this pack, keep planning internals under `skills/read/plan/core/` (not `skills/read/plan/plan/`).
- When moving folders, keep `name:` path-faithful with `internal-<full-path-with-dashes>` for read/write internals.

## 5. Workspace staging conventions (`.cursor/`)

- `.cursor/plan.md` is the canonical planning hub.
- `.cursor/plans/<slug>.md` is an overflow split linked from the hub.
- `.cursor/gh/issues/` stores issue draft bodies/manifests.
- `.cursor/gh/pr/` stores PR draft fragments.
