---
name: internal-read-git-doc-graphs
description: >-
  Read-only: when to add mermaid, color/layout conventions; diagram scaffolds in ./diagrams/*/SKILL.md.
  Optional legacy-graphs-pack-scaffold (legacy GRAPHS.md). Callers: @git-docs. Does not write files.
---

# Internal: Documentation graphs (`internal-read-git-doc-graphs`)

**Read-only.** **When to diagram** (kind choice), **Mermaid constraints**, **color/layout narrative**, and **diagram scaffolds** all live **here**—**one** library to open for figures. **Hex tables**, **named `classDef`s**, and **copy-paste blocks** live only in **[`diagrams/palette/SKILL.md`](./diagrams/palette/SKILL.md)**, **[`diagrams/examples/SKILL.md`](./diagrams/examples/SKILL.md)**, **`diagrams/scaffold-*/SKILL.md`**. **`[legacy-graphs-pack-scaffold/SKILL.md](./legacy-graphs-pack-scaffold/SKILL.md)`** — optional legacy **`docs/GRAPHS.md`** only.

**Consumers:** scaffold passes, **`@git-docs`**.

## When to add a diagram

- **Flows** (pipelines, hand-offs, branches) → **flowchart** or **sequence** template.
- **Parallel tracks / subgraphs** → **DAG** template.
- **States / modes** → **state** template.
- **User journeys, friction, recovery** → **`diagrams/scaffold-user-journey/SKILL.md`**.
- **Which option / when to use** → **`diagrams/scaffold-decision/SKILL.md`**.
- **Actors or lanes** (user, product, support) → **`diagrams/scaffold-stakeholder-flow/SKILL.md`**.
- **Static-only** folders may use a **small hierarchy** figure or skip if prose suffices.

## Do (diagrams)

- Pick **one** kind template, copy into a **single** fenced **`mermaid`** block with **`classDef`** / **`class`** inlined (**colored**—no monochrome-only narrative diagrams).

## Published shape

- **Where:** fenced **`mermaid`** inside **`docs/**`**, minimal root **`README.md`**, or legacy hubs.
- **Scaffolds:** one block per **`diagrams/scaffold-*/SKILL.md`** child skill.
- **Rules:** constraints below + **palette** + **examples** (do **not** duplicate long ramps or sample figures in this file).

## Do not

- Run shell, **`git`**, or **`gh`** from this skill.
- Duplicate long **palette** / **examples** bodies in this file—link **`diagrams/palette`** / **`diagrams/examples`** child skills instead.

## Mermaid constraints

- **Node IDs:** `camelCase` / `PascalCase` / `snake_case` — **no spaces** in IDs.
- **Labels** with spaces/special chars: **double-quoted** node text, e.g. `node["/gh-start"]`.
- **GitHub renderer safety:** quote any label containing `@` (for example `node["@gh-pr"]`) or other special punctuation to avoid parse failures like `LINK_ID` in README/PR/issue rendering. Reference: [GitHub Mermaid docs](https://docs.github.com/get-started/writing-on-github/working-with-advanced-formatting/creating-diagrams).
- **Edge labels** with parens/commas: **quotes**, e.g. `A -->|"O(n)"| B`.
- **Reserved words:** avoid bare `end`, `subgraph`, `graph`, `flowchart` as IDs; use `endNode`, `subgraph id [Label]`.
- **Subgraphs:** explicit ids: `subgraph id [Human label]`.

## Color — spine first

**Unique to this file:** narrative for **A→…→B** emphasis (entry-strong vs leaf-strong). **Implementation:** always **`classDef`** + **`class`** at block end; contrast per **`diagrams/palette/SKILL.md`**.

1. **Spine** — pick the main directed path; **A** = entry / trigger, **B** = outcome (or **leaf-strong**: strongest on terminal outcome—**one story per figure**).
2. **Ramp** — 2–5 `classDef` steps along the spine (mix toward white / lower saturation between anchors); **side branches** muted (`pubM`, `intM`, `neuM`, legacy `hueGhM`, `uxOff`, … per **`diagrams/palette/SKILL.md`**).
3. **Do not** — rainbow unrelated nodes; identical strong fills everywhere; low-contrast labels.

For skill-pack or agent-architecture figures, prefer domain ramps from **`diagrams/palette/SKILL.md`**: **`pub*`** (public skills), **`hum*`** (human instructions), **`int*`** (internal narrowed skills), with **`neu*`** only for non-skill context.

**Worked colored DAGs** — copy from **`diagrams/examples/SKILL.md`** only.

## Layout

- **TB** — portrait-friendly single column; **LR** — wide pipelines.
- **Avoid** dense N×M grids; split into two figures or use **`sequenceDiagram`** when time-order dominates vertical space.

## File layout

| File | Role |
| --- | --- |
| **`SKILL.md`** (this) | Constraints + layout narrative |
| **`legacy-graphs-pack-scaffold/SKILL.md`** | Legacy **`docs/GRAPHS.md`** only |
| [`diagrams/palette/SKILL.md`](./diagrams/palette/SKILL.md) | Named ramps |
| [`diagrams/scaffold-*/SKILL.md`](./diagrams/) | Per-kind scaffolds |
| [`diagrams/examples/SKILL.md`](./diagrams/examples/SKILL.md) | Sample figures |

## See also

- [`internal-read-git-doc-exemplars`](../doc-exemplars/SKILL.md) — markdown rubric
- [`internal-read-git-repo-layout`](../repo-layout/SKILL.md) — hub contract
- [`@git-docs`](../../../git/docs/SKILL.md)
