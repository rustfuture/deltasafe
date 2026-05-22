---
name: git-docs
description: >-
  Post-green doc accuracy: edit existing markdown in place only—no new/deleted/moved .md, no split/merge of files.
  Additive tweaks (bullets, links, facts) when the change set requires it; no-op if docs already match. After @git-review
  succeeds, or docs-only pass. Does not run internal-write-gh-evaluate. New paths or doc-tree reshaping are normal edits outside this skill.
---

# `@git-docs` (documentation sync — no structure)

Normative fences / full matrix: [`internal-read-git-repo-layout`](../../read/git/repo-layout/SKILL.md), [`internal-write-gh-documentation`](../../write/gh/documentation/SKILL.md).


**Public.** **Goal:** After **`@git-review`** has **succeeded** (or on a **docs-only** pass on a clean tree), check whether **existing** documentation needs **minimal** factual updates so it matches the **current** branch and repo. **Edit only paths that already exist**—**additive** or **surgical** corrections (wrong sentence, missing bullet, link, env var, command). If nothing is missing, **report that docs are already good** and **make no edits** (**no-op**). **Not** for fixing failing tests—loop **`@git-review`** first.

**Canonical layout (read-only here):** **[`internal-read-git-repo-layout`](../../read/git/repo-layout/SKILL.md)** + **[`internal-read-git-doc-wiki`](../../read/git/doc-wiki/SKILL.md)** (light orientation for **`docs/`** depth on **other** repos). **New** narrative paths, mirror folders, moves, splits: **not** this skill—add or reshape files in a regular change set; use **`internal-read-git-readme-tree`** (bootstrap bullets) and **`internal-read-git-repo-layout`** as guides.

**Libraries (orchestration + implicit conventions):** **[`internal-write-gh-documentation`](../../write/gh/documentation/SKILL.md)** (§8a in-place posture + **implicit** **`internal-read-git-doc-*`** bundle), **`internal-read-git-readme-root`**, **`internal-read-git-doc-exemplars`**, **`internal-read-git-doc-graphs`** — use them for **in-place** edits only. **Do not** wait for the user to name **`internal-read-git-doc-table`** / **`internal-read-git-doc-index`** when fixing **`README.md`** or **`docs/**`** tables and indexes—apply those rules as part of this skill (same bundle as **`internal-write-gh-documentation`** / **`internal-read-git-repo-layout`**).

---

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Prefer running after `@git-review` succeeds or in a docs-only pass.
2. Use `@git-push` orchestration context when docs sync is part of publish prep.
3. Docs mirror fences: **`internal-read-git-repo-layout`** and **`internal-read-git-doc-wiki`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-repo-layout`
2. `internal-read-git-doc-wiki`
3. `internal-write-gh-documentation`
4. `internal-read-git-repo-classification`
5. `internal-read-git-doc-reference`
6. `internal-read-git-doc-user-guide`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Return to `@git-review` if code-health checks still fail.
2. Use `@gh-pr` after docs updates are ready to ship with branch changes.

## Q&A bypass ENV

- `SKIP_QA_GIT_DOCS=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## No structure (hard rules)

- **Do not** **create**, **delete**, **rename**, or **move** **`.md`** files.
- **Do not** **split** one doc into several or **merge** files.
- **Do not** **reorganize** the whole section tree for style or preference.
- **Within** a file, prefer **small** edits (fix a line, **append** a bullet) over rewriting large regions or churning prose.
- If the **information architecture** or mirror tree is wrong, **say so in chat** and point to **`internal-read-git-repo-layout`** and **`internal-read-git-readme-tree`**—**do not** fix structure inside **`@git-docs`**.
- If a file is **oversized** and should be split, **say so in chat**—**do not** split here (use a follow-up change set with **`internal-write-plan-skill-safety`** if the edit is large).

---

## On invoke

