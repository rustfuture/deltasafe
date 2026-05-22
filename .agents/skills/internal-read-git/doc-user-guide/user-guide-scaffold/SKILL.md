---
name: internal-read-git-doc-user-guide-user-guide-scaffold
description: >-
  Read-only: legacy USER_GUIDE scaffold. Parent internal-read-git-doc-user-guide.
---

---
template_for: docs/USER_GUIDE.md
source_skill: internal-read-git-doc-user-guide
instructions: >-
  Copy into docs/USER_GUIDE.md. Replace skill names and paths with your pack. Link install to root README.
---

# User guide

**Purpose:** **install from scratch**, **prerequisites**, **workflows**, **per-repo storage**, **troubleshooting**. Use **raw Terminal commands** (especially on **macOS**). **README-only repos:** fold this into **root `README.md`**; catalogs live in **`docs/README.md`** plus **`docs/git.md`**, **`docs/gh.md`**, **`docs/internal.md`**, … (**this** pack) or optional **`docs/`** mirror. **Legacy:** deep catalogs in **`docs/reference/**`** or **[REFERENCE.md](REFERENCE.md)**; hub index **`docs/README.md`** if present.

**Root** **`README.md`** stays short; **`docs/<domain>/`** documents domains (**this** pack), or optional **`docs/.../README.md`** mirrors the **`skills/`** tree on other repos (no per-folder README under **`skills/`** when using that mirror policy).

---

## Prerequisites (macOS)

Run in **Terminal** (replace versions with what your project needs).

```bash
# Xcode CLI tools (if prompted)
xcode-select --install

# Homebrew (if missing) — see https://brew.sh
# /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Example toolchains (adapt)
brew install git
brew install gh && gh auth login
# brew install node@22   # if your pack needs Node
```

**Cursor:** install the editor from vendor instructions; restart after adding skills. **Skills install:** from repo root, `./install.sh` (or your pack’s script)—see root **README**.

---

## Diagrams

Prefer **small `mermaid` blocks** in **folder `README.md`** / **root `README.md`** (README-only), or in **`docs/reference/**`** (legacy). **Color / spine rules:** **`internal-read-git-doc-graphs`** **`SKILL.md`** + **`diagrams/`** child skills—optional legacy **`docs/GRAPHS.md`**, not default. Optional PNG export: **`diagrams/diagrams-index/SKILL.md`** in that library.

---

## Workflows

Open a **target git repo** in Cursor. Invoke with `/skill-name` or `@skill-name`.

| Goal | Skills |
| --- | --- |
| Issue → implement → PR | `@gh-issue-pick` / `@gh-issue-view` → `@git-start` → `@gh-pr` (runs pull, review, push, then PR) |
| Verify (no push) | `@git-review` |
| Commit + push | `@git-push` |
| Pull request | `@gh-pr` (full chain: `@git-pull` → `@git-review` → `@git-push` → PR text) |
| New branch from clean `main` | `@git-start` |
| Doc accuracy (existing files, after green verify) | `@git-docs` — **not** part of **`@git-review`** evaluate; run after **`@git-review`** when you need a broader in-place pass |

**Typical task:** **`@git-start`** creates a branch and runs **`@git-push`** (commit-if-needed + publish). Run **`@git-review`** before PR work if you want green locally first, or rely on **`@gh-pr`** which runs **`@git-review`** then **`@git-push`**, then PR text and **`gh pr create` / `edit`**.

---

## Documentation workflow

| Situation | Skill | Notes |
| --- | --- | --- |
| Greenfield or missing mirror / large `docs/` reshuffle | Normal change set | Add or move **`docs/`** paths in git like any other source; use **`internal-read-git-readme-tree`** + **`internal-read-git-repo-layout`** as guides; **`internal-write-plan-skill-safety`** when the diff is huge. |
| After code changes, before commit | `@git-commit` or `@git-push` §1–2 | **`@git-commit`** for a local savepoint; **`@git-push`** runs inventory then **`@git-commit`** only when needed. For doc-only tweaks before commit, **`@git-docs`** (existing **`.md`** in place). **New** paths → add them in a normal edit first. |
| After tests pass (verify pipeline) | `@git-review` §8a | §8a light polish on **existing** paths (**`internal-write-gh-documentation`**). |

Do **not** use **`@git-review`** alone to **reshape** the whole doc tree for big moves — do that as explicit file edits with the usual review and safety confirms.

---

## Per-repo storage (`<project>/.cursor/`)

Optional **`.cursor/`** snippets — copy subtrees from this pack’s **`docs/`** (and diagram scaffolds under **`skills/read/git/doc-graphs/diagrams/`**) (see root **`README.md`** and **`docs/README.md`**). **`@git-zip`** writes the **`.zip`** **outside** the repo root by default—confirm paths per **`internal-write-plan-skill-safety`**.

---

## Troubleshooting

| Issue | What to try |
| --- | --- |
| Skill not listed | Re-run your install script; restart Cursor |
| Stack not detected | README + CI config at repo root |
| Checks fail | Install toolchains; re-run `@git-review` |
| `gh` CLI missing | Install GitHub CLI and `gh auth login` |

---

## See also

- **[Skills reference](REFERENCE.md)**
- **[Skills graphs](GRAPHS.md)**
- **[Root README](../README.md)**
