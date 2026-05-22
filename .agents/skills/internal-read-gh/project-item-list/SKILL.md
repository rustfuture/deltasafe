---
name: internal-read-gh-project-item-list
description: >-
  Normative gh project item-list shapes, normalized item fields, v1 body-based dependency edges, Kahn-style topological
  layers with cycle and unresolved-edge reporting. Read-only. Behavioral parity with tests/check-project-order-fixtures.py.
---

# Internal: GitHub project item list + order analysis (`internal-read-gh-project-item-list`)

**Library.** **Sole** normative **`gh project item-list`** fences for this pack plus the **v1 dependency / layering contract** consumed by **`@gh-project-order-readonly`**.

**Mutations** (**`gh project item-edit`**, **`item-archive`**, **`item-delete`**, reorder APIs, …) are **out of scope** for v1—never run them from this file. Inventory-only reads and in-chat analysis only.

## Behavioral parity (automated)

Layering, cycle detection, and unresolved-edge classification **must match** the stdlib checker **[`tests/check-project-order-fixtures.py`](../../../../tests/check-project-order-fixtures.py)** (fixtures under **`tests/fixtures/project-order/`**). If prose and code diverge, **fix the SKILL or the checker** so they agree.

## Cadence (read-only)

This library is read-only and needs **no** **Proceed** gate for listing or analysis. Do **not** imply permission to run any **`internal-write-gh-project-commands`** fence from this file.

## Auth scope

`gh project` commands require token scope **`project`**. Verify with:

```bash
gh auth status
```

If needed:

```bash
gh auth refresh -s project
```

## Resolve project number (owner scope)

Use **[`internal-read-gh-project-list`](../project-list/SKILL.md)** when the user names a board by title or you need to disambiguate **`NUMBER`** + **`OWNER`**.

```bash
gh project list --owner "@me" --limit 50 --format json
```

## List items on a project (normative base)

```bash
gh project item-list <NUMBER> --owner "@me" --format json --limit 200
```

### Caller refinement (flags)

- `--owner "@me"` or `--owner ORG_LOGIN`
- `--limit N` (CLI default is low; prefer **`-L 200`** or documented cap for boards)
- `--format json` (required for machine parsing)
- `--query '…'` when supported (see `gh project item-list --help`); GHES may omit advanced query

### Compact preview (jq)

```bash
gh project item-list 1 --owner "@me" --format json -L 50 --jq '.items[] | {title: .title, type: .content.type, number: .content.number, repo: .repository}'
```

## JSON shape (normative expectations)

Hosts and CLI versions vary; treat these as **required for analysis** when present:

- Top-level **`items`** — array of project items.
- Per item (typical Projects v2 via CLI):
  - **`id`** — project item node id (stable within the project).
  - **`title`** — display title (may mirror content).
  - **`repository`** — `owner/name` string when the item is linked to a repo object.
  - **`content`** — object when the item wraps GitHub content:
    - **`type`** — e.g. `Issue`, `PullRequest`, or draft types.
    - **`number`** — issue/PR number when applicable.
    - **`body`** — issue/PR body text used for **v1 edge parsing** (may be empty).
    - **`title`** — content title when distinct from top-level title.

When **`content.body`** is missing, treat **`body` as empty** for edge parsing (no inferred edges).

## Normalization contract (in-chat)

For each item, derive:

| Field | Rule |
| --- | --- |
| **`id`** | Prefer CLI `id`; else stable synthetic key from `(repo,number,type)` for reporting. |
| **`kind`** | Lowercased mapping: issue-like → **`issue`**; PR → **`pull_request`**; draft-only → **`draft`**; unknown → **`unknown`**. |
| **`repo`** | `owner/name` from `repository` when present; else **unknown** (skip same-repo `#` edges; see unresolved). |
| **`number`** | Integer when `content.number` exists for issues/PRs; else **null**. |
| **`title`** | Prefer `content.title` then item `title`. |
| **`body`** | `content.body` or empty string. |

## v1 dependency edges (body lines only)

