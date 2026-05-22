---
name: internal-read-git
description: >-
  Internal git-oriented read-only libraries for discovery, docs conventions, repo layout, and merge/diff playbooks.
---

# `skills/read/git/`

Read-only library layer consumed by public `@git-*` and `@gh-*` workflows.

Docs map: [`docs/read.md`](../../../docs/read.md) for read-library navigation and [`docs/internal.md`](../../../docs/internal.md) for read/write internal boundaries and safety pairing.

- `discover-dependencies`, `configuration` for check command discovery.
- `doc-*`, `readme-*`, `repo-layout`, `repo-classification` for docs structure.
- `project-structure-eval` for one-pass posture scoring (kind + pillars + themes).
- `git-diff-summary` and `merge-conflicts` for git narratives and conflict handling.

These files define policies and command-shape guidance; they do not mutate git or GitHub state directly.
