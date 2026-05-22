---
name: internal-read-plan-core-readiness-checklist
description: >-
  Read-only readiness checklist for plan completion: minimum quality gates before handing artifacts to issue
  publish or execution workflows.
---

# Readiness checklist

Use before returning a final artifact.

- [ ] Goal is explicit and testable.
- [ ] Primary artifact path is explicit (default `.cursor/plan.md` unless user chose another path).
- [ ] Current behavior is described with at least one concrete example.
- [ ] Desired behavior is described with at least one concrete example.
- [ ] Scope and non-goals are explicit.
- [ ] Plan has clear domain sections with small, independent tasks.
- [ ] Ship mode is explicit (work now vs defer/create issue).
- [ ] Non-regression expectations are explicit.
- [ ] Verification includes acceptance and regression intent.
- [ ] Open questions and assumptions are listed.
- [ ] If choices exist, alternatives and decision rationale are captured.
- [ ] Out-of-scope ideas are either deferred into sub-plans or queued for an issue.

If any gate is missing, ask a focused follow-up before finalizing.

If you need a stable output shape, follow [`generic-brief-profile`](../generic-brief-profile/SKILL.md) or [`github-issue-profile`](../github-issue-profile/SKILL.md).

- [ ] Confidence threshold is met (goal, scope, and verification are explicit enough to execute).
