---
name: internal-read-gh-issue-labels-label-decorate
description: >-
  Read-only: gh label list/match/create command shapes. Parent internal-read-gh-issue-labels.
---

# Issue labels — decorate (examples)

Used by **[`internal-read-gh-issue-labels`](./SKILL.md)** and **`@gh-issues`**.

**Inventory (read-only):**

```bash
gh label list --json name,description,color --limit 200
```

---

## Example A — Map to existing labels

**Input (chat):** “Add label guidance for skills pack; touch `skills/read/gh/` and templates.”

**Draft title:** `Document issue label heuristics for skill repos`

**Draft body (excerpt):** “… acceptance: label list command, 0–3 labels, confirm before `gh label create` …”

**Step:** Existing labels include `documentation`, `enhancement`.

**Output (proposal table):**

| Candidate | Action | GitHub label |
| --- | --- | --- |
| docs / templates | reuse | `documentation` |
| feature work | reuse | `enhancement` |

**Attach (preview, not run by agent until confirm):** use the canonical create shapes in **[`internal-write-gh-issue-commands`](../../../../write/gh/issue-commands/SKILL.md#create-issue-after-proceed)** and pass `--label "documentation,enhancement"` after the structured proceed gate.

---

## Example B — One new label, then attach

**Input:** “Track flaky CI on `gh-pr` skill; file an issue.”

**Draft title:** `Flaky CI when running gh-pr orchestration tests`

**Existing labels:** `bug`, `ci` (no `area:gh`).

**Step:** Propose **`ci`** + **`bug`**; suggest **`area:gh`** as **new** (repo has other `area:*` labels).

**Output:**

| Candidate | Action | Notes |
| --- | --- | --- |
| CI | reuse | `ci` |
| defect | reuse | `bug` |
| domain | **create** | `gh label create area:gh --color ededed --description "GitHub PR CLI skills"` → **structured confirm** first |

**Then create issue:** use the same canonical create shapes in **[`internal-write-gh-issue-commands`](../../../../write/gh/issue-commands/SKILL.md#create-issue-after-proceed)** with `--label "ci,bug,area:gh"` after confirm.

---

## Do not

- Attach **more than three** labels without explicit user ask.
- Create labels that only duplicate an existing name with different spelling.