1. **Goal line** — One sentence (or inherit from **`@git-review`** / **`@gh-pr`** context when this pass follows green verify).
2. **AskQuestion** (finite modes), per **structured-qa** **§2** — e.g. **Diff-scoped check** (default when following a small code change—infer **relevant** existing docs from the user’s stated paths, **`@git-review`** output, or workspace reads; **do not** embed **`git`** recipes in this skill) · **Wider inventory** (**`docs/**/*.md`**, root **`README.md`**, repo-root **`*.md`**) · **New paths / tree reshape** — not this skill; say so and stop.
3. **Large in-place edits** (many files, high line count) — **plan preview** + **No**/**Yes** per **skill-safety** + **structured-qa** **§1c** / **§3a**.

---

## Do

1. **Start from the change set** — Use **touched paths** implied by the user, review artifacts, or **`internal-write-gh-documentation`** alignment posture; open the **smallest** set of **existing** doc files that should reflect those changes.
2. **Tiny / non-doc changes** — If nothing user-facing changed or **existing** docs already mention it, **state that and stop** (**no-op**). **Do not** edit for style alone.
3. **Wider pass** — When the user asks, walk **`docs/**/*.md`**, root **`README.md`**, and repo-root **`*.md`** (still **in place** only). **Exclude by default** **`skills/**/SKILL.md`** unless the user **explicitly** expands scope (those files follow **skill authoring**).
4. **Apply** — Patch using the same discipline as **`internal-write-gh-documentation`**: lists, links, commands, optional **colored** mermaid per **`internal-read-git-doc-graphs`** **inside** existing files. **Implicit conventions** — for **`## Contents`**, **tables**, **overview** sections, and **diagrams**, apply **`internal-read-git-doc-index`**, **`internal-read-git-doc-table`**, **`internal-read-git-doc-explanation`**, and **`internal-read-git-doc-graphs`** rules **without** asking the user to cite each library.

## Do not

- Run **`internal-write-gh-evaluate`** — that is **`@git-review`**.
- **Commit, push, or GitHub PR mutations** — use **`@git-push`** / **`@gh-pr`**.
- **Promise** bulk doc-tree fixes from this skill alone.

---

## Verification

- [ ] User saw **mode** (diff-scoped vs wider) or explicit scope.
- [ ] **No-op path** noted when docs already match (**no** edits).
- [ ] **No structural** file operations this run (no new/deleted/moved **`.md`**).
- [ ] Tree / new-path needs **called out** in chat when appropriate—not silently “fixed” here.
- [ ] Caller directed to **`@git-review`** if tests are not green yet.

---

## Pack onboarding (human reference)

Use this section for quick onboarding to the skill pack. The operational rules above remain the contract for `@git-docs`.

### Install and verify

From repo root:

Useful options:

- `--clean` (wipes whole `CURSOR_SKILLS_DIR`, then reinstalls this pack), `--uninstall` (removes only this pack’s folders; confirm with `DELETE` or `CURSOR_SKILLS_UNINSTALL_CONFIRM=yes`), `--verify-only`, `--dry-run`, `--repo DIR`
- `CURSOR_SKILLS_DIR` to change destination (default `~/.cursor/skills`)

Run checks:

### Standard flow examples

- **Plan to issue:** Cursor Plan (optional) → chat / `.cursor/plan.md` (optional) → `@gh-issues`
- **Issue to PR:** `@gh-issue-view` -> `@git-start` -> `@gh-pr` (runs **`@git-pull`**, **`@git-review`**, **`@git-push`**, then PR text)
- **PR vs issue review:** `@gh-pr-view` + `@gh-issue-view`, then `@gh-issue-review` when reshaping is needed

### Execution hand-off pattern

1. Start from the tracked issue instead of raw chat.
2. Convert issue scope into ordered todos/checkpoints.
3. Recheck todos against issue acceptance criteria.
4. Implement, verify, and then publish.

### Model-tier guidance

| Phase | Typical model | Why |
| --- | --- | --- |
| Fast planning loops | `composer-2` | Lower-cost iteration while refining scope and examples. |
| Issue/body polish | `composer-2` | Good for tightening wording and acceptance criteria. |
| Implementation and verification | `codex-5.3` | Stronger multi-file execution and test handling. |

### Additional docs

- [`README.md`](../../../README.md) — concise visual entry point
- [`docs/README.md`](../../../docs/README.md) — docs hub
- [`docs/git.md`](../../../docs/git.md), [`docs/gh.md`](../../../docs/gh.md), [`docs/internal.md`](../../../docs/internal.md)

---

## See also

- [`internal-read-git-repo-classification`](../../read/git/repo-classification/SKILL.md) — optional repo-kind hints for doc inventory scope
- [`@git-review`](../review/SKILL.md) — verify + §8a light polish
- [`@git-push`](../push/SKILL.md) — publish orchestration (**`@git-commit`** when needed); does **not** bundle **`@git-docs`**
- [`internal-read-git-repo-layout`](../../read/git/repo-layout/SKILL.md) — **`skills/`** + **`docs/`** hub contract
- [`internal-read-git-doc-reference`](../../read/git/doc-reference/SKILL.md) · [`internal-read-git-doc-user-guide`](../../read/git/doc-user-guide/SKILL.md) — legacy **`REFERENCE.md`** / **`USER_GUIDE.md`** migration only
