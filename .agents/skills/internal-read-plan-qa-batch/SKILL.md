---
name: internal-read-plan-qa-batch
description: Read-only builder for structured AskQuestion batches in planning.
---

# Q&A batch builder

- Build finite-choice AskQuestion rounds with short labels.
- When planning alternatives are a **finite** set, surface them as **AskQuestion** options per **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** **§1**, not as prose-only lists the user must parse in chat.
- Include one freeform outlet only when needed.
- Keep prompts aligned with `internal-write-plan-structured-qa`.
