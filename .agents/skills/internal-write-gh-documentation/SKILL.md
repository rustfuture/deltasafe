---
name: internal-write-gh-documentation
description: >-
  Sharpen existing docs/** (wiki sections, docs/** indexes, optional slimmer domain layouts on other repos),
  minimal root README.md, and legacy paths if present. Runs as @git-review §8a after green evaluate. Does not create new
  doc pages. Broader post-green pass: @git-docs. Not a substitute for @git-review §8a.
---

# Internal: Project documentation (`internal-write-gh-documentation`)

**Library.** Single **batch** for **`@git-review`** **§8a**: after **[`internal-write-gh-evaluate`](../evaluate/SKILL.md)** succeeds, **edit existing** markdown only.

**Write safety:** before any documentation mutation, set a clear **Goal**, present **AskQuestion** confirmation options when scope is non-trivial, and only proceed after explicit **Proceed/confirm** per **`internal-write-plan-skill-safety`** + **`internal-write-plan-structured-qa`**.

**Consumers:** **`@git-review`** §8a only. For **missing `docs/.../README.md`** or mirror layout questions, read **`internal-read-git-readme-tree`** (read-only bootstrap + parity checklist) and **`internal-read-git-repo-layout`**—**do not** invent a bulk “regen” step here. For **post-green** doc accuracy (**existing** paths **in place**; **no** new/deleted/moved **`.md`**), **`@git-docs`**—read **`internal-read-git-repo-layout`** so links match the tree.

**Scope:** **Complement** what already exists—align lists, links, and commands with **current** tree and CI. **Do not** bulk regen **`docs/`** here.

---

## Do

1. **Repo root** — Target **open** project (not necessarily cursor-skills).
2. **Inventory** — Always: **minimal** root **`README.md`**, **`docs/**`**. Include **existing** paths only: **`docs/README.md`** (simple repos may use **only** this file under **`docs/`**); if present, also profile wiki dirs (**`docs/architecture/`**, **`docs/organization/`**, **`docs/domains/`**, **`docs/workflows/`**, **`docs/data/`**, **`docs/conventions/`**, **`docs/operations/`**, **`docs/api/`**, **`docs/services/`**). If legacy **`skills/**/README.md`** (other repos) or root **`USER_GUIDE.md`** / **`REFERENCE.md`** still exist, include only when present—this pack keeps **no** README under **`skills/`** when mirror policy applies. **Do not** **create** new **`docs/`** pages here. If **no** **`docs/`** hub exists, point the user to **`internal-read-git-repo-layout`** / **`internal-read-git-readme-tree`**—**`@git-docs`** does not create new pages.
3. **Sharpen** — Patch in place: commands, lists, links, **colored** mermaid; match **current** tree, CI, and **`internal-write-gh-evaluate`**. **Implicit markdown conventions (no user citation required)** — apply together with this step whenever the file has **`## Contents`**, **overview** prose, **tables**, or **mermaid**: **`internal-read-git-doc-index`** (numbered lists; **no** index tables), **`internal-read-git-doc-explanation`** (explanation sections), **`internal-read-git-doc-table`** (reference tables: sort + largest-row callout), **`internal-read-git-doc-graphs`** (when to diagram + palette), **`internal-read-git-doc-exemplars`** (tone/rubric), **`internal-read-git-readme-root`** (minimal root README when path is repo root), **`internal-read-git-repo-layout`** (hub vs mirror). **Do not** duplicate long prose from those libraries in chat—open the linked **`SKILL.md`** when needed.
4. **Do not** use this library as a shortcut for **`@git-docs`** or **`@git-commit`**—those skills own broader in-place doc edits and staging/commits respectively.

---

## Do not

- **Create** new `.md` files (including scaffold / “missing page” fills).
- Run if Evaluate **failed**.
- `git commit` / `git push`.

---

## Verification

- [ ] Only existing paths were modified.
- [ ] Docs match repo + checks; **`## Contents`** avoids markdown **tables** for pure indexes; **reference tables** follow **`internal-read-git-doc-table`** when edited.

---

## Notes

- Prefer **mermaid** in `.md` for flowcharts (**`skills/read/git/doc-graphs/diagrams/`** + **`internal-read-git-doc-graphs`**).
- **`SKILL.md`** conventions: keep **`name`** / **`description`** accurate; link delegates instead of duplicating command fences.

## See also

- [`internal-read-git-readme-tree`](../../read/git/readme-tree/SKILL.md) — read-only mirror bootstrap + checklist
- [`internal-read-git-repo-layout`](../../read/git/repo-layout/SKILL.md)
- [`@git-docs`](../../../git/docs/SKILL.md)
- [`@git-review`](../../../git/review/SKILL.md) §8a
