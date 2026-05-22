---
name: internal-read-git-doc-graphs-legacy-graphs-pack-scaffold
description: >-
  Read-only compact legacy graph scaffold. Keep this lightweight and link to in-pack skill DAG sources.
---

# Legacy graphs scaffold (compact)

Use this only for repositories that still keep a legacy graphs page. Keep the content short and rely on in-pack skill links.

## Rules

- Keep diagrams acyclic and focused on core public skills.
- Do not copy full command lists into this file.
- Source operational behavior from `skills/**/SKILL.md`.

## Minimal graph

```mermaid
flowchart TB
  gitPull["@git-pull"] --> gitReview["@git-review"]
  gitReview --> gitPush["@git-push"]
  gitPush --> ghPr["@gh-pr"]
  ghIssues["@gh-issues"] --> issueCmds["internal-write-gh-issue-commands"]
  ghPr --> prCmds["internal-write-gh-pr-commands"]
  cursorPlan[Cursor Plan] -.-> ghIssues
```

## See also

- [`internal-read-git-doc-graphs`](../SKILL.md)
- [`internal-read-git-repo-layout`](../../repo-layout/SKILL.md)
- [`internal-read-git-workflows`](../../workflows/SKILL.md)
