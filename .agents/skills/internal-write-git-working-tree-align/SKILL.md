---
name: internal-write-git-working-tree-align
description: >-
  Read-only playbook: local git reset/clean (no gh). Dirty-tree confirm (no stash), discover clean targets, git clean
  dry-run, confirm and execute git reset --hard to a caller-supplied ref and git clean. Optional non-git prunes table.
  Callers: @git-main (MODE=combined), @git-reset (MODE=split). Does NOT fetch, checkout branches, or resolve ALIGN_REF.
---

# Internal: Working tree align (reset + clean)

**Read-only library.** **Local `git` only** (no **`gh`**). Parent skills set **`$ALIGN_REF`** (resolved ref string, e.g. **`upstream/main`**, **`origin/feature-x`**) and **`$BRANCH`** (current branch name for messages). This library does **not** fetch, **not** `git checkout`, and **not** compute **`$ALIGN_REF`**—use **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)** + parent workflow for that.

**Prompts** — apply **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** + **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** (**§3a**: **Goal** + **AskQuestion**, **Abort** / **Proceed**). **Never** **`git stash`**.
**Master-confirm rule:** **`SKIP_QA_*`** and **`SKIP_QA_WRITE`** do not bypass session-level confirm for **`git reset --hard`**, **`git clean`**, or optional global prune commands.

---

## Parameters (set by caller)

| Parameter | Meaning |
| --- | --- |
| **`$ALIGN_REF`** | Tip to match after **`git reset --hard`** (already fetched and valid). |
| **`$BRANCH`** | Current branch name (for **Goal** lines). |
| **`MODE`** | **`combined`** (**`@git-main`**) — one confirm may cover **reset + clean** (split if dry-run huge). **`split`** (**`@git-reset`**) — **separate** confirm before reset and before clean. |

---

## 1. Dirty tree (no stash)

**When:** **`git status --short`** is **not** empty **before** reset/clean (caller skips this block if the tree is already clean).

1. Show **`git status --short`** (and a one-line summary; for **`@git-reset`**, include modified / untracked / staged counts).
2. **Stop** and **AskQuestion** per **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** **§3a** with a **Goal** line chosen by the **caller**:
   - **`@git-main`** style: align local **`main`** to **`$ALIGN_REF`** and remove untracked/ignored files per planned **`git clean`**—destroying uncommitted work.
   - **`@git-reset`** style: align branch **`$BRANCH`** to **`$ALIGN_REF`** and run the planned **`git clean`**—permanently discarding local diffs.
3. Options:
   - **Keep work / abort** — end the **parent** skill; user should commit, copy files aside, or use another workflow.
   - **Trash and align** — user explicitly accepts destruction; continue to **§2**.
4. **Do not** stash. **Do not** offer stash as a third path (**`@git-reset`**: if vague, default **abort** until **trash and align** is explicit).

---

## 2. Discover — what `git clean` may touch

**Read-only.** Scan the worktree (repo root; monorepo hints from README if obvious). **Brief** bullets: stack signals and **large** paths likely listed by **`git clean -fdxn`**.

Common ignored / untracked targets (examples):

- **Node:** `node_modules/`, `dist/`, `build/`, `.next/`, `coverage/`
- **Python:** `.venv/`, `__pycache__/`, `.pytest_cache/`, `.ruff_cache/`
- **Rust:** `target/`
- **Go:** `vendor/` or untracked binaries
- **JVM / .NET:** `target/`, `build/`, `bin/`, `obj/`
- **Terraform:** `.terraform/`

For richer stack detection patterns, align with **`@git-review`** / **[`internal-read-git-discover-dependencies`](../../read/git/discover-dependencies/SKILL.md)** when helpful.

---

## 3. Clean dry-run

Before any **`git clean`** that removes files:

```bash
git clean -fdxn
```

Show output (truncate if enormous: counts + top directories). If the user prefers **keeping ignored** files, plan **`git clean -fd`** / **`git clean -fdn`** instead and say so before confirm.

---

## 4. Confirm and execute

### MODE `combined` (`@git-main`)

- State **`git reset --hard "$ALIGN_REF"`** + the **`git clean`** command from **§3** (**`-fdx`** default, or **`-fd`** if keeping ignored).
- **One** **Goal** + **AskQuestion** (**Abort** / **Proceed**) may cover **both** reset and clean when **§1** was **trash and align** or only untracked/ignored remain. **Split** into **two** confirms if the dry-run is **large** or **surprising**.
- **Only after Proceed:**  
  - **`git reset --hard "$ALIGN_REF"`**  
  - **`git clean -fdx`** (or **`git clean -fd`** if keep-ignored)

**Do not** run **`git pull`** after this block (caller rule).

### MODE `split` (`@git-reset`)

1. **Reset confirm** — State branch **`$BRANCH`** and **`git reset --hard "$ALIGN_REF"`**. Remind that **tracked** files match **`$ALIGN_REF`** and uncommitted **tracked** edits are discarded (**§1** already handled consent when dirty). **Goal** + **AskQuestion** (**Abort** / **Proceed**).
2. **Execute reset** — Only after Proceed: **`git reset --hard "$ALIGN_REF"`** (stay on **`$BRANCH`**).
3. **Clean confirm** — Remind: **`git clean -fdx`** (or **`git clean -fd`**) matches **§3** dry-run. **Goal** + **AskQuestion** (**Abort** / **Proceed**).
4. **Execute clean** — Only after Proceed: **`git clean -fdx`** or **`git clean -fd`**.

---

## 5. Optional — non-git / global caches

**Only** if the user **explicitly** asked (**`@git-main`**) or per table (**`@git-reset`**). **Each** command: **own** **Goal** + **AskQuestion** (**Abort** / **Proceed**). Skip if not applicable.

| Topic | When to offer | Example | Confirm |
| --- | --- | --- | --- |
| **Docker** | `Dockerfile` / compose present **or** user asked | `docker builder prune -f` | **Goal** + **AskQuestion** before run |
| **Docker (aggressive)** | User explicitly asks | `docker system prune` / `-a` | **Goal** + **AskQuestion** (explicit **Proceed**) |
| **Podman** | Containerfile / user asked | `podman builder prune`, `podman system prune` | Same as Docker |
| **npm (global cache)** | User asked | `npm cache clean --force` | **Goal** + **AskQuestion** |
| **Playwright** | User asked; Playwright in deps | User cache under **`$HOME`** | **Goal** + **AskQuestion** before delete |
| **Rust / extra** | User asks | project script / third-party tool | **Goal** + **AskQuestion** per command |
| **Other** | README, Makefile, user names a tool | project-specific | **Goal** + **AskQuestion** per command |

---

## Do not (library)

- **`git stash`**
- **`git push`**
- Fetch or resolve **`$ALIGN_REF`** (caller’s responsibility)

## See also

- **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)** — **`CANONICAL_MAIN_REF`**, **`$TARGET`** priority (**`@git-reset`** §4).
- **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** — reset/clean triggers.
