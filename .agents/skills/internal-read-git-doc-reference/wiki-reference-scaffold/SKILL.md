---
name: internal-read-git-doc-reference-wiki-reference-scaffold
description: >-
  Read-only: legacy REFERENCE/wiki page scaffold. Parent internal-read-git-doc-reference.
---

# Skills reference

Complete catalog of **every** skill in this repository: **public** skills first (user-invoked under `skills/git/` and `skills/gh/`), then **internal** libraries (`skills/<domain>/`). Invocation: `/flattened-name` or `@flattened-name` (path under `skills/` with `/` → `-`, e.g. `skills/gh/pr/SKILL.md` → **`gh-pr`**).

**Install:** [README.md](../README.md) · **Doc index:** [README.md](README.md) (wiki) or this file (legacy) · **Workflows:** [USER_GUIDE.md](USER_GUIDE.md)

---

## Documentation set (`docs/`)

**Preferred (wiki):** [README.md](README.md), [USER_GUIDE.md](USER_GUIDE.md), **`reference/**`** — scaffold **manual / agent-led** from **`internal-read-git-doc-reference`** child skills (**`reference-readme-scaffold`**, **`docs-hub-scaffold`**, …) **confirm-first**. **Legacy:** this monolithic **[REFERENCE.md](REFERENCE.md)**; optional **[GRAPHS.md](GRAPHS.md)**. Templates stay in **`skills/read/git/doc-*`**.

| Path | Role |
| --- | --- |
| [README.md](README.md) | Doc hub index (wiki layout) |
| [reference/README.md](reference/README.md) | Reference wiki start page |
| [REFERENCE.md](REFERENCE.md) | This file — legacy full catalog |
| [USER_GUIDE.md](USER_GUIDE.md) | Install, prerequisites, workflows |
| [GRAPHS.md](GRAPHS.md) | Optional legacy diagram file |

---

## Folder and file index

Hub **READMEs** (subtree scope and **Related** links)—use with **`docs/reference/**`** or small **mermaid** in each **README** when a diagram helps:

- **[`docs/git.md`](../../../../../docs/git.md)**, **[`docs/gh.md`](../../../../../docs/gh.md)**, **[`docs/internal.md`](../../../../../docs/internal.md)** — domain wikis (**this** pack)
- **`skills/git/**`**, **`skills/gh/**`**, **`skills/read/**`**, **`skills/write/**`** — normative **`SKILL.md`** per folder
- **[`skills/write/plan/`](../../../../write/plan/)** — structured Q&A + safety (**`internal-write-plan-structured-qa`**, **`internal-write-plan-skill-safety`**)
- **[`skills/read/git/SKILL.md`](../../../../read/git/SKILL.md)**, **[`skills/read/gh/SKILL.md`](../../../../read/gh/SKILL.md)**, **[`skills/read/plan/SKILL.md`](../../../../read/plan/SKILL.md)** — library root indexes

**Scaffold sources** — **`skills/read/git/doc-index/`** skeleton child skills + diagram scaffolds under **`internal-read-git-doc-graphs`**, plus other **`internal-read-git-doc-*`** child **`SKILL.md`** bodies. **`@git-docs`** only aligns **existing** pages **in place**—it does **not** add new **`docs/`** files from scaffolds alone.

| Scaffold role | Skill | Location |
| --- | --- | --- |
| **`docs/README.md`** + **`docs/reference/**`** | [`internal-read-git-doc-reference`](../SKILL.md) | [`reference-readme-scaffold/SKILL.md`](../reference-readme-scaffold/SKILL.md), [`docs-hub-scaffold/SKILL.md`](../docs-hub-scaffold/SKILL.md) |
| **`docs/REFERENCE.md`** (legacy) | same | [`wiki-reference-scaffold/SKILL.md`](./SKILL.md) |
| **Optional `docs/GRAPHS.md`** | [`internal-read-git-doc-graphs`](../doc-graphs/SKILL.md) | [`../doc-graphs/legacy-graphs-pack-scaffold/SKILL.md`](../doc-graphs/legacy-graphs-pack-scaffold/SKILL.md) |
| **`docs/USER_GUIDE.md`** | [`internal-read-git-doc-user-guide`](../doc-user-guide/SKILL.md) | [`../doc-user-guide/user-guide-scaffold/SKILL.md`](../doc-user-guide/user-guide-scaffold/SKILL.md) |
| **Root `README.md`** | [`internal-read-git-readme-root`](../readme-root/SKILL.md) | skill body + exemplar § |
| **`docs/...` mirror `README.md`** | [`internal-read-git-readme-tree`](../readme-tree/SKILL.md) | Rules in **`SKILL.md`**; optional examples under [`doc-index/`](../doc-index/) |

