---
name: internal-read-gh-pr-content-pr-body-skeleton
description: >-
  Read-only: PR body markdown skeleton. Parent internal-read-gh-pr-content. Optional minimal Refs/Fixes line only when
  @gh-pr §9 confirmed keyword path; no default Linked issues block.
---

# Canonical internal scaffold

This file is the canonical internal reference for PR body skeleton guidance.

## Skeleton (canonical / normalized)

Use **[`internal-read-gh-pr-body-sections/section-patterns`](../pr-body-sections/section-patterns/SKILL.md)** for heading names. Typical flow: **TL;DR** → **What changed** (nested bullets) → tradeoffs / risks.

**Issue linkage:** omit a dedicated **Linked issues** heading. If **`@gh-pr`** **§9** confirmed the **keyword path**, append **at most one line** at the end of the body, for example `Refs #42` or `Fixes #42` (never multiple **`Fixes`** without explicit user confirmation). Otherwise rely on **Projects** (**`gh project item-add`**) and/or **UI** per **`internal-write-gh-pr-commands`**.
