---
name: git-tag
description: >-
  Optional full @git-push before @git-main when origin exists and the repo is dirty, ahead of
  upstream, or the user wants publish-first; then sync main, annotated yyyy-mm-dd tag, and push tag
  to origin with fewer Q&A gates. Local tag only is an explicit opt-out.
---

# `@git-tag` (optional publish-first, sync main, label, push tag — confirm first)

Normative fences / full matrix: [`internal-write-git-tag`](../../write/git/tag/SKILL.md).


**Public.** **Goal:** **Persist** an **annotated** tag **`refs/tags/${TAG}`** (default **`${TAG}`** = **`${DATE}`**, **ISO yyyy-mm-dd**, local timezone) at **`GIT_TAG_TARGET`** (default **`HEAD`**) after synchronizing local **`main`**, then **publish** that tag to **`origin`** when **`origin`** exists and the user confirms—only after **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** and explicit **Proceed**. **Local tag only** is an explicit **opt-out**. This skill **owns** all **tag mutations** (**keep vs replace** local, **skip vs force** remote tag push). **`@git-zip`** does **not** run tag mutating fences; it **hands off** here when a tag must be created or repointed, then exports a **`.zip`** via **`internal-write-git-zip`**. **Collisions:** if the tag **already exists locally**, confirm **keep vs replace** before mutations. If **`origin`** already has the tag and the user wants **push**, confirm **skip vs force**. If **`origin`** is **missing**, **local tag only** still succeeds.