**Source of truth:** explicit **line prefixes** in the issue body (same-repo **`#n`** references only). Aligns with **[`internal-read-gh-issue-projects-relationships`](../issue-projects-relationships/SKILL.md)** §3.3 body fallback guidance.

### Accepted prefixes (case-insensitive)

Each line is evaluated independently; leading/trailing whitespace ignored:

1. **`Blocked by: …`**
2. **`Parent: …`**
3. **`Depends on: …`**

After the colon, collect every **`#<digits>`** token as a referenced issue number **in the same repository as the item** (v1 does not parse cross-repo URLs).

### Semantics (prerequisite graph)

- **`Blocked by: #N`** on issue **X** means **N** is a prerequisite of **X** → directed edge **`N → X`** (complete **N** before **X**).
- **`Depends on: #N`** → **`N → X`**.
- **`Parent: #N`** on child **X** → **`N → X`** (parent precedes child work in v1 ordering).

### Missing targets

If **`#N`** is parsed but **N** is not an **issue on the same project board** (normalized issue set), record **`unresolvedEdges`** with `{ from: X, to: N, reason: "target_not_on_board" }` and **do not** add that edge to the graph.

References on lines that **do not** match the three prefixes are **ignored** for edges (free text may mention `#123` without creating a dependency).

## Pull requests and drafts (v1)

- Include them in **inventory** and reporting groups.
- **Do not** auto-parse PR/draft bodies for dependency edges in v1 (unclear semantics vs issues). List under **`skippedNonIssues`** (or equivalent prose) so absence of edges is explicit.

## Algorithm (Kahn layers + cycles)

1. **Nodes** — all **issues** on the board (`kind == issue` with a `number`).
2. **Edges** — only for references that resolved to on-board issues (`N → X`).
3. **In-degree** — count incoming edges inside the node set.
4. **Layers** — repeatedly take every node with **in-degree 0**, remove them, decrement in-degrees of successors. Each removal wave is one **parallel track** (items in a layer may proceed concurrently).
5. **Order within a layer** — stable **item-list order** (the order items appeared in **`gh project item-list`** JSON) to break ties.
6. **Cycles** — if remaining nodes exist and **no** in-degree-0 node exists, **Kahn stalls**. Emit one cycle group as those remaining nodes **sorted by item-list order** (v1 treats the entire stall set as one reported cycle component for readability on small boards).

## Output schema (report to chat)

| Key | Meaning |
| --- | --- |
| **`layers`** | Ordered list of layers; each layer is issue **numbers** (or `id` + title if numbers absent). |
| **`cycles`** | List of stall components (issue numbers) when cyclic; empty when DAG resolved completely. |
| **`unresolvedEdges`** | Parsed refs that could not become edges. |
| **`skippedNonIssues`** | PRs/drafts/other items not fed into the issue DAG (v1). |

## Large boards

Prefer **`-L 200`** (or document a cap). When truncated, state **truncation** in chat and avoid implying a complete graph.

## Worked examples (mental / fixture-backed)

Fixture JSON lives under **`tests/fixtures/project-order/`**; expected outputs are asserted in **`tests/check-project-order-fixtures.py`**.

- **`no-edges.json`** — independent issues; single layer preserving list order.
- **`dag.json`** — chain `1 → 2 → 3` via `Blocked by` / `Depends on`; three singleton layers.
- **`cycle.json`** — mutual prerequisite lines; **no** complete layering; one **cycle** group `[1, 2]`.
- **`missing-ref.json`** — `Blocked by: #999` when **999** not on board → **unresolvedEdges**; remaining issues stay in one layer with stable order.

## See also

- **[`internal-read-gh-project-list`](../project-list/SKILL.md)** — `gh project list` inventory.
- **[`internal-read-gh-issue-projects-relationships`](../issue-projects-relationships/SKILL.md)** — project attach + relationship playbook.
- **[`@gh-project-order-readonly`](../../../gh/projects/order-readonly/SKILL.md)** — public read-only entry.
- **[`internal-write-gh-project-commands`](../../../write/gh/project-commands/SKILL.md)** — mutating project CLI fences (do not invoke from this read path).
