---
name: internal-read-plan-core-turn-order
description: >-
  Read-only turn-order guidance for planning conversations: choose the next question that most reduces delivery
  ambiguity while keeping prompts short and decision-focused.
---

# Turn order

1. Clarify goal and user-visible outcome.
2. Capture current behavior (facts, examples, constraints).
3. Capture desired behavior and acceptance target.
4. Confirm change surface and non-goals.
5. Clarify alternatives and decision rationale (if forks exist).
6. Define verification and regression expectations.
7. Gather edge cases/worked examples.
8. Surface remaining risks or unknowns.

When step 5 surfaces a **finite** set of forks (for example two or three concrete approaches), prefer **AskQuestion** with short labels per **[`internal-write-plan-structured-qa`](../../../../write/plan/structured-qa/SKILL.md)** **§1** instead of listing A/B/C only in chat prose.

When multiple unknowns exist, ask the question that most reduces implementation risk first.
Use repeated one-question refinement loops until readiness gates pass or the user chooses to stop.

## See also

- [`../readiness-checklist/SKILL.md`](../readiness-checklist/SKILL.md)
- [`../../../../write/plan/structured-qa/SKILL.md`](../../../../write/plan/structured-qa/SKILL.md)

- Insert a domain classifier step early (docs, unit-tests, integration, bug, enhancement, dependencies) so subsequent questions use the right template.
