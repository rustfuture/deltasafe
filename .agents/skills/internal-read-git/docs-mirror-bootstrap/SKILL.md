---
name: internal-read-git-docs-mirror-bootstrap
description: >-
  Read-only: bootstrap docs/ mirror pages from README inventory using canonical internal doc libraries.
---

# Internal: docs mirror bootstrap

**Read-only.** Use this when creating a docs mirror for the first time or after a large layout change. This is not part of `@git-review` and does not replace in-place updates from `@git-docs`.

## Inputs

- Repository root.
- Scope (`skills/` subtree only, or full repository).

## Steps

1. Enumerate candidate `README.md` files in the chosen scope.
2. Decide which paths should have a docs mirror page.
3. Map each source path to a `docs/<parallel-path>/README.md` target.
4. Draft each target page using canonical doc libraries:
   - [`internal-read-git-doc-index`](../doc-index/SKILL.md)
   - [`internal-read-git-doc-explanation`](../doc-explanation/SKILL.md)
   - [`internal-read-git-doc-table`](../doc-table/SKILL.md) when reference tables are needed
5. Apply wiki/layout constraints from:
   - [`internal-read-git-repo-layout`](../repo-layout/SKILL.md)
   - [`internal-read-git-doc-index`](../doc-index/SKILL.md)
6. For diagram-heavy pages, use [`internal-read-git-doc-graphs`](../doc-graphs/SKILL.md) and child templates.

## Do not

- Treat this as continuous mirror regeneration.
- Replace `@git-docs` or `internal-write-gh-documentation` for routine in-place doc alignment.