---

## Public skills

### Local git (`skills/git/`)

| Invoke | Source | Description |
| --- | --- | --- |
| **`@git-review`** | [`skills/git/review/SKILL.md`](../../../../git/review/SKILL.md) | *Discover → install → evaluate → §8a docs.* |
| **`@git-docs`** | [`skills/git/docs/SKILL.md`](../../../../git/docs/SKILL.md) | *Post-green doc accuracy: existing `.md` in place only; may no-op; Q&A for scope.* |
| **`@git-main`** | [`skills/git/main/SKILL.md`](../../../../git/main/SKILL.md) | *Align to canonical main.* |
| **`@git-pull`** | [`skills/git/pull/SKILL.md`](../../../../git/pull/SKILL.md) | *Fetch + merge main + conflicts playbook.* |
| **`@git-push`** | [`skills/git/push/SKILL.md`](../../../../git/push/SKILL.md) | *Inventory → `@git-commit` if needed → branch gate → push.* |
| **`@git-reset`** | [`skills/git/reset/SKILL.md`](../../../../git/reset/SKILL.md) | *Reset to target ref.* |
| **`@git-start`** | [`skills/git/start/SKILL.md`](../../../../git/start/SKILL.md) | *Dirty-tree gate → @git-main → new branch → optional @git-push.* |
| **`@git-zip`** | [`skills/git/zip/SKILL.md`](../../../../git/zip/SKILL.md) | *Orchestrates `@git-tag` when needed, then `git archive` export (GitHub-style); runnable blocks in `internal-write-git-zip`.* |

### GitHub (`skills/gh/`)

| Invoke | Source | Description |
| --- | --- | --- |
| **`@gh-pr`** | [`skills/gh/pr/SKILL.md`](../../../../gh/pr/SKILL.md) | *@git-pull → @git-review → @git-push → PR description → `gh pr`.* |
| **`@gh-pr-*`** (list, view, close, skills) | [`skills/gh/pr/`](../../../../gh/pr/) | *PR helpers; create/edit only via **`@gh-pr`**; list **`internal-read-gh-pr-list`**; mutations **`internal-write-gh-pr-commands`**; hub **[`list-view-hub/SKILL.md`](../../../../read/gh/pr-list/list-view-hub/SKILL.md)**.* |
| **`@gh-issue-*`** (create, list, view, pick, edit, close, delete-closed) | [`skills/gh/issues/`](../../../../gh/issues/) | *Issues CRUD + dedupe + label helpers; list **`internal-read-gh-issue-list`**; view/mutations **`internal-write-gh-issue-commands`**; template **`gh repo view`** in **`internal-read-gh-repo-forms-json`**.* |

---

## Internal skills

Libraries under `skills/<domain>/`. Flattened names: path with `/` → `-` (e.g. `write/gh/evaluate` → **`internal-write-gh-evaluate`**).

### Agent (`skills/write/plan/`)

| Invoke | Source | Description |
| --- | --- | --- |
| **`internal-write-plan-structured-qa`** | [`skills/write/plan/structured-qa/SKILL.md`](../../../../write/plan/structured-qa/SKILL.md) | *AskQuestion / confirm UX.* |
| **`internal-write-plan-skill-safety`** | [`skills/write/plan/skill-safety/SKILL.md`](../../../../write/plan/skill-safety/SKILL.md) | *What needs confirm.* |

### Internal libraries (verify + docs + `gh`)

