---
name: internal-read-plan-core-github-issue-profile
description: >-
  Read-only profile for planning issue-ready artifacts: goal, current vs desired behavior, alternatives,
  scope/change surface, non-regression, verification, and ambiguity-killing examples.
---

# Profile: github issue

Use when the final output should be ready for **`@gh-issues`**.

## Required sections

- Goal and intended outcome.
- Current behavior with concrete examples.
- Desired behavior with concrete examples.
- Scope and expected change surface.
- Compatibility and non-regression expectations.
- Verification approach (acceptance + regression tests).
- Worked examples and edge cases.
- Risks/open questions and alternatives (when meaningful forks exist).

## Output shape

- **Title:** concise statement aligned with **[`internal-read-gh-pr-content/title-line`](../../../gh/pr-content/title-line/SKILL.md)** rules.
- **Body:** follow **[`internal-read-gh-issue-spec`](../../../gh/issue-spec/SKILL.md)** and compatible skeleton guidance in **[`internal-read-gh-issue-description-issue-body-skeleton`](../../../gh/issue-description/issue-body-skeleton/SKILL.md)**.

## See also

- [`internal-read-gh-issue-spec`](../../../gh/issue-spec/SKILL.md)
- [`internal-read-gh-issue-description`](../../../gh/issue-description/SKILL.md)
