---
name: internal-write-git-commit
description: >-
  Runnable local git add/commit command blocks for @git-commit. Caller owns structured Q&A and Proceed before mutating
  steps. No push and no GitHub API.
---

# Internal: git commit commands (`internal-write-git-commit`)

**Library.** **Runnable** **`git add`** / **`git commit`** sequences for **`@git-commit`**. **Caller** owns **structured Q&A** and **Proceed** before mutating blocks.

## Preconditions (read-only)

```bash
TOP=$(git rev-parse --show-toplevel)
```

## Inspect working tree (read-only)

```bash
git -C "$TOP" status --short
```

## Stage tracked changes only (mutating)

```bash
git -C "$TOP" add -u
```

## Stage tracked and untracked changes (mutating)

**Default for [`@git-commit`](../../../git/commit/SKILL.md)** — use this block unless the caller’s **single gate** (or one follow-up) narrows scope to tracked-only or selected paths.

```bash
git -C "$TOP" add -A
```

## Stage selected paths (mutating)

Set **`PATHS`** as a shell-safe path list provided by the caller.

```bash
git -C "$TOP" add -- $PATHS
```

## Create commit (mutating)

Set **`COMMIT_SUBJECT`** before this block.

```bash
git -C "$TOP" diff --cached --quiet || git -C "$TOP" commit -m "$COMMIT_SUBJECT"
```

## See also

- [`@git-commit`](../../../git/commit/SKILL.md)
- [`@git-push`](../../../git/push/SKILL.md)
