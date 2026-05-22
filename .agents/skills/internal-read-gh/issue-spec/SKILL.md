---
name: internal-read-gh-issue-spec
description: >-
  Read-only contract for high-quality implementation-ready issue bodies: required sections, quality bar,
  anti-ambiguity examples, non-regression expectations, and verification guidance.
---

# Internal: Issue spec contract

**Read-only library.** This is the canonical issue-body quality contract for this pack when a repository does not impose a stronger issue template.

## Do

- Draft issue bodies with these required sections:
  1. Goal/outcome.
  2. Current situation.
  3. Examples today vs desired.
  4. Scope of change.
  5. Non-regression/compatibility.
  6. Verification (acceptance + regression tests).
  7. Worked examples/edge cases.
  8. Risks/open questions.
- Prefer concrete examples over abstract prose.
- Preserve existing behavior requirements explicitly when extending a feature.
- Make verification executable by a future implementer without guessing.
- When planning surfaces **finite** implementation forks before publish, prefer **AskQuestion** (structured multiple-choice) per **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** **§1** so the user picks in the Q&A UI instead of parsing A/B/C prose alone; align pre-flight gates with **[`internal-read-gh-issue-preflight-qa`](../issue-preflight-qa/SKILL.md)**.

## Do not

- Publish vague issue bodies that omit examples or regression expectations.
- Treat this as a mutation workflow; publishing remains owned by `@gh-issues`.

## Quality bar (done criteria)

- Goal is specific enough that success can be judged.
- Current and desired behavior each include at least one example.
- Change surface is bounded (in scope/out of scope clear).
- Non-regression expectations are explicit.
- Verification includes both acceptance and regression intent.
- Major ambiguity points are resolved or documented as open questions.

## Exemplars

### Exemplar: extend calculator operations

- **Goal:** add `multiply` and `divide` while preserving `sum` and `subtract` behavior.
- **Current example:** `2 + 3 -> 5`, `8 - 3 -> 5`, `2 * 3 -> unsupported operation`.
- **Desired example:** `2 * 3 -> 6`, `8 / 2 -> 4`, `8 / 0 -> stable validation error`.
- **Scope:** parser/evaluator support for `*` and `/`, divide-by-zero guard, no API schema break.
- **Verification:** acceptance tests for multiply/divide, regression tests for sum/subtract, negative tests for divide-by-zero.
- **Open questions:** precision policy (float/decimal, rounding behavior).

### Exemplar: issue router upgrade

- **Goal:** when intent is vague, route through planning first, then run deterministic create-or-update.
- **Current example:** vague prompt jumps straight into create/update logic.
- **Desired example:** vague prompt -> Cursor Plan (or chat) -> stable issue artifact -> list -> dedupe -> edit/create.
- **Scope:** wording and routing updates only; no new API mutations; dedupe logic unchanged.
- **Verification:** dry-run both paths (vague intent path and ready artifact path) and validate links.
- **Open questions:** keep router wording concise while preserving clarity.

## See also

- [`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md) — **§1** finite choices and prompt shape when offering alternatives in Cursor.

## Consumers

- [`internal-read-plan-core-github-issue-profile`](../../plan/core/github-issue-profile/SKILL.md)
- [`@gh-issue-review`](../../../../gh/issues/review/SKILL.md)
- [`internal-read-gh-issue-description-issue-body-skeleton`](../issue-description/issue-body-skeleton/SKILL.md)
