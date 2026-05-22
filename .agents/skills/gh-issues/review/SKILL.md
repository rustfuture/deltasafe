---
name: gh-issues-review
description: >-
  GitHub issues (`gh issue view` / `gh issue list`): read-only — discover repo context, view target issue(s), list/search open issues for overlaps, compare like dedupe
  but for reshape; propose clearer title/body and AskQuestion on gaps. No GitHub writes—@gh-issues applies changes after
  confirm. Complements @git-review (code health) with issue clarity.
---

**Report shape:** [`REVIEW_REPORT.md`](../../../read/gh/issue-description/review-report/SKILL.md).

# GitHub: review / reshape issues (read-only)

Normative fences / full matrix: [`internal-write-gh-issue-commands`](../../../../write/gh/issue-commands/SKILL.md), [`internal-read-gh-issue-list`](../../../../read/gh/issue-list/SKILL.md).


**Public.** **Goal:** After a **quick** issue (e.g. from phone), **understand the codebase**, the **issue as written**, and **other open issues**—then produce an **overlap map**, **gap list**, and an **optional draft** title/body the user can apply with **`@gh-issues`**. Same spirit as **`@git-review`** (health pass) but for **issue quality and de-duplication**, **without** running format/lint/tests unless the user explicitly asks for full **`@git-review`**.

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-issue-view` first to lock scope on the target issue body/comments.
2. Use `@gh-issue-list` to capture nearby overlap context before reshape suggestions.
3. View/list command shapes: **`internal-read-gh-issue-list`** and **`internal-write-gh-issue-commands`** (names only; Execution batch runs them).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-git-discover-dependencies`
2. `internal-write-gh-issue-commands`
3. `internal-read-gh-issue-dedupe`
4. `internal-read-gh-issue-spec`
5. `internal-write-plan-structured-qa`
6. `internal-read-gh-issue-list`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. Route accepted wording changes to `@gh-issues` for create/edit mutation.
2. Use `@git-review` separately when code-health validation is also required.

## Q&A bypass ENV

- `SKIP_QA_GH_ISSUES_REVIEW=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

1. **Repo context (read-only)** — Run **[`internal-read-git-discover-dependencies`](../../../read/git/discover-dependencies/SKILL.md)** **in full** so the review can tie the issue to **real paths**, stacks, and CI/README signals. **Do not** run **`internal-write-gh-install-dependencies`**, **`internal-read-git-configuration`**, **`internal-write-gh-evaluate`**, or the rest of **`@git-review`** unless the user asks—this skill stays **issue-centric**.
2. **Target issue(s)** — Execute **View** in **[`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)**. Accept **`#N`**, URL, or a number from **`@gh-issue-pick`**. Pull **title**, **body**, **labels**, **assignees**, **state**, **URL**; add **`comments`** to the JSON set when the thread matters for scope.
3. **Peer inventory** — **List / search** in **`internal-read-gh-issue-list`** and **[`DEDUPE_CHECKLIST.md`](../../../read/gh/issue-dedupe/dedupe-checklist/SKILL.md)**. Cast a **wide enough net** (keywords from title/body, relevant labels, **`--limit`** / search query). **Read-only.**
4. **Overlap / duplicate check** — Compare the target to peers (titles + body leads). Optionally run **[`internal-read-gh-issue-dedupe`](../../../read/gh/issue-dedupe/SKILL.md)** using a **candidate** string equal to the **intended reshaped** title + body summary (or the **current** text when you are only cleaning wording). When the target is **`#N`**, **ignore self-match** on **`#N`**—treat other hits as **peer overlaps** (see **`internal-read-gh-issue-dedupe`** **reshape-review** note).
5. **Synthesis** — Map issue asks to **files / areas** surfaced in discovery; flag **underspecified** work (missing acceptance checks, unclear user impact, missing error handling, version assumptions). Score gaps against **[`internal-read-gh-issue-spec`](../../../read/gh/issue-spec/SKILL.md)** when the issue should be implementation-ready.
6. **Prompt when unclear** — Use **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** (**§1a** summary, **§2** short labels) for **priority**, **scope split vs absorb**, **duplicate resolution**, or **product intent** before locking a draft.
7. **Report** — Follow **[`REVIEW_REPORT.md`](../../../read/gh/issue-description/review-report/SKILL.md)** section order: **target snapshot**, **codebase fit**, **peer overlaps**, **gaps** (including issue-spec checklist), **proposed reshape** (optional).
8. **Hand-off** — **`@gh-issues`** applies the create-or-edit path in **`internal-write-gh-issue-commands`** after **`internal-read-gh-issue-description`** + user **Proceed**.

## Do not

- **Issue create, edit, comment, close, or delete** mutations inside this skill.
- Pretend overlap analysis is done without **listing** open issues at least once (unless the repo truly has **zero** other open issues).
- Skip **`internal-write-plan-structured-qa`** when the user’s goal is **ambiguous** and you would otherwise guess scope.

## On invoke

**`@gh-issue-review`** — **Fixed order:** **discover-dependencies** → **view target** → **list/search peers** → **overlap + gap synthesis** → **structured questions if needed** → **written report** (pasteable template). **Stop** on missing **GitHub CLI** auth or unknown repo context—tell the user to authenticate the CLI or set **`--repo`** per **`internal-read-gh-issue-list`** / **`internal-write-gh-issue-commands`** / **`internal-read-gh-repo-stream`**.

**Typical use:** “I filed **`#42`** on mobile—tighten it and see if we already track this elsewhere.”

## See also

- [`@gh-issue-view`](../view/SKILL.md) · [`@gh-issue-list`](../list/SKILL.md) · [`@gh-issue-pick`](../pick/SKILL.md)
- [`@gh-issues`](../SKILL.md)
- [`@git-review`](../../../git/review/SKILL.md) — workspace **code** health (separate concern)
- [`internal-read-gh-issue-spec`](../../../read/gh/issue-spec/SKILL.md)
- [`internal-read-gh-issue-dedupe`](../../../read/gh/issue-dedupe/SKILL.md) · [`internal-read-gh-issue-list`](../../../read/gh/issue-list/SKILL.md) · [`internal-write-gh-issue-commands`](../../../write/gh/issue-commands/SKILL.md)
