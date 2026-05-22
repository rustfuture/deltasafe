---
name: internal-read-gh-pr-preflight-qa
description: >-
  Read-only question bank for @gh-pr pre-flight. Ensures source/target branch intent, readiness, and
  overlap handling are clarified before create/edit mutations.
---

# PR pre-flight Q&A

Use this before PR create/edit mutation.

## Gather (deducible first)

- Source branch (`HEAD`) and target base branch.
- Existing matching PRs by head/base/title.
- Repo stream context (same-repo vs fork).

## Clarify (ask when ambiguous)

- Should this update an existing PR or open a new PR?
- If existing PR matches, which PR number should be edited?
- Is branch state ready to ship (green checks expected)?
- Any explicit reason to keep draft/WIP instead of ready-for-review?

## AskQuestion shape

- Summarize match evidence in the prompt text.
- Offer safe-first choices: Abort / Edit existing / Create new.
- Use one refinement outlet only.

## See also

- [`internal-read-gh-pr-list`](../pr-list/SKILL.md)
- [`internal-read-gh-pr-description`](../pr-description/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
- [`@gh-pr`](../../../../gh/pr/SKILL.md)
