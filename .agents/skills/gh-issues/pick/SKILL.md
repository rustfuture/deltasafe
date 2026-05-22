---
name: gh-issues-pick
description: >-
  GitHub issues (`gh issue list` / `gh issue view`): list open issues with filters, then structured AskQuestion so the user picks the next task; read-only until user
  choice. Shapes: internal-read-gh-issue-list. Suggest @gh-issue-view and @git-start after selection.
---

# GitHub: pick next issue

Normative fences / full matrix: [`internal-read-gh-issue-list`](../../../../read/gh/issue-list/SKILL.md).


## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Run after `@gh-issue-list` when you already have filtered candidate issues.
2. Use `@gh-issue-view` first when issue details must be expanded before selection.
3. List/pick normative fences: **`internal-read-gh-issue-list`** (names only; Execution batch runs it).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. `internal-read-gh-issue-list`
2. `internal-write-plan-structured-qa`

## After batch (public, optional, sequential)

Choose follow-up based on outcome:

1. After selection, route details to `@gh-issue-view` for richer context.
2. Then move to `@git-start` when implementation should begin on a branch.

## Q&A bypass ENV

- `SKIP_QA_GH_ISSUES_PICK=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` can bypass routine write-flow Q&A where the owning workflow allows it.
- High-risk or destructive confirmations still require explicit user confirmation.

## Do

1. **Goal** — One line: what “next” means (e.g. highest priority **`bug`**, unblocked **`help wanted`**, etc.).
2. **Inventory (read-only)** — Run **List / search** blocks from **[`internal-read-gh-issue-list`](../../../read/gh/issue-list/SKILL.md)** with user-supplied **`--label`**, **`--assignee`**, **`--milestone`**, or search text. Cap **`--limit`** (e.g. **30**).
3. **Present candidates** — Short table: **#n — title — labels** for the top set.
4. **AskQuestion** — **Short labels** for **top N** issues plus **`Refine filters`** / **`Cancel`**. This step is **choice UX** only—**no** GitHub mutations here.
5. **After selection** — If the user picked an issue: run **`@gh-issue-view`** on that **#n**; suggest **`@git-start`** with the **title** when branching. If **Refine filters**, return to step **2** with updated filters.

## Do not

- Create, edit, close, or delete issues from this skill.

## See also

- [`internal-read-gh-issue-list`](../../../read/gh/issue-list/SKILL.md)
- [`@gh-issue-list`](../list/SKILL.md)
- [`@gh-issue-view`](../view/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
