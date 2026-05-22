---
name: git-zip
description: >-
  Orchestrate @git-tag (when a tag is needed) then export a GitHub-style .zip via internal-write-git-zip
  from refs/tags/TAG only, or export from an existing tag after read-only verification. No tag mutations
  in this skill. Runnable export blocks: internal-write-git-zip.
---

# `@git-zip` (label via `@git-tag`, then export `.zip` — confirm first)

Normative export fences: [`internal-write-git-zip`](../../write/git/zip/SKILL.md). Read-only tag checks (when verifying an existing tag): same **Inspect** shapes as [`internal-write-git-tag`](../../write/git/tag/SKILL.md) — **no** Create / Replace / Push from this skill.


**Public.** **Goal:** Produce **one** **`.zip`** file **like GitHub “Source code (zip)”** — **`git archive`** of **tracked files** at **`refs/tags/${TAG}`** only (default **`TAG=${DATE}`**, **no** **`.git`**, **no** untracked files), basename **`${PROJECT}-${DATE}.zip`**. This skill **orchestrates only**: hand off **tag create/replace/push** to **[`@git-tag`](../tag/SKILL.md)**, then run **`internal-write-git-zip`**. It does **not** run **`internal-write-git-tag`** **mutating** fences.

**Depends on [`@git-tag`](../tag/SKILL.md)** whenever the tag must be created, repointed, or pushed—**`@git-zip`** does not duplicate **`@git-tag`** Q&A or tag mutation steps.

**Normative shell** — **`git archive`**: **[`internal-write-git-zip`](../../write/git/zip/SKILL.md)**. Naming and flow: **[`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-zip-workflow)**. **Writes** outside the repo root by default (parent directory)—**[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** (**writes outside git workspace root**): **Proceed** must restate exact paths.

**Full clone backup** (includes **`.git`**) is **not** this skill’s default — see **`internal-write-git-zip`** **Optional full-tree backup** only if the user explicitly wants it.

---

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use **`@git-tag`** first when the export should follow a **new or repointed** date tag on synced **`main`**.
2. Use when the user already has **`refs/tags/${TAG}`** and only needs **read-only** verification plus **`.zip`** export.
3. Export runnable blocks: **`internal-write-git-zip`** only (names only; Execution batch runs them after **Proceed**).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file). **Do not** list **`internal-write-git-tag`** for **mutations**—tag writes run only under **`@git-tag`**.

1. `internal-write-plan-structured-qa`
2. `internal-write-git-zip`
3. `internal-read-git-workflows`
4. `internal-write-plan-skill-safety`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use release or handoff workflows that consume the produced **`.zip`** artifact.
2. If the tag was not pushed and should be on **`origin`**, run **`@git-tag`** (or complete an interrupted **`@git-tag`** run) before relying on a remote **`.zip`** mirror.

## Q&A bypass ENV

- `SKIP_QA_GIT_ZIP=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## On invoke

1. **Preconditions** — Run **Preconditions (read-only)** in **`internal-write-git-zip`** for **`TOP`**, **`PROJECT`**, **`DATE`**. Set **`DEST_DIR`** (default: parent of **`TOP`**). Set **`TAG="${DATE}"`** unless the user overrides **`TAG`** in chat; restate **`refs/tags/${TAG}`** as the export ref.
2. **Remote (optional)** — Run **Check `origin` exists** from **`internal-write-git-tag`** when you need **Inspect remote tag** or **Fetch remote tags** for the summary. Skip if the user will run **`@git-tag`** immediately (that skill performs its own **`origin`** checks).
3. **Tag inventory** — Run **Inspect local tag** for **`${TAG}`**. On the **existing-tag** path only, optionally run **Inspect remote tag** after **Check `origin` exists** succeeds.
4. **Summary block** (**structured-qa §1a**) — include **`TOP`**, **`DEST_DIR`**, default zip path **`${DEST_DIR}/${PROJECT}-${DATE}.zip`**, **`ARCH_REF=refs/tags/${TAG}`**, local tag line, and (if checked) whether the tag exists on **`origin`**.

---

## Q&A (required order)

### Round 0 — Export path

**Question:** “How should **`@git-zip`** obtain **`refs/tags/${TAG}`** before export?”

- **`Run @git-tag` then export `.zip`** (**default**) — Hand off fully to **`@git-tag`** (optional publish-first **`@git-push`**, **`@git-main`**, label, tag-push per that skill’s gates). After it completes, set **`ARCH_REF=refs/tags/${TAG}`** and run **Plain archive** from **`internal-write-git-zip`**.
- **`Export .zip from existing tag`** — User confirms **`TAG`** (explicit name; default **`${DATE}`**). Run **Inspect local tag** (and optional **Inspect remote tag**); **abort** if local tag is missing unless the user switches to **`Run @git-tag`**. **No** Create / Replace / Push from this skill. Set **`ARCH_REF=refs/tags/${TAG}`** and run **Plain archive** from **`internal-write-git-zip`**.
- **`Abort`**

### Round 1 — Proceed

Restate **`ARCH_REF`** (always **`refs/tags/${TAG}`**) and the zip path.

**Question:** “Ready to export **`.zip`**?”

- **`Abort`** — **Stop**; no export write.
- **`Review`** — Return to the **Summary block** (**On invoke** §4); adjust **`DEST_DIR`**, **`TAG`**, or Round 0 choice **without** running **Plain archive** yet.
- **`Proceed`** —
  - **`Run @git-tag` then export `.zip`** — Run **`@git-tag`** first. Then set **`ARCH_REF=refs/tags/${TAG}`** and run **Plain archive** from **`internal-write-git-zip`**.
  - **`Export .zip from existing tag`** — Set **`ARCH_REF=refs/tags/${TAG}`** after read-only verification. Run **Plain archive** from **`internal-write-git-zip`**.

---

## Do

- Show **real example paths** before **Proceed**.
- Follow **structured-qa** **§1a** → **§2** → **§5**.
- Keep **`ARCH_REF`** as **`refs/tags/${TAG}`** for **`@git-zip`** public paths.

## Do not

- **Branch push**, **commits**, **tag create/replace/push**, or **GitHub PR** mutations from this skill—use **`@git-tag`** / **`@git-push`** / **`@gh-pr`** as appropriate.
- **Embed** runnable **`bash`** fences here — use **`internal-write-git-zip`** only for **`git archive`** (and optional full-tree backup there if explicitly requested).
- Default to **full-tree `zip` including `.git`** unless the user explicitly asked for **Optional full-tree backup** in **`internal-write-git-zip`**.

---

## Verification

- [ ] User saw **example path** for **`.zip`**.
- [ ] **`ARCH_REF`** is **`refs/tags/${TAG}`** (matches Round 0 choice).
- [ ] Any **tag mutation** ran under **`@git-tag`**, not from **`@git-zip`**.
- [ ] **Proceed** was captured before **Plain archive** (including after any **Review** loop).

---

## See also

- [`@git-tag`](../tag/SKILL.md) — **tag-only** flow (**mutations** + **`@git-main`** prelude)
- [`internal-write-git-zip`](../../write/git/zip/SKILL.md) — **`git archive`** (**export** leaf)
- [`internal-write-git-tag`](../../write/git/tag/SKILL.md) — read-only **Inspect** / **Check `origin`** shapes for verification; **mutations** only when **`@git-tag`** runs these blocks
- [`internal-read-git-workflows`](../../read/git/workflows/SKILL.md#git-zip-workflow)
- [`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)
- [`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)