| Invoke | Source | Description |
| --- | --- | --- |
| **`internal-read-git-configuration`** | [`../../../../read/git/configuration/SKILL.md`](../../../../read/git/configuration/SKILL.md) | *Resolve format/lint/test commands.* |
| **`internal-read-git-discover-dependencies`** | [`../../../../read/git/discover-dependencies/SKILL.md`](../../../../read/git/discover-dependencies/SKILL.md) | *Scan repo for stacks and gaps.* |
| **`internal-write-gh-documentation`** | [`../../../../write/gh/documentation/SKILL.md`](../../../../write/gh/documentation/SKILL.md) | *Sharpen existing markdown — §8a.* |
| **`internal-read-git-readme-tree`** | [`../readme-tree/SKILL.md`](../readme-tree/SKILL.md) | *Read-only mirror bootstrap + checklist.* |
| **`internal-write-gh-evaluate`** | [`../../../../write/gh/evaluate/SKILL.md`](../../../../write/gh/evaluate/SKILL.md) | *Run verify matrix.* |
| **`internal-write-gh-install-dependencies`** | [`../../../../write/gh/install-dependencies/SKILL.md`](../../../../write/gh/install-dependencies/SKILL.md) | *Install / build — §3.* |
| **`internal-read-git-repo-layout`** | [`../repo-layout/SKILL.md`](../repo-layout/SKILL.md) | *Layout rules — read-only.* |
| **`internal-read-gh-repo-forms-json`** | [`../../../../read/gh/repo-forms-json/SKILL.md`](../../../../read/gh/repo-forms-json/SKILL.md) | *Sole `gh repo view --json` shapes for issue/PR templates.* |
| **`internal-read-gh-pr-description`** | [`../../../../read/gh/pr-description/SKILL.md`](../../../../read/gh/pr-description/SKILL.md) | *PR templates + delta + title/body.* |
| **`internal-read-gh-pr-body-sections`** | [`../../../../read/gh/pr-body-sections/SKILL.md`](../../../../read/gh/pr-body-sections/SKILL.md) | *Static PR body patterns.* |
| **`internal-read-gh-issue-list`** | [`../../../../read/gh/issue-list/SKILL.md`](../../../../read/gh/issue-list/SKILL.md) | *`gh issue list` / `gh search issues` (read-only).* |
| **`internal-write-gh-issue-commands`** | [`../../../../write/gh/issue-commands/SKILL.md`](../../../../write/gh/issue-commands/SKILL.md) | *`gh issue` view/create/edit/close/delete.* |
| **`internal-read-gh-pr-list`** | [`../../../../read/gh/pr-list/SKILL.md`](../../../../read/gh/pr-list/SKILL.md) | *`gh pr list` / `view` / `diff --stat` (read-only).* |
| **`internal-write-gh-pr-commands`** | [`../../../../write/gh/pr-commands/SKILL.md`](../../../../write/gh/pr-commands/SKILL.md) | *`gh pr create`, `gh pr edit`, `gh pr close`.* |
| **`internal-read-git-doc-graphs`** | [`../doc-graphs/SKILL.md`](../doc-graphs/SKILL.md) | *README mermaid rules + `diagrams/`.* |
| **`internal-read-git-doc-reference`** | [`../SKILL.md`](../SKILL.md) | *REFERENCE.md template.* |
| **`internal-read-git-doc-user-guide`** | [`../doc-user-guide/SKILL.md`](../doc-user-guide/SKILL.md) | *USER_GUIDE.md template.* |
| **`internal-read-git-doc-exemplars`** | [`../doc-exemplars/SKILL.md`](../doc-exemplars/SKILL.md) | *Public markdown patterns + adoption rubric.* |
| **`internal-read-git-readme-root`** | [`../readme-root/SKILL.md`](../readme-root/SKILL.md) | *Root README template.* |
| *…* | *…* | *`repo-stream`, `merge-conflicts`, `git-diff-summary`, …* |

---

## Pipeline order (verify)

**`@git-review`:** discover-dependencies → install-dependencies → configuration → evaluate → internal-write-gh-documentation (§8a). Diagram: [GRAPHS.md](GRAPHS.md).

---

## See also

- [Skills graphs](GRAPHS.md)
- [User guide](USER_GUIDE.md)
- [install.sh](../install.sh) — if your pack uses a copy script
