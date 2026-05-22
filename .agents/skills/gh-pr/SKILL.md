---
name: gh-pr
description: >-
  Pull requests (`gh pr` create/edit on GitHub). Order: full @git-pull → full @git-review → full @git-push → PR lookup +
  user intent gate → internal-read-gh-pr-description (§5–7: templates, delta, API-first linking + issue scan, title/body)
  → §9 confirmed PR create/edit and optional gh project item-add. Uses internal-read-gh-repo-stream;
  internal-write-plan-structured-qa before GitHub mutations. Does NOT merge PR on GitHub.
---

**Orchestration:** [`PR_ORCHESTRATION.md`](../../read/gh/pr-content/pr-orchestration/SKILL.md).

# PR

Normative fences / full matrix: [`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md), [`internal-write-gh-pr-commands`](../../write/gh/pr-commands/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Run **`@git-pull`** to refresh branch/base context before drafting PR intent.
2. Run **`@git-review`** so the workspace is green before publish and PR text.
3. Run **`@git-push`** so local commits reach the remote before PR drafting.
4. PR description + command shapes: **`internal-read-gh-pr-description`** and **`internal-write-gh-pr-commands`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-gh-pr-preflight-qa`
2. `internal-read-gh-pr-list`
3. `internal-read-gh-pr-description`
4. `internal-read-gh-pr-content`
5. `internal-read-git-git-diff-summary`
6. `internal-read-gh-repo-stream`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Use `@gh-pr-view` to validate created/updated PR state and metadata.
2. Use `@gh-pr-close` only when the user explicitly requests closure.

## Q&A bypass ENV

- `SKIP_QA_GH_PR=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

- Run PR pre-flight requirement gathering using **[`internal-read-gh-pr-preflight-qa`](../../read/gh/pr-preflight-qa/SKILL.md)** before PR lookup/mutation.
- Run **full** **[`@git-pull`](../../git/pull/SKILL.md)** first; it owns sync/conflict mechanics and does not publish.
- Run **full** **[`@git-review`](../../git/review/SKILL.md)** next; it owns install, evaluate (format/lint/test), and §8a docs polish—**no** commit or push.
- Run **full** **[`@git-push`](../../git/push/SKILL.md)** next; it owns commit-if-needed, branch gate, and publish (plus push-failure recovery).
- Resolve open-PR inventory and existing-PR house style using **[`internal-read-gh-pr-list`](../../read/gh/pr-list/SKILL.md)** and **[`LIST_VIEW_HUB.md`](../../read/gh/pr-list/list-view-hub/SKILL.md)** (hub only), then run structured user intent selection before setting **`PR_NUM`**.
- Run **[`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md)** **in full** (**§5–§7** there: templates, §6 delta, **§6.5–6.6** API-first linking ladder + issue hint scan, **§7** title/body + triple pass). **Shape:** **[`internal-read-gh-pr-content`](../../read/gh/pr-content/SKILL.md)** (**[`TITLE_LINE.md`](../../read/gh/pr-content/title-line/SKILL.md)**, **[`BODY_SKELETON.md`](../../read/gh/pr-content/pr-body-skeleton/SKILL.md)**); **diff clustering:** **[`internal-read-git-git-diff-summary`](../../read/git/git-diff-summary/SKILL.md)** + **[`diff-summary.md`](../../read/git/git-diff-summary/delta-narrative/SKILL.md)**.
- Apply one confirmed PR create/edit action per safety + structured-qa.
- Set destination, target repo, and head fields per **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)**. Draft body from the full branch delta produced by **internal-read-gh-pr-description**.

## Do not

- Create or edit a PR before **`@git-pull`** + **`@git-review`** + **`@git-push`** succeed.
- Inline verification commands instead of the ordered **`@git-review`** step (full skill).
- Merge the PR on GitHub from this skill—title/body only.
- **`@gh-pr`** when **`BRANCH`** is **`main`** on same-repo with no fork—sync only; **stop** (no PR).
- Run PR mutations inside **internal-read-gh-pr-description** — only this public skill applies the final confirmed action.

**Chain (fixed order):**

1. **[`@git-pull`](../../git/pull/SKILL.md)** — complete sync/conflict pass; no publish.
2. **[`@git-review`](../../git/review/SKILL.md)** — full workspace verify (evaluate + §8a); no commit or push.
3. **[`@git-push`](../../git/push/SKILL.md)** — inventory, **`@git-commit`** when needed, branch gate, **`git push`**.
4. **This skill continues** — PR lookup, draft title/body through internals, then one confirmed create/edit.

**Destination and PR targets:** See **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)** for destination, target repo, base branch, and PR head rules.

## On invoke

