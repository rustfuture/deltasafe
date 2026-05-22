---
name: internal-read-plan-core
description: >-
  Read-only planning library: profile selection, turn order, readiness gates, and artifact shape for issue-ready or
  generic execution briefs. Consumed from @gh-issues pre-flight and related planning chat (no public @plan-* skill).
---

# Internal: Agent planning library

**Read-only library.** This is the single owner for planning profiles, question ordering, and readiness criteria used from **`@gh-issues`** pre-flight / **Sharpen** rounds (and other callers that embed the same libraries).

## Do

- Select one profile first:
  - **[`github-issue-profile`](./github-issue-profile/SKILL.md)** for tracked work that should become a GitHub issue.
  - **[`generic-brief-profile`](./generic-brief-profile/SKILL.md)** for non-GitHub planning artifacts.
- Default to `.cursor/plan.md` as the primary artifact; split oversized domains to `.cursor/plans/<slug>.md` and link them from the primary file.
- Drive the conversation with **[`turn-order`](./turn-order/SKILL.md)** so each turn resolves one high-impact unknown.
- Validate completion with **[`readiness-checklist`](./readiness-checklist/SKILL.md)** before returning the final artifact.
- Keep `.cursor/plan.md` as the single canonical hub when using file-backed plans; if split files are needed, link all sub-plans from the hub.
- Use confidence/definition-of-done checks from [`internal-read-plan-confidence`](../confidence/SKILL.md) before handing off to **`@gh-issues`** publish steps or **`@git-start`**.
- For disjoint goals, partition with [`internal-read-plan-parts`](../parts/SKILL.md).
- Use **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** for finite choices and confirms inside planning interactions.

## Do not

- Run `gh` or `git` commands.
- Perform any mutation flow (issue create/edit, branching, commits, PR creation).
- Duplicate issue shape rules that are owned by **[`internal-read-gh-issue-spec`](../../gh/issue-spec/SKILL.md)**.

## On invoke

Profile pick → question turn order → readiness gates → emit stable artifact + hand-off.

## See also

- [`github-issue-profile`](./github-issue-profile/SKILL.md)
- [`generic-brief-profile`](./generic-brief-profile/SKILL.md)
- [`turn-order`](./turn-order/SKILL.md)
- [`readiness-checklist`](./readiness-checklist/SKILL.md)
- [`internal-read-gh-issue-spec`](../../gh/issue-spec/SKILL.md)
- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)
- [`internal-read-plan-parts`](../parts/SKILL.md)
- [`internal-read-plan-confidence`](../confidence/SKILL.md)
