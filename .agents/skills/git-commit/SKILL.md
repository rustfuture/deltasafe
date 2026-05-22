---
name: git-commit
description: >-
  Local savepoint: read-only inventory first; if nothing to commit, no-op without Q&A. Otherwise one Proceed for
  stage (default git add -A) + commit message + mutating add/commit. No git-review, no docs pass, no push.
---

# `@git-commit` (local savepoint commit, no push)

Normative fences / full matrix: [`internal-write-git-commit`](../../write/git/commit/SKILL.md).


**Public.** **Goal:** record a local commit with **one** explicit **Cancel / Proceed** before mutating, without invoking **`@git-review`**, **`@git-docs`**, or **`@git-push`**.

**Normative shell** — **[`internal-write-git-commit`](../../write/git/commit/SKILL.md)**.

**Default scope:** Unless the user chooses **tracked-only** or **selected paths** in the **single** gate below, staging uses **`git add -A`** at **`TOP`** (full working tree under the repo root; **`.gitignore`** still applies)—the **Stage tracked and untracked** block in **`internal-write-git-commit`**.

---

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-review` first when you need verification context before saving.
2. Use after local edits when you need a savepoint before alignment workflows.
3. Savepoint commit shapes: **`internal-write-git-commit`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-write-git-commit`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-push` when you are ready to publish (orchestrated commit-if-needed + push).
2. Use `@git-tag` for release tagging after a confirmed savepoint.

## Q&A bypass ENV

- `SKIP_QA_GIT_COMMIT=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## On invoke

1. **Preconditions** — Run **Preconditions (read-only)** in **`internal-write-git-commit`**.
2. **Inventory** — Run **Inspect working tree (read-only)** and summarize staged/unstaged/untracked state.
3. **Nothing to commit** — If the working tree is **clean** (nothing to stage or commit: no short-status lines that imply pending work), **stop** with a one-line no-op explanation. **Do not** run **AskQuestion**.
4. **Single gate — Stage + message + Proceed** (**AskQuestion**, only when step 3 does not apply) — One prompt that includes:
   - **Branch context:** If current branch is **`main`** **and** **`git remote get-url origin`** succeeds, **warn** that **`@git-push`** will steer toward **feature branch + PR** when **`origin`** exists; **recommend** **[`@git-start`](../start/SKILL.md)** or **`git switch -c …`** before committing on **`main`** unless the user explicitly wants a **`main`** savepoint.
   - **Staging:** Default **`git add -A`** at **`TOP`**. Options in **the same** question: **Abort**, **Proceed — full tree (`git add -A`)**, **Tracked only (`git add -u`)**, **Selected paths** (user names **`PATHS`**; you **export** or restate them before **Proceed**).
   - **Message:** Confirm **`COMMIT_SUBJECT`** (user supplies or confirms in-chat before **Proceed**).
   - **Proceed** — Restate staging mode, **`COMMIT_SUBJECT`**, and that **Create commit** runs immediately after.
5. **Mutations** — Run the matching staging block from **`internal-write-git-commit`**, then **Create commit (mutating)**. If the user chose **Selected paths**, run **Stage selected paths** after **`PATHS`** is set.

**Narrow scope follow-up:** If **Selected paths** or **`add -u`** needs clarification (paths ambiguous), **one** follow-up question is allowed—do **not** restart a multi-round ladder.

---

## Do

- Use this skill for local savepoints before destructive/sync workflows.
- Keep commit messages short and intent-oriented.
- Exit early with a no-op when inventory shows nothing to commit.

## Do not

- Claim this commit is verified/green; green is **`@git-review`**, and **`@gh-pr`** runs **`@git-review`** before **`@git-push`** when crafting a PR.
- Push from this skill.
- Present this skill as a way to **bypass** **`@git-push`** branch policy when **`origin`** exists.

---

## Verification

- [ ] **Inventory** ran; **no-op** or **single gate** path is documented in the result.
- [ ] **Proceed** / **Abort** captured before mutating commands (when step 4 ran).
- [ ] Result includes either commit SHA or a no-op explanation.

---

## See also

- [`internal-write-git-commit`](../../write/git/commit/SKILL.md)
- [`@git-push`](../push/SKILL.md)
- [`@git-tag`](../tag/SKILL.md)
