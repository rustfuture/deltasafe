---
name: internal-write-plan-skill-safety
description: >-
  Read-only: which operations need summary → confirm → act (push, commit, deletes, reset/clean, bulk/structural
  creates, writes outside repo root, system-wide toolchain changes when the agent would perform them); §1e triage for
  writes vs lightweight reads. Does NOT replace parent skills—layers on structured-qa for prompt UX.
---

# Internal: Skill safety (irreversible and high-impact actions)

**Read-only policy.** Skills that perform **irreversible** or **high-impact** work must **summarize impact**, then **obtain explicit user confirmation**, then **act**. Confirmation UX follows **[`internal-write-plan-structured-qa`](../structured-qa/SKILL.md)** **§1a**, **§1b**, **§1c**, **§1e**, **§2**, **§3–§3a**, **§5**, **§8** (compact summary / **plan preview** before Q&A, **Cursor** **AskQuestion** for finite choices, **one** freeform reminder when needed—**no** redundant **Other** + duplicate chat invite; default **abort** on ambiguity; failure triage uses safe-first ~3-option choices).

**Does not** replace parent skill steps (e.g. **`@git-reset`** **Proceed** before **`git reset --hard`**)—this library defines **what** to confirm and **how** to frame it.

**Global gate** — **Any online mutation** (anything that **changes** remote GitHub state: **`gh issue`/`gh pr`/`gh api`** writes, **`git push`**, and equivalent) and **any write to paths outside the current git workspace root** (or outside an explicit user allowlist) require **Goal + structured confirm** (**AskQuestion** or equivalent per **`internal-write-plan-structured-qa`**) **before** the agent runs them. **Read-only** inventory (**`internal-read-gh-issue-list`**, **`internal-read-gh-pr-list`**, **`internal-read-gh-repo-stream`**, **`internal-write-gh-issue-commands`** **View** section when used for single-issue read-only, …) does **not** replace that gate for follow-on writes—the **mutation skill** owns **Proceed**.
**Master-confirm override rule** — **`SKIP_QA_*`** and **`SKIP_QA_WRITE`** flags can suppress low-risk prompt repetition, but they **never** bypass the session-level master confirm for online mutations, destructive local operations, or writes outside repo root.

