---
name: internal-read-plan-parts
description: Read-only planner for splitting one goal into disjoint domain plan parts.
---

# Plan parts

- Partition multi-goal requests into independent sections in `.cursor/plan.md`.
- Keep each part with goal, scope, verification, and hand-off.
- When needed, move large parts into `.cursor/plans/<slug>.md` linked from the hub.

**Consumers:** **`@gh-issues`** uses these boundaries for the optional [Multi-intent clustering](../../../gh/issues/SKILL.md#multi-intent-clustering-optional) path: each **disjoint** part maps to **at most one** shipped issue after per-theme dedupe (or merges into an **edit #n** row when overlap wins). Parts are not a substitute for user confirmation on split vs merge.