**Normative shell** — **[`internal-write-git-tag`](../../write/git/tag/SKILL.md)**. **Tag push** is **`git push`** — **[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** (**online mutation**): **Goal + summary + Proceed** before any **Push** block (normal or **force**).

**Public vs internal tree (this skill as root)**

- **Public children (orchestration):** **[`@git-push`](../push/SKILL.md)** (**conditional** — run **full** hand-off when **publish-first** read-only checks fire; see **On invoke**), **[`@git-main`](../main/SKILL.md)** (sync local `main` to canonical remote), **[`@git-commit`](../commit/SKILL.md)** (**optional** — when the tree is dirty, **`origin` is missing** or the user needs a savepoint **without** running **`@git-push`** first).
- **Optional user-run sibling:** **[`@git-reset`](../reset/SKILL.md)** when you explicitly need **hard discard** alignment on the current branch before switching to `main`—invoke it yourself; this skill does **not** embed reset fences (reset stays a **leaf** public skill).
- **Internal leaves:** **`internal-write-git-tag`** (Create / Replace / Push fences), plus **`internal-write-plan-structured-qa`**, **`internal-write-plan-skill-safety`** in the Execution batch below.

---

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use **`@git-main`** alignment context when the tag should be anchored from synced **`main`**.
2. Use **`@git-push`** when branch work may need verify+publish **before** tagging (see **On invoke** publish-first check).
3. Tag **mutation** fences: **`internal-write-git-tag`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-write-plan-structured-qa`
2. `internal-write-git-tag`
3. `internal-write-plan-skill-safety`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use **`@git-zip`** when the tagged tree should be exported as a **GitHub-style** **`.zip`** (after this skill completes; **`@git-zip`** orchestrates **`internal-write-git-zip`** only for export).

## Q&A bypass ENV

- `SKIP_QA_GIT_TAG=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## On invoke

1. **Preconditions** — Run **Preconditions (read-only)** in **`internal-write-git-tag`**. If **`git rev-parse`** fails, **stop**. Default **`TAG="${DATE}"`**; if the user chose a different tag name in **chat** for this run, restate it and ensure the agent **`export TAG=...`** before **Inspect** / **Create** / **Replace** / **Push** blocks so fences resolve **`TAG="${TAG:-$DATE}"`**.
2. **Branch gate** — Read current branch first. If not on `main`, run structured Q&A: **Abort**, **Commit current branch work first (`@git-commit`) then continue**, or **Continue without commit**.
3. **Publish-first read-only check** — Evaluate **without mutating**: does **`git remote get-url origin`** succeed? Is **`git status --short`** non-empty **or** (when upstream exists) is **`HEAD`** **ahead** of **`@{u}`** (`git rev-list --left-right --count HEAD...@{u}` left count > 0) **or** did the user explicitly ask to **publish before tag**?
   - **If `origin` exists** **and** any of the above is true → run **full [`@git-push`](../push/SKILL.md)** once **before** **`@git-main`**. If **`@git-push`** fails, **stop** (do not tag on a failed publish leg).
   - **Else** → skip **`@git-push`** with a one-line reason (clean / no remote / nothing ahead).
4. **Preflight savepoint (when step 3 skipped)** — If the working tree is still dirty and the user needs a local savepoint before syncing **`main`**, run **`@git-commit`** (no push).
5. **Sync prelude** — Run full **[`@git-main`](../main/SKILL.md)** once so local `main` matches canonical remote **`main`** in this standalone workflow.
6. **Read-only inventory** — Run **Inspect local tag**, and if **`origin`** resolves run **Inspect remote tag**; if **`origin`** is missing, note tag push is unavailable without adding a remote (not a hard error for local-only). Then run **Inspect previous reachable tag** and **Inspect commits since previous tag**.
7. **Goal line** — One sentence (e.g. anchor synced `main` **`HEAD`** with **`TAG`**, **recommended:** publish the tag to **`origin`** when the remote exists).
8. **Summary block** (**structured-qa** **§1a**) — Bullets: **`TOP`**, current **`BRANCH`**, **`TAG`**, **`GIT_TAG_TARGET`** (default **`HEAD`**), local exists/missing (+ short SHA if present), remote line or “no **`origin`**” / “remote tag missing”, previous tag marker, and a capped commit list (**`${PREV_TAG}..HEAD`** when available).

**Post-merge note:** If the tag must point at **released** **`main`**, ensure **`origin/main`** already contains the merge (PR merged or equivalent) **before** step 5—otherwise **`@git-main`** aligns to **`main`** without your local feature commits.

---

## Q&A (fewer gates)

### Gate 1 — Tag publish intent

**Question:** “Publish tag to **`origin`** when the remote exists (**recommended**), or **local tag only**?”

- **`Local tag + push to origin`** (**recommended** when **`origin`** exists) — After **Gate 4 — Proceed**, ensure local tag then push per **Gate 3** / non-force as applicable. If **`origin`** is **missing** and the user chose this option, run structured triage per **`structured-qa` §8**: **Abort**, **Continue as local tag only**, or **Pause and configure `origin`, then retry**.
- **`Local tag only`** — Local tag steps only; **never** push the tag.
- **`Cancel`**

### Gate 2 — Local tag already exists (skip if inspect shows **local:missing**)

**Question:** “Tag exists locally. **Keep** or **repoint** to **`GIT_TAG_TARGET`**?”

- **`Keep existing`** — No local tag mutation this run (skip **Create** / **Replace**).
- **`Replace — repoint tag (force local)`** — After **Gate 4**, run **Replace local annotated tag (force)** in **`internal-write-git-tag`**.
- **`Cancel`**

### Gate 3 — Remote tag + push (skip unless Gate 1 = **`Local tag + push to origin`** **and** **Inspect remote tag** shows **`refs/tags/${TAG}`**)

**Question:** “Remote already has this tag. **Skip push** or **force push**?”

- **`Skip push`**
- **`Force push — overwrite remote tag`** — After **Gate 4**, run **Push one tag to `origin` (force)**.
- **`Cancel`**

If the remote **does not** have the tag yet, **skip Gate 3**; after local ensure, use **Push one tag to `origin`** (non-force) when Gate 1 included push.

### Gate 4 — Proceed (when Gate 1 ≠ **`Cancel`** and no prior **Cancel**)

Restate **`TAG`**, **`GIT_TAG_TARGET`**, which **Create** / **Replace** / **Push** / **Push (force)** steps will run, and **`origin`** URL if pushing.

**Question:** “Run selected tag actions now?” — **`Cancel`** / **`Proceed`**

**`Proceed`** — Run **`internal-write-git-tag`** in order (skip steps the user declined):

1. **Local** — If **local missing**: **Create annotated tag only if missing**. If **local exists** and **Gate 2** = **Replace**: **Replace local annotated tag (force)**. If **local exists** and **Gate 2** = **Keep**: skip local mutating blocks.
2. **Push** (only when Gate 1 = **`Local tag + push to origin`** and **`origin`** exists) — If **Gate 3** = **Force push**: **Push one tag to `origin` (force)**. Else if **Gate 3** = **Skip push**: skip. Else (**remote tag missing**): **Push one tag to `origin`**.

---

## Do

- Use **date-only** tag names by default: **`yyyy-mm-dd`**, aligned with **`internal-write-git-tag`** defaults.
- Verify **`origin`** with **read-only** **`git remote get-url origin`** before any **push** summary claims a publish path.
- Run **Gate 2** whenever the local tag exists **before** **Gate 4**, so the user picks **keep** vs **replace**.
- When **`@git-push`** ran in **On invoke** step 3, do **not** duplicate branch publish logic here—this skill handles **tag** push only after **`@git-main`**.

## Do not

- Run **Replace local** or **Push … (force)** without an explicit user choice in **Gate 2** / **Gate 3**.
- Skip the ordered prelude (**branch gate → optional `@git-push` → optional `@git-commit` → `@git-main`**) unless the user explicitly asks for a different flow.
- **Embed** runnable **`bash`** fences in **this** file beyond what authoring needs — use **`internal-write-git-tag`** only.

---

## Verification

- [ ] Preconditions and default **`TAG`** (or override) were shown.
- [ ] Prelude (**publish-first `@git-push`**, optional **`@git-commit`**, **`@git-main`**) completed or explicitly skipped with reason.
- [ ] **`origin`** was checked before offering tag **push**.
- [ ] Previous tag + commit summary was shown before **Gate 4**.
- [ ] **Gate 2** ran when local tag pre-existed; **Gate 3** ran when push + remote tag existed.
- [ ] **Gate 4** **Proceed** / **Cancel** recorded before **Create** / **Replace** / **Push** blocks.

---

## See also

- [`internal-write-git-tag`](../../write/git/tag/SKILL.md) — **Run** blocks (**mutations**; **`@git-tag`** only)
- [`@git-push`](../push/SKILL.md) — optional **publish-first** prelude when checks fire
- [`@git-commit`](../commit/SKILL.md) — local savepoint when **`@git-push`** did not run and the tree is dirty
- [`@git-main`](../main/SKILL.md) — align local `main` before tagging
- [`@git-reset`](../reset/SKILL.md) — optional **user-run** hard alignment (**leaf** skill)
- [`@git-zip`](../zip/SKILL.md) — **`@git-tag`** then **`internal-write-git-zip`**
- [`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)
- [`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)