- **Write triage shape** — **Online writes** and **writes outside the repo root** use **[`internal-write-plan-structured-qa`](../structured-qa/SKILL.md)** **§1e** (**Abort** / **Review** / **Proceed**) before the first mutating command when the user may need to refine targets without a binary **No**/**Yes** only.
- **Reads stay lightweight** — **Read-only** online and outside-repo passes do **not** use that stacked triage; see **§1e** read carve-out.

For reusable prompt shape and labels, use [`internal-write-plan-structured-qa`](../structured-qa/SKILL.md) §1a, §1c, and §3a.

## Internal libraries: read vs write cadence

- **Read-only internals** — Libraries whose **normative fences** are **only** non-mutating toward GitHub and **non-destructive** toward disk by default (**`internal-read-gh-issue-list`**, **`internal-read-gh-pr-list`**, **`internal-read-gh-repo-stream`**, **`internal-read-git-merge-conflicts`** playbook, **`internal-read-gh-issue-dedupe`** checklist logic, **`internal-read-git-discover-dependencies`** when the parent forbids install/evaluate, most **`internal-read-git-doc-*`**, **`internal-read-git-git-diff-summary`** as read-only diff input, …). **Parents** that are **read-only** (e.g. **`@gh-issue-list`**, **`@gh-pr-list`**, **`@gh-issue-review`** inventory) may use them **without** an extra confirmation **unless** the parent skill itself needs **AskQuestion** (ambiguous pick). **Never** treat a read-only pass as permission to **skip** **Proceed** on a later **`gh`** write, **`git push`**, **commit**, **reset/clean**, **outside-repo write**, or **destructive install**—those steps keep their **Triggers** rows below.
- **Write / mutation internals** — Libraries that own **`gh`** **mutations**, **`gh api`** writes, **`git`** history or index **mutations** via invoked commands, or **destructive** installs (**`internal-write-gh-issue-commands`**, **`internal-write-gh-pr-commands`**, **`internal-write-git-tag`** (**`git tag`** / **`git push`** tag), **`internal-write-gh-install-dependencies`** when cleans apply, …). **Always** run behind a **public** skill that performs **Goal + structured confirm** before the first mutating fence. **Extra scrutiny** when the effect is **online** or **outside this repository folder**.

## `internal/gh` naming vs `git` work

- All pack internals live under **`skills/read/gh/`** with path-faithful `internal-read-gh-*` / `internal-write-gh-*` names. **Split by job**, not by the word “git” in a name:
  - **GitHub / network** — **`gh`** and GitHub **HTTP/API** shapes for issues/PRs/templates/stream (**`internal-read-gh-issue-list`**, **`internal-read-gh-pr-list`**, **`internal-write-gh-issue-commands`**, **`internal-write-gh-pr-commands`**, **`internal-read-gh-repo-forms-json`**, **`internal-read-gh-repo-stream`**, **issue/PR description** libs, …).
  - **Local `git` / workspace shell** — **`internal-write-git-zip`**, **`internal-write-git-tag`**, **`internal-read-git-git-diff-summary`**, **`internal-read-git-merge-conflicts`**, **`internal-write-git-working-tree-align`**: **local terminal** playbooks and **`git`** usage **without** substituting **`@gh-*`**. **Public** **`@git-*`** skills under **`skills/git/`** own user intent for push/pull/reset/zip/tag; they consume these locals plus **discover / install / configuration / evaluate** for **toolchain** work on disk.

---

## Pattern

1. **Summarize** — In chat: counts, branch/remotes, `git status --short`, `git diff --stat` (or staged stat), representative paths (cap long lists with “+N more”), dry-run output when available—combined with a **Goal** line into one **brief but detailed** preamble per **[`internal-write-plan-structured-qa`](../structured-qa/SKILL.md)** **§1a**. When **§1c** applies (large change set, outside-repo writes, many new doc paths), the preamble must include an explicit **plan** (what will run, file count, top paths).
2. **Goal + confirm** — Then **AskQuestion** (or equivalent) with **very short** labels (**No**/**Yes** or **Abort**/**Proceed**, etc.) per **§2**; **at most one** brief chat reminder after the buttons per **§5** when the flow is not using the **§1b** “omit reminder” carve-out. Do **not** treat free-form chat as the **only** confirmation when the choice set is finite. One confirm may cover **commit + push** when both happen in the same flow; **merge** **§1c** with **§3a** into **one** round when both apply.
3. **Act** — Only after explicit **Yes** / **Proceed** (or equivalent) from that structured step, or after the user’s **alternative** instruction in **chat** (re-summarize if scope changed).

---

## Structured confirm (authoring)

**Structured confirm** — Shorthand for child **`SKILL.md`** files: meet the relevant **Triggers** row (what the summary must include; confirm **Yes** where required) **and** apply **[`internal-write-plan-structured-qa`](../structured-qa/SKILL.md)** **§1c** (when bulk/large) + **§3a** / **§1e** (offline-outside-repo + online writes) for **Goal** + **AskQuestion** shape—**merge** into **one** round when both apply. Link **this file** at least once per skill or per major section; in the same section you may then write **structured confirm** instead of repeating both library names.

---

## Triggers (minimum)

| Trigger | Summary must include | Confirm |
| --- | --- | --- |
| **`git push`** / first upstream publish / **`git push origin <tag>`** | Branch or **tag** name, **`origin`** URL when pushing tags, remote intent, stat for what will be committed if the tree is not clean | Yes, before **commit** (if any) and **push** |
| **`git push --force-with-lease`** (or equivalent force-like push) | Branch, remote, why non-force push failed, explicit risk to remote history/teammates, and safer alternatives considered | Yes — separate explicit confirm from normal push confirm; default **Abort** on ambiguity |
| **`gh issue delete`** (bulk closed cleanup) | Count + preview table of ids/titles; irreversible on GitHub | Yes, before **each** delete wave (**`@gh-issue-delete-closed`**) |
| **`gh project delete`** (bulk closed cleanup) | Owner scope + preview table of numbers/titles; irreversible on GitHub | Yes, before **each** delete wave (**`@gh-project-delete-closed`**) |
| **`git commit`** (when the skill runs it) | Same stat / one-line message intent | Yes (may share one confirm with push) |
| **`gh pr create`** | Target repo, **base** / **head**, one-line intent (“open a new PR with this title/body”) | Yes, before **`gh pr create`** |
| **`gh pr edit`** | PR number, summary of title/body replacement (“replace remote PR description”) | Yes, before **`gh pr edit`** |
| **`gh pr close`** | PR number, repo (`--repo` when applicable), one-line intent (“close PR #*N*”) | Yes, before **`gh pr close`** |
| **`gh label create`** | Label name(s), color, short description intent | Yes, before **`gh label create`** (may be **separate** from **`gh issue create`** confirm when risks would blur) |
| **Delete** paths (tracked or untracked) | Count + representative paths | Yes |
| **`git reset --hard`**, **`git clean`**, destructive install cleans | Dry-run or impact list where applicable | Yes |
| **Bulk / structural file creation** (many new paths from a plan) | Count + top paths / move map | Yes |
| **Many new `README.md` / doc hub pages in one change set** | Count + categories + top paths | Yes — **§1c** plan preview + **No**/**Yes**; fold `.gitkeep` into the same summary (no separate modal) |
| **Writes outside git workspace root** (or outside an explicit user allowlist) | Paths named; **Goal** (“touch paths outside repo”) | Yes — **§1c** + **§1e** before any write (**Abort** / **Review** / **Proceed**) |
| **Large change set** (see below) | **§1c** plan embedded in preamble: file count + `git diff --stat` excerpt + top paths | Yes — use **§1c**; **merge** with **commit/push** confirm into **one** **No**/**Yes** when both apply |

**Online mutation rows** — Rows whose triggers change GitHub or **`origin`** (**`git push`**, **`gh pr create`/`edit`/`close`**, **`gh issue`** mutations, **`gh project delete`**, **`gh label create`**, …) use the same **§1e** triage shape when a **Review** beat is needed—not limited to binary **No**/**Yes** when the user must refine targets or copy before mutating.

