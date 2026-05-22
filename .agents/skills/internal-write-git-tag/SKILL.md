---
name: internal-write-git-tag
description: >-
  Normative local git tag + tag push commands (check origin, ISO date yyyy-mm-dd tag name); optional replace local
  annotated tag (force) and optional force-push to origin when the caller's Q&A records explicit override. Mutating
  fences are run only under @git-tag (Proceed there). @git-zip may use read-only blocks here for verification before
  internal-write-git-zip. No gh / GitHub API. Caller runs structured Q&A and Proceed before mutating fences.
---

# Internal: git tag commands (`internal-write-git-tag`)

**Library.** **Runnable** **`git tag`** / **`git push`** **mutating** sequences are owned by **[`@git-tag`](../../../git/tag/SKILL.md)**—that public skill runs **Create** / **Replace** / **Push** after **Gate 2** / **Gate 3** / **Gate 4 — Proceed**. **[`@git-zip`](../../../git/zip/SKILL.md)** does **not** invoke these mutating blocks; it **orchestrates** **`@git-tag`** when a tag must be created or repointed, and may use **read-only** sections of this file (**Check `origin` exists**, **Inspect local tag**, **Inspect remote tag**, **Fetch remote tags**) to verify **`refs/tags/${TAG}`** on the **existing-tag** export path before **`internal-write-git-zip`**.

**Caller** (always **`@git-tag`** for **mutations**) owns **structured Q&A** and **Proceed** before any mutating block using **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)**. **Never** run **Replace local** or **Push ... (force)** unless the user explicitly chose those options in **`@git-tag`** **Gate 2** / **Gate 3**. When **`@git-tag`** runs **Push**, do so **before** any follow-on **`@git-zip`** **Plain archive** so **`origin`** matches the tagged tree being zipped.

**Master-confirm rule:** **`SKIP_QA_*`** and **`SKIP_QA_WRITE`** do not bypass session-level confirm for tag replacement, remote tag pushes, or force pushes.

## Preconditions (read-only)

Resolve **before** user confirm rounds.

```bash
TOP=$(git rev-parse --show-toplevel)
DATE=$(date +%Y-%m-%d)
```

**Default tag:** **`TAG="${DATE}"`** unless the caller **`export TAG=...`** before read/mutate blocks (then use that **`TAG`**). Blocks below resolve **`TAG="${TAG:-$DATE}"`**.

**Optional target ref** (default **`HEAD`**): export **`GIT_TAG_TARGET=HEAD`** or another commit-ish before **Create** / **Replace** blocks.

## Check `origin` exists (read-only)

Use **before** promising **`git push origin ...`**. Non-zero exit if **`origin`** is missing.

```bash
git remote get-url origin
```

## Fetch remote tags (read-only; requires `origin`)

Run before selecting or comparing tags on **`origin`** when `origin` exists.

```bash
git fetch origin --tags --prune
```

## Inspect local tag (read-only)

```bash
TAG="${TAG:-$DATE}"
if git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null 2>&1; then
  echo "local:refs/tags/${TAG}=$(git rev-parse "refs/tags/${TAG}^{}")"
else
  echo "local:missing refs/tags/${TAG}"
fi
```

## Inspect remote tag (read-only; requires `origin`)

Fails if **`origin`** is absent—caller should have run **Check `origin` exists** first.

```bash
TAG="${TAG:-$DATE}"
git ls-remote --tags origin "refs/tags/${TAG}"
```

Empty output => no tag with that exact name on **`origin`** yet.

## Inspect latest local tag (read-only)

Use when a caller needs a **latest** local tag name for display or manual **`TAG=`** choice (not a silent default for **`@git-zip`**).

```bash
LATEST_LOCAL_TAG="$(git tag --list --sort=-v:refname | sed -n '1p')"
if [[ -n "$LATEST_LOCAL_TAG" ]]; then
  echo "latest_local_tag:$LATEST_LOCAL_TAG"
else
  echo "latest_local_tag:none"
fi
```

## Inspect previous reachable tag (read-only)

Use after branch/main alignment to summarize candidate release history.

```bash
PREV_TAG=$(git describe --tags --abbrev=0 2>/dev/null || true)
if [[ -n "$PREV_TAG" ]]; then
  echo "prev_tag:$PREV_TAG"
else
  echo "prev_tag:none"
fi
```

## Inspect commits since previous tag (read-only)

Set **`LOG_LIMIT`** if needed (default **25**). If there is no previous tag, show the latest commits from **`HEAD`**.

```bash
LOG_LIMIT="${LOG_LIMIT:-25}"
if [[ -n "${PREV_TAG:-}" ]]; then
  git log --oneline "${PREV_TAG}..HEAD" | sed -n "1,${LOG_LIMIT}p"
else
  git log --oneline HEAD | sed -n "1,${LOG_LIMIT}p"
fi
```

## Create annotated tag only if missing (mutating)

**Only after** caller **`@git-tag`** **Proceed**. **Idempotent** first-time create. **No-op** when **`refs/tags/${TAG}`** already exists — use **Replace local annotated tag (force)** when the user chose to repoint the tag.

```bash
cd "$TOP"
TAG="${TAG:-$DATE}"
GIT_TAG_TARGET="${GIT_TAG_TARGET:-HEAD}"
if ! git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null 2>&1; then
  git tag -a "$TAG" -m "$TAG" "$GIT_TAG_TARGET"
fi
```

## Replace local annotated tag (force) (mutating)

**Only after** caller **`@git-tag`** **Gate 4 — Proceed** and an explicit **"replace local"** / **"force local"** choice (**Gate 2**). Moves **`refs/tags/${TAG}`** to **`GIT_TAG_TARGET`** (annotated).

```bash
cd "$TOP"
TAG="${TAG:-$DATE}"
GIT_TAG_TARGET="${GIT_TAG_TARGET:-HEAD}"
git tag -fa "$TAG" -m "$TAG" "$GIT_TAG_TARGET"
```

## Push one tag to `origin` (mutating)

**Only after** caller **`@git-tag`** **Proceed** for **push** when the remote **does not** already have this tag **or** it matches what you are publishing without needing **`--force`**. Requires **`origin`**.

```bash
TAG="${TAG:-$DATE}"
git push origin "refs/tags/${TAG}:refs/tags/${TAG}"
```

If this fails because **`origin`** has a different object for **`${TAG}`**, the caller should have offered **force push** per **`@git-tag`** **Gate 3** and then run **Push one tag to `origin` (force)** instead.

## Push one tag to `origin` (force) (mutating)

**Only after** caller **`@git-tag`** **Gate 4 — Proceed** and an explicit **"force push"** / **"overwrite remote tag"** choice (**Gate 3**). **Destructive** on **`origin`**: replaces the remote tag ref. Requires **`origin`**.

```bash
TAG="${TAG:-$DATE}"
git push --force origin "refs/tags/${TAG}:refs/tags/${TAG}"
```

## See also

- [`internal-write-git-zip`](../zip/SKILL.md) — zip archive naming (**`${PROJECT}-${DATE}.zip`**) is separate from **tag** name (**`${TAG}`** / default **`${DATE}`**)
- [`@git-tag`](../../../git/tag/SKILL.md) — public orchestration (**mutations**)
- [`@git-zip`](../../../git/zip/SKILL.md) — orchestrates **`@git-tag`** then **`internal-write-git-zip`**; **read-only** use of this file for verification on the existing-tag path only
