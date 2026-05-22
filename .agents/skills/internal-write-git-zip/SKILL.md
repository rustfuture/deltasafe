---
name: internal-write-git-zip
description: >-
  Normative local shell: default git archive (GitHub-style zip, tracked tree only, no .git); optional full-tree zip
  including .git for rare full-clone portable backups. No gh / GitHub API. Caller runs Q&A first; this library holds
  runnable command blocks. Consumed by @git-zip.
---

# Internal: git zip commands (`internal-write-git-zip`)

**Library.** **Runnable** **`git archive`** (default). **Caller** ([`git-zip`](../../../git/zip/SKILL.md)) owns **structured Q&A** and **Proceed** before invoking these steps via **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)**. **Tag** create/replace/push is owned by **[`@git-tag`](../../../git/tag/SKILL.md)** via **`internal-write-git-tag`**; **`@git-zip`** calls **`@git-tag`** when needed, then sets **`ARCH_REF=refs/tags/${TAG}`** and invokes this file for export only.
**Master-confirm rule:** **`SKIP_QA_*`** and **`SKIP_QA_WRITE`** do not bypass session-level confirm for out-of-repo archive writes or optional full-tree backup writes.

**Default behavior:** **`git archive --format=zip`** — same class of artifact as GitHub **"Source code (zip)"** for a ref (**no** **`.git`**, **no** untracked files). Primary caller **`@git-zip`** sets **`ARCH_REF=refs/tags/${TAG}`** (default **`TAG=${DATE}`**) after **`@git-tag`** or after read-only tag verification. Other callers may set **`ARCH_REF`** themselves (including **`HEAD`**) only with their own Q&A and scope.

## Preconditions (read-only)

Resolve repo root and naming facts **before** the Q&A rounds in the public skill. **Do not** write archives until **Proceed** after path/tag confirmation.

```bash
TOP=$(git rev-parse --show-toplevel)
PROJECT=$(basename "$TOP")
DATE=$(date +%Y-%m-%d)
```

Export **`TOP`**, **`PROJECT`**, **`DATE`**. Choose **`DEST_DIR`** with the user (default: parent of **`TOP`**). **Caller** sets **`ARCH_REF`**:

- **`refs/tags/${TAG}`** — snapshot of the **annotated tag** (**`@git-zip`** public paths always use this after **`@git-tag`** or existing-tag verification; default **`TAG=${DATE}`**).
- **`HEAD`** — only for **non-`@git-zip`** callers that explicitly export the current commit with their own confirmation.

## Variables

Substitute **`TOP`**, **`DEST_DIR`**, **`DATE`**, **`PROJECT`**, **`ARCH_REF`**. Output basename: **`${PROJECT}-${DATE}.zip`**.

## Plain archive (default — contents only)

**Tracked files at `ARCH_REF` only** — **no** **`.git`**.

```bash
mkdir -p "$DEST_DIR"
OUT="${DEST_DIR}/${PROJECT}-${DATE}.zip"
git -C "$TOP" archive --format=zip --output "$OUT" "$ARCH_REF"
```

Optional **top-level folder inside the zip** (closer to GitHub's **`owner-repo-ref/`** layout):

```bash
git -C "$TOP" archive --format=zip --prefix="${PROJECT}/" --output "$OUT" "$ARCH_REF"
```

## Optional full-tree backup (includes `.git`)

**Not** **`@git-zip`** default — portable **clone** of the working tree **including** **`.git`** (large). Only when the user explicitly asks for **full repository** backup, not GitHub-style.

```bash
cd "$TOP"
mkdir -p "$DEST_DIR"
OUT="${DEST_DIR}/${PROJECT}-${DATE}-full.zip"
zip -qr "$OUT" . -x "*.zip"
```

## See also

- [`@git-tag`](../../../git/tag/SKILL.md) — tag create/replace/push (**mutations**); run before **`@git-zip`** export when the tag is not already settled
- [`internal-read-git-workflows`](../../../read/git/workflows/SKILL.md#git-zip-workflow) — naming + **GitHub vs full-tree**
- [`@git-zip`](../../../git/zip/SKILL.md) — public orchestration: **`@git-tag`** (when needed), then **`git archive`** at **`refs/tags/${TAG}`**
- [`internal-write-git-tag`](../tag/SKILL.md) — **Create** / **Replace** / **Push** (**`@git-tag`** only); read-only **Inspect** shapes for **`@git-zip`** verification on the existing-tag path