**Large change set (defaults)** — Treat as **large** when **either**: (a) **≥10 files** changed (staged + unstaged intended for the operation), or (b) **≥500 lines** net change from `git diff --stat`. Parent skills may tighten or relax with user consent.

### Exception: **`@git-pull`** merge commits

Merge commits from **`@git-pull`**—including **`git merge`** when it completes without conflict and **`git commit`** after conflict resolution—**do not** need a separate **AskQuestion** before each commit. The user already invoked **`@git-pull`** to bring **`@{u}`** and **`ROOT_BRANCH`** into the current branch; that intent covers finishing the merge.

If **`git status`** shows **unexpected** state before merging (e.g. unrelated uncommitted changes the user did not imply), **stop** and clarify in **chat** per **[`internal-write-plan-structured-qa`](../structured-qa/SKILL.md)** **§4**; default **abort** on ambiguity. Publishing remains **`@git-push`** (with its own confirm).

### Exception: ephemeral scratch paths in **`@git-push`**

Removing **only** directories or files the agent **created in the same `@git-push` run** as throwaway output (e.g. a random temp path never staged) so they are **not** committed **does not** require a separate **AskQuestion**—outside the **Delete** row’s normal confirm. **Do not** use this for tracked paths, user-owned dirs, or anything beyond that narrow case.

---

## Scope for new files (“create”)

- **No** extra confirm for routine **single-file** edits the user clearly asked for.
- **Yes** confirm for **bulk** or **structural** adds—e.g. **many** new **`docs/`** or mirror **`README.md`** paths in one change set, **`@git-docs`** when touching **many** **existing** doc paths in one run (**§1c**), plan-driven edits that add **many** new paths or reshape trees.

---

## Cross-skill Q&A (quick map)

| Situation | Skill / library |
| --- | --- |
| User-driven **many** new doc / mirror paths | **Large create** — **AskQuestion** + **No**/**Yes** before bulk writes (**`internal-write-plan-skill-safety`**) |
| User-driven doc sync (many **existing** files) | **`@git-docs`** — **AskQuestion** for mode; **No**/**Yes** before bulk in-place writes (**§1c**) |
| Verify + **§8a** doc polish | **`@git-review`** — runs **`internal-write-gh-documentation`** after green evaluate |
| Commit + push | **`@git-push`** — structured confirm for large change sets (**§3a**) |
| `gh pr create` / `gh pr edit` | **`@gh-pr`** §9 — confirm before mutation; **`internal-read-gh-pr-description`** for **multiple** PR templates **or** **material template reshape** (keep / blend / pack) — **separate** from that **§9** PR mutation confirm when risks would blur |
| `gh issue create` / `gh issue edit` / `gh issue close` / `gh issue delete` | **`@gh-issues`**, **`@gh-issue-close`**, **`@gh-issue-delete-closed`** — confirm before mutation; **`internal-read-gh-issue-description`** for **multiple** issue templates **or** **material reshape** — **separate** from final **`gh`** confirm when needed |
| `gh project delete` | **`@gh-project-delete-closed`** — confirm before mutation; preview owner scope + closed set before each delete wave |
| **`gh label create`** (issue label decoration) | **`@gh-issues`** after **`internal-read-gh-issue-labels`** — confirm before **`gh label create`**; then include label flags in the chosen issue mutation path |
| Delivery from issue anchor (`#n` / URL) with branch + PR delegation | **`@gh-issue-view`** then **`@git-start`** — read full issue extract via **`internal-write-gh-issue-commands`** view when needed, present TODO plan + checkpoints, then **Goal + AskQuestion + Proceed** before mutating downstream flows (**`@git-start`**, implementation writes, **`@gh-pr`**) |
| **`gh pr close`** | **`@gh-pr-close`** — **Goal** + **AskQuestion** + **Proceed** before **`gh pr close`** |

---

## Do

- Apply this policy whenever a trigger row applies; link **`internal-write-plan-structured-qa`** for how to ask.
- Keep confirms **one phase per major risk**—do not merge unrelated risks into a single “yes” (see structured-qa §3).

## Do not

- Skip summary or confirmation for rows in the table above.
- Treat ambiguous replies as consent—default **abort** unless the parent skill states otherwise.

---

## See also

- **[`internal-write-plan-structured-qa`](../structured-qa/SKILL.md)** — Prompt UX (AskQuestion, ordering, escape hatches).

### Optional cleanup for ephemeral `.cursor/gh` staging

When a run created disposable staging paths under `.cursor/gh/issues/` or `.cursor/gh/pr/`, offer a final **AskQuestion** to keep or delete those paths. Default to **keep** on ambiguity.
