---
name: gh-projects-order-readonly
description: >-
  GitHub Projects (`gh project item-list`): read-only GitHub Project item list + dependency layering (body-based edges), parallel tracks, cycles, and unresolved
  refs. Shapes: internal-read-gh-project-item-list. No GitHub mutations.
---

# GitHub: project item order (read-only)

Normative fences, JSON normalization, edge grammar, and layering algorithm: **[`internal-read-gh-project-item-list`](../../../read/gh/project-item-list/SKILL.md)**.

## Before batch (public, optional, sequential)

Run these only when they improve context for this invocation:

1. Use `@gh-issue-view` when the user needs one issue’s detail before trusting order output on that row.
2. Use when the user wants a **recommended execution order** or **parallel tracks** for a **GitHub Project** board (issues + other items), without changing GitHub state.
3. Resolve **`OWNER`** and project **`NUMBER`** first (title match via **`internal-read-gh-project-list`** / user-supplied values).

## Execution batch (internal, sequential)

Run this internal sequence in order (no runnable command fences in this public file):

1. **`internal-read-gh-project-list`** — when project **number** is unknown.
2. **`internal-read-gh-project-item-list`** — list items, normalize, parse edges, emit **layers / cycles / unresolvedEdges / skippedNonIssues** per v1 contract.

## After batch (public, optional, sequential)

1. Offer **`@gh-issue-view`** on a chosen issue from layer 0 when the user wants detail before branching.
2. Offer **`@git-start`** when the user is ready to implement from an issue anchor.

## Q&A bypass ENV

- `SKIP_QA_GH_PROJECT_ORDER_READONLY=true` bypasses routine Q&A for this specific public skill.
- Default behavior is unset/false, which keeps normal Q&A active.
- Shared `SKIP_QA_WRITE=true` does **not** authorize GitHub mutations here—this skill stays read-only.

## Do

- Run **`gh project item-list`** and analysis per **[`internal-read-gh-project-item-list`](../../../read/gh/project-item-list/SKILL.md)**; summarize **layers**, **cycles**, and **unresolved** refs clearly.
- Prefer **`-L 200`** (or documented cap) and say when output may be **truncated**.
- Treat **PRs/drafts** as **inventory-only** for v1 edge parsing (see internal library).

## Do not

- Reorder, archive, delete, or edit project items via **`gh project item-*`** mutations from this skill.
- Invent dependency edges not covered by the **v1 body line prefixes** in the internal library.

## See also

- [`internal-read-gh-project-list`](../../../read/gh/project-list/SKILL.md)
- [`internal-read-gh-issue-projects-relationships`](../../../read/gh/issue-projects-relationships/SKILL.md)
- [`@gh-project-delete-closed`](../delete-closed/SKILL.md) — separate mutating cleanup lane (owner scope + delete only)
