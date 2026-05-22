---
name: internal-write-gh-issue-commands-close-as-duplicate
description: >-
  Read-only: close issue as duplicate (gh shapes). Parent internal-write-gh-issue-commands.
---

# Close as duplicate

GitHub does not merge issue bodies; use **duplicate** semantics.

**After user Proceed:**

1. Comment on the duplicate with a link to the canonical issue: `gh issue comment <dup> --body "Duplicate of #<canonical> …"`.
2. Close the duplicate: `gh issue close <dup>` (or `gh issue close <dup> --reason "not planned"` if your `gh` version supports reason flags—match local `gh issue close --help`).

Replace `<dup>` / `<canonical>` with numeric ids. **Confirm before** any `gh issue comment` / `gh issue close`.