*`@gh-pr`* — **Order is fixed:** **`@git-pull`** → **`@git-review`** → **`@git-push`** → list matching open PRs via internals → **AskQuestion intent gate** (summary in prompt text: **Abort** / **Proceed** / one refine outlet) → resolve **`PR_NUM`** → **[`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md)** (templates → §6 delta → **§6.5–6.6** linking ladder + issue hint scan → **§7** title/body + triple pass) → **§9** final confirmed apply (PR create/edit summary must include **linking ladder**: candidate issues, **project `item-add`** vs **minimal keyword line** vs **UI-only**, and **`project` number/owner** when applicable) → optional same-batch **`gh project item-add`** per **`internal-write-gh-pr-commands`** when **§9** confirmed. **Never** create a PR to discover duplicates. Editing overwrites title/body with fresh content; when **`PR_NUM`** is set, treat the current PR body as a house-style hint only (see **[Existing PR as template](../../read/gh/pr-description/SKILL.md#existing-pr-as-template)**).

**Diff scope:** The PR narrative must reflect **everything** different between **destination** and **current `HEAD`** (merge-aware file list), **not** a subset tied to the “last PR update.”

**After sync/publish:** Draft title/body **only after** **`@git-push`** so local **`HEAD`** matches the remote when a push ran. Delta mechanics belong to **[`internal-read-git-git-diff-summary`](../../read/git/git-diff-summary/SKILL.md)**.

**If the diff looks wrong:** re-review fork vs same-repo classification in **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)**. Re-run **full** **`@git-pull`**, **`@git-review`**, and **`@git-push`**, then continue from PR lookup.

**“Since last commit on destination”:** use the full **`HEAD`** delta after the destination tip; **internal-read-git-git-diff-summary** owns the exact ranges.

## Workflow

### 0. Classify fork vs same-repo and set variables

*`@gh-pr`* — Apply **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)** before PR lookup and drafting. Do **not** re-derive fork/upstream rules inline—keep them consistent with **`@git-main`** / **`@git-pull`** via that library.

If fork intent is clear but the upstream remote is missing, run triage per **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** **§8**: **Abort**, **Reclassify intent as same-repo and continue with that target**, or **Pause and configure `upstream`, then retry `@gh-pr`**.

### 1. Branch and head ref

*`@gh-pr`* — Use **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)** for branch/head classification. Same-repo **`main`** syncs only and stops after pull/review/push legs; feature branches and fork branches continue to PR lookup/drafting with the head values from that library.

### 1.5 Pre-flight requirements (required)

*`@gh-pr`* — Run **[`internal-read-gh-pr-preflight-qa`](../../read/gh/pr-preflight-qa/SKILL.md)** to confirm source/target branch intent, overlap handling preference (edit existing vs create new), and readiness expectations before any create/edit path.

### 2. Hand off to **`@git-pull`** (required)

> Run the full Cursor skill **[`@git-pull`](../../git/pull/SKILL.md)** end-to-end. It owns fetch/merge/conflict details through **[`internal-read-git-merge-conflicts`](../../read/git/merge-conflicts/SKILL.md)**.
> If **`@git-pull`** errors, the user aborts, or conflicts stay unresolved → **stop** **`@gh-pr`** here.

### 3. Hand off to **`@git-review`** (required)

> Run the full Cursor skill **[`@git-review`](../../git/review/SKILL.md)** end-to-end. It owns discovery, install, evaluate, and §8a documentation polish—**not** staging, commit, or push.
> If **`@git-review`** fails, the user aborts, or the tree is not green → **stop** **`@gh-pr`** here (do not **`@git-push`** broken work).

### 4. Hand off to **`@git-push`** (required; owns push recovery)

> Run the full Cursor skill **[`@git-push`](../../git/push/SKILL.md)** end-to-end. It owns working-tree inventory, optional **`@git-commit`**, branch gate, publish confirmation, and **`git push`** behavior.
> **`@git-push`** owns branch gate + push-failure recovery. If it exits without a successful publish (Abort or unrecoverable error), **stop** **`@gh-pr`** here. If publish succeeds after recovery on a new branch, continue and use the updated branch/head values from **`internal-read-gh-repo-stream`**.

### 5. Resolve existing open PR (before create or before drafting)

*`@gh-pr`* — **Run immediately after §5**, **before** **[`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md)** and before body drafting. Use only **[`internal-read-gh-pr-list`](../../read/gh/pr-list/SKILL.md)** command fences + head/base/repo fields from **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)** to list matching open PRs.

**If zero matches** — keep **`PR_NUM`** empty and continue to **§6–8** for a create path.

**If one or more matches** — **do not** assume **`PR_NUM`**. Run **AskQuestion** with the compact inventory summary embedded in the prompt text (at minimum: count, `#n` + title lines, head→base, target repo). Keep options short and safe-first:

