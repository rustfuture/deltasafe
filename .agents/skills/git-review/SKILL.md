---
name: git-review
description: >-
  Make the current workspace healthy: internal-read-git-discover-dependencies → install-dependencies → configuration →
  evaluate (format, lint, tests; prefer fixing failing code and unformatted sources) → internal-write-gh-documentation (§8a).
  No git add, commit, or push. Command matrix in internal-write-gh-evaluate + internal-read-git-configuration.
---

# `@git-review` (workspace health → report)

Normative fences / full matrix: [`internal-read-git-discover-dependencies`](../../read/git/discover-dependencies/SKILL.md), [`internal-write-gh-evaluate`](../../write/gh/evaluate/SKILL.md).


**Public.** **Goal:** the tree **as it is now** works — dependencies installed, **sources formatted**, **lint clean**, **tests passing**, then **§8a** **light** polish of **existing** **`docs/**`** + minimal root **`README.md`** (parity with repo + CI—**not** bulk regen). **Not** staging, **commit**, or **publish**—use **`@git-commit`** and **`@git-push`** for savepoints and publish; **`@gh-pr`** runs this skill before **`@git-push`** when opening or updating a PR.

Run each **internal** skill below **in full**, in order. Details live in linked files—**not** duplicated here.

| Step | Internal skill |
| --- | --- |
| 1–2 | [`internal-read-git-discover-dependencies`](../../read/git/discover-dependencies/SKILL.md) |
| 3 | [`internal-write-gh-install-dependencies`](../../write/gh/install-dependencies/SKILL.md) |
| — | [`internal-read-git-configuration`](../../read/git/configuration/SKILL.md) |
| 4–7 | [`internal-write-gh-evaluate`](../../write/gh/evaluate/SKILL.md) |
| 8a | [`internal-write-gh-documentation`](../../write/gh/documentation/SKILL.md) |
| 9 | Report (this skill only) |

```mermaid
flowchart LR
  classDef intH fill:#004d40,color:#fff
  classDef int1 fill:#00695c,color:#fff
  classDef int2 fill:#00897b,color:#fff
  classDef int3 fill:#4db6ac,color:#111
  classDef intT fill:#b2dfdb,color:#111
  D[discover-deps] --> I[install-deps] --> C[configuration] --> E[evaluate]
  E --> DOC[documentation 8a]
  class D intT
  class I int3
  class C int2
  class E intH
  class DOC int1
```

**Failure:** Stop on first failure; **skip step 8a** if Evaluate failed.

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@git-pull` first when local branch might be behind upstream/main.
2. Use before publish workflows when workspace health needs a full verification pass.
3. Discover/install/evaluate stack: **`internal-read-git-discover-dependencies`** → **`internal-write-gh-install-dependencies`** (names only; Execution batch runs below).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-discover-dependencies`
2. `internal-write-gh-install-dependencies`
3. `internal-read-git-configuration`
4. `internal-write-gh-evaluate`
5. `internal-write-gh-documentation`
6. `internal-write-plan-skill-safety`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@git-docs` for broader post-green in-place docs accuracy updates.
2. Use `@git-push` when you are ready to publish verified changes.

## Q&A bypass ENV

- `SKIP_QA_GIT_REVIEW=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do not

- **Stage, commit, or push** — use **`@git-push`** when you want to publish from this workspace.
- Fetch / merge / reset — use **`@git-pull`**, **`@git-main`**, **`@git-reset`**.
- Skip internals or run checks outside **`internal-write-gh-evaluate`**.

**Also used by:** **`@gh-pr`** runs this **same** pipeline **before** **`@git-push`** in the PR chain—**`@git-review`** by itself does **not** stage, commit, or push.

## Report §9

Summarize: discovery, prepare, evaluate (pass/fail; call out **format fixes**, **lint fixes**, **test/code fixes** applied), **§8a** documentation pass (yes/no; paths touched briefly).

## Notes

- **Issue clarity** (reshape / duplicates / thin phone drafts) is **`@gh-issue-review`** + **`@gh-issues`**—**not** this pipeline unless you explicitly run **`@git-review`** for code health.
- **`internal-write-gh-install-dependencies`** (step 3) runs **destructive** wipes only after **structured confirm** (**[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** + **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** **§3a**).
- Standalone **`@git-push`** does **not** run this pipeline; ad-hoc checks there do **not** replace **`internal-write-gh-evaluate`** when the user asked for full verify.
- **`@gh-pr`** assumes **full** **`@git-review`** completed successfully **before** **`@git-push`** in its fixed order, then PR description work follows.
- **Bulk `docs/`** regen and stripping **`skills/**/README.md`** is **outside** this pack’s skills—do it in normal edits with **`internal-read-git-repo-layout`**, **`internal-read-git-readme-tree`** (read-only bootstrap notes), and **`internal-write-plan-skill-safety`** as needed. **§8a** is **light** polish on **existing** paths during verify. **Full** post-green doc accuracy (**in-place** only; may **no-op**) is **`@git-docs`** after green—**not** a substitute for green evaluate. This pipeline **does not** replace those skills.