- **Abort** — stop `@gh-pr`; no drafting and no mutation.
- **Proceed** — choose edit/create intent. For one match, label directly (for example, **Proceed — edit #n** / **Proceed — create new**). For multiple matches, either include one **Proceed — edit #n** option per PR (within sane count) or run one additional narrow AskQuestion to pick the PR.
- **Refine** — one outlet only per **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** §5 (prefer `additionalSuggestion` *or* one trailing chat line, not both).

If user selects edit, set **`PR_NUM`** to the selected PR. If user selects create, leave **`PR_NUM`** empty. Mention in the summary that GitHub may reject a duplicate open PR for the same head→base.

### 6–8. Templates, delta, linking ladder + issue scan, title, body, triple pass

*`@gh-pr`* — Read and execute **[`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md)** **in full** (§§5–7, including **§6.5–6.6** and **triple pass**), including **Material template reshape** when a **single** repo template would otherwise be altered by **[merge rules](../../read/gh/pr-description/SKILL.md#merge-rules-all-sources)** without an explicit user pick. Produce finalized **`--title`** and **`--body`** (or body file) for **§9** below.

### 9. Apply — `edit` or `create` (one), optional project attach

*`@gh-pr`* — **After** the triple pass in **internal-read-gh-pr-description**, **before** any PR mutation: run **AskQuestion** where the prompt text itself contains the pending mutation summary (goal, create vs edit, repo, head/base, title line, that body will be replaced, **candidate issues from §6.6 with evidence**, and the chosen **linking ladder**: **project** **`item-add`** (project number + owner) vs **minimal `Refs`/`Fixes`/`Closes` line in body** vs **UI-only** for Development links). Use **Abort** first, **Proceed** to execute exactly as summarized, and one refinement outlet per **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** §5. This final confirm remains required by **[`internal-write-plan-skill-safety`](../../write/plan/skill-safety/SKILL.md)** PR mutation rows. Apply exactly one **`gh pr create`/`edit`** only after **Proceed**. When **§9** confirmed **project attach**, run **`gh project item-add`** for the PR URL and each issue URL in the **same Proceed batch** per **[`internal-write-gh-pr-commands`](../../write/gh/pr-commands/SKILL.md)**. Optionally run **`gh pr view --json closingIssuesReferences`** there to verify keyword-based inference.

---

## Notes

*`@gh-pr`*
- **Prerequisites:** authenticated GitHub CLI and repo root.
- **Fork:** `upstream` remote required.
- **Order:** **`@git-pull`** → **`@git-review`** → **`@git-push`** → PR lookup/style → **[`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md)** → confirmed apply.
- **Push recovery ownership:** branch gating and push-failure triage live in **`@git-push`**. Do not duplicate those prompts in **`@gh-pr`**.
- **Decision points:** if open PR matches exist, prompt for intent in **§5** before drafting; always run final mutation confirm in **§9** before `gh pr create` / `gh pr edit`.
- **PR body:** PR lookup resolves **`PR_NUM`**; **[`internal-read-gh-repo-stream`](../../read/gh/repo-stream/SKILL.md)** supplies target repo and destination. **Multiple** templates → **[`internal-write-plan-structured-qa`](../../write/plan/structured-qa/SKILL.md)** (no auto-pick). **TL;DR** first, **nested** “what changed,” **list** tradeoffs—**no** **commit counts**, **review route**, or compare-metadata in the posted body. **Issue traceability:** follow **[`internal-read-gh-pr-description`](../../read/gh/pr-description/SKILL.md)** §6.5–7—**no** default “Linked issues” prose section; optional **minimal keyword line** or **`gh project item-add`** only after **§9** user choice (see **`internal-write-gh-pr-commands`**).

## See also

- **[`@gh-pr-skills`](skills/SKILL.md)** — skill-pack PR narrative and when to use list/view vs **`@gh-pr`**.
- **[`@gh-pr-list`](list/SKILL.md)** · **[`@gh-pr-view`](view/SKILL.md)** — read-only PR list/view (**[`internal-read-gh-pr-list`](../../read/gh/pr-list/SKILL.md)**; hub **[`LIST_VIEW_HUB.md`](../../read/gh/pr-list/list-view-hub/SKILL.md)**).
- **[`@gh-pr-close`](close/SKILL.md)** — close PRs with structured confirm.
- **[`internal-read-gh-pr-preflight-qa`](../../read/gh/pr-preflight-qa/SKILL.md)** — pre-flight question bank for branch/overlap ambiguity.

### Hand off (outside **`@gh-pr`**)

> To **only** sync with **`main`** without opening/updating a PR, run **`@git-pull`** then **`@git-push`** alone. **`@gh-pr`** is for when you also want the **GitHub PR** updated from the **full** branch vs **destination** diff.

- For complex PR writing, stage temporary draft fragments under `.cursor/gh/pr/` before final create/edit confirmation.
