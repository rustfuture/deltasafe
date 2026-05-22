---
name: internal-write-plan-structured-qa
description: >-
  Read-only: AskQuestion vs chat, short labels, ordering, one freeform outlet; §1e Abort/Review/Proceed for offline
  outside-repo + online writes. Pairs with internal-write-plan-skill-safety (which ops need confirm). Does not replace
  parent skill content.
---

# Internal: Structured Q&A

**Read-only policy library.** Skills that **ask**, **confirm**, or offer **finite choices** in Cursor apply **§1–§8** here (**§1** includes **§1e** triage). **Write libraries** (`skills/write/<domain>/**`): the **caller** (usually a public **`@git-*` / `@gh-*`** skill) must **merge every planned mutation** into one **summary**, then run **AskQuestion** (or equivalent) with **Proceed** before any fence in a **`write/`** library—**never** “chat yes” alone.

**Pairing:** **`internal-write-plan-skill-safety`** names **which** actions need summary → confirm; **this file** is **how** to prompt (**§3a** for those triggers; **§1e** for **Abort** / **Review** / **Proceed** on offline-outside-repo and online writes).
**Master-confirm override rule:** **`SKIP_QA_*`** and **`SKIP_QA_WRITE`** may skip repetitive low-risk prompts, but they never waive session-level master confirm for online mutations, destructive actions, or writes outside repo root.

## Do not

- Replace **`internal-write-plan-skill-safety`** wholesale.

---

## 1. Finite choices — structured UI first

- Use **AskQuestion** (or equivalent) for small known outcome sets; **gather facts** (names, dry-run) **before** options.

### 1a. Order: summary → question → options → one chat line

1. **Summary** — Goal line + tight facts (`git status --short`, `git diff --stat`, counts, top paths, commit intent—whatever parent requires). **Detail lives here**; labels stay tiny.
2. **Question** — one sentence (“Proceed with …?”).
3. **Structured options** — real finite set only; no fake **Other…** if §5 chat line covers escape (**§5**).
4. **One** trailing chat invite max (**§5**).

### 1b. Minimal binary (low-risk doc apply)

Short summary (~5 bullets) → one-line question → **No**/**Yes** (**No** first) → optional trailing line (omit when noisy; **keep** for §3a triggers).

### 1c. Plan preview (bulk / non-trivial writes)

Plan + file count + paths → “Proceed?” → **No**/**Yes** (**No** first). **Yes** = execute as summarized. **Large** commit+push: **one** combined **No**/**Yes** with **`internal-write-plan-skill-safety`**—do not stack duplicate modals for the same phase.

### 1d. Pre-write and mutation prompts (brief question, summary above)

For prompt gates that immediately decide a write path (for example, choose edit/create before drafting, or confirm a final GitHub mutation):

1. Put the **full actionable summary** in the assistant message immediately before AskQuestion (goal, target, key facts, what happens on Proceed).
2. Keep the **AskQuestion prompt text short** (one sentence; identifiers and action only). Do not paste full PR/issue bodies, long diffs, or large markdown blocks in the prompt itself.
3. Use safe-first options: **Abort** first, then one or more **Proceed** options that map to exact actions.
4. Offer exactly one refinement outlet per **§5** (either **`additionalSuggestion`** or one trailing chat line, not both).
5. If multiple **Proceed** options exist, keep labels short and specific (for example, “Proceed — edit #n”, “Proceed — create new”).

**`@gh-issues` multi-issue batch:** When several themes ship in one invocation, put a compact **table** in the pre-prompt summary (theme → **edit #n** or **create** → body file path if any). Offer **Abort** (safe-first), optional **Defer** / narrow options if the parent skill allows, then **Proceed — batch** whose summary already lists **every** `gh issue create` / `gh issue edit` in execution order. **One** Proceed confirms the **whole** batch. If the user prefers smaller steps, stop after partial execution and route remaining themes to a **follow-up `@gh-issues`** (new summary + Proceed).

This pattern complements **§3a** and is commonly used in **`@gh-pr`** intent and mutation confirms.

**`@gh-issues` pre-dedupe gate:** after each issue-draft refinement summary, use **Sharpen / Abort / Ship** (finite labels, **§5**) before inventory/dedupe; orchestration lives in **[`internal-read-gh-issue-preflight-qa`](../../read/gh/issue-preflight-qa/SKILL.md)**.

### 1e. Pre-write triage — offline outside repo + online mutations

When **`internal-write-plan-skill-safety`** applies to **writes outside the git workspace root** or **online mutations** (remote GitHub changes, **`git push`**, durable **`gh`** writes, tag push to **`origin`**, …):

1. **Summary first** — **Goal** + targets + counts per **§1a**; use **§1c** when bulk/large or many paths.
2. **AskQuestion** (safe-first order when labels are ordered): **Abort** / **Review** / **Proceed**.
   - **Abort** — stop; no mutating fence.
   - **Review** — pause to restate targets, paths, or intent in chat; adjust destination/repo/title/body **without** running **`gh`**, **`git push`**, **`git archive`**, or other mutating blocks yet. Then re-summarize and ask again.
   - **Proceed** — execute **exactly** what was summarized.

This tri-modal gate is for **writes only**. **Read-only** work (**`gh` list/view/search** when non-mutating, **`internal-read-gh-repo-stream`**, reading paths outside the workspace for **inspection**) stays **straightforward**: optional tight **§1a** preamble, then run the read **without** stacking **Abort / Review / Proceed**. Use **AskQuestion** on reads only for **finite disambiguation** (for example which issue id), not as mandatory safety triage.

**Relation to §8:** **Review** overlaps **Change plan** in failure/plan-change triage—prefer the same short labels across flows.

---

## 2. Option design

- **Very short labels** — **No**/**Yes**, **Abort**/**Proceed**, or verb+object; optional ~80-char template hint.
- **~2–12** options; larger sets → narrow first.
- **Never** paragraphs inside option text.

---

## 3. Destructive / binary flows

- Opposing actions clear; **impact before confirm** (`git status`, dry-run, counts).
- **Default-safe:** ambiguous → abort unless parent says otherwise.
- **One confirm per major risk**—do not merge unrelated risks into one **Yes**.

### 3a. Goal-first (safety triggers)

For ops flagged in **`internal-write-plan-skill-safety`** (commit, push, reset/clean, destructive installs, **`gh`** PR/issue mutations, bulk deletes, writes outside repo, …):

1. **Preamble** — **Goal** (one line) + compact facts (**§1a**).
2. **Question** — one short sentence (e.g. “Proceed with commit and push?”).
3. **AskQuestion** — **No**/**Yes** or **Abort**/**Proceed**; safe option first when order matters.
4. **One** chat reminder after buttons (**§5**); skip if redundant with **`additionalSuggestion`**.

The modal prompt may repeat only minimal identifiers (branch/tag/target), while the detailed summary remains in the message above.

Remote irreversible (**push**, durable **`gh`** writes) and local destructive (**reset/clean**, wipes) share this pattern.

---

## 4. Open-ended

Chat prompt; one question + inline examples; re-ask with constraint on validation failure (e.g. branch name).

---

## 5. Freeform — no duplicate outlets

- **Safest option first** when UI order matters.
- **Escape** = real negative options (**No**/**Abort**)—not **Other…** **and** “or type in chat” **and** **`additionalSuggestion`** same turn.
- **One** freeform outlet: either **one** post-button chat line **or** JSON **`additionalSuggestion`**—not both as duplicate “type something else” paths.
- **`allow_multiple: true`** only when truly needed (rare).

---

## 6. Tool naming

Describe capability (“structured multiple-choice”) vs hard-coding a single IDE control name.

---

## 7. Consumers (index)

**Any** skill with **AskQuestion** / destructive **Proceed**: **`@git-push`**, **`@git-main`**, **`@git-reset`**, **`@git-start`**, **`@git-review`** (install prepare), **`@git-docs`**, **`@git-zip`**, **`@gh-pr`**, **`@gh-pr-close`**, **`@gh-issues`**, **`@gh-issue-close`**, **`@gh-issue-delete-closed`**, **`@gh-project-delete-closed`**, **`@gh-issue-pick`**, **`@gh-issue-review`** (scope only), **`internal-read-gh-pr-description`** (multi-template / material reshape), **large** doc edits (**`internal-write-plan-skill-safety`** **confirm-first**).

**Internal:** **`internal-write-plan-skill-safety`** — trigger list vs **§3a** / **§1e** here.

---

## 8. Failure / plan-change triage

Use this pattern when a mutating step fails (for example **`git push`**, **`gh`** mutation), or when a new fact changes the plan (for example wrong branch, missing remote, protected/default-branch restrictions).

1. Put full failure context in the assistant message above the prompt. Keep AskQuestion prompt brief and action-oriented (for example, “Push failed on branch `foo`; choose next step.”).
2. Keep options to about **3** when possible, **safe-first**:
   - **Abort** — stop without further writes.
   - **Change plan** — switch approach (for example create/switch branch, choose a different target, or refine in chat per **§5**).
   - **Proceed / Retry** — retry same path; if risky (for example force-like behavior), name the risk in the label/prompt.
3. Do not hide risky actions behind generic “Proceed.” If an option can rewrite shared history or bypass protection, call that out explicitly and rely on **`internal-write-plan-skill-safety`** triggers.
4. Keep one refinement outlet only (either **`additionalSuggestion`** or one trailing chat line), consistent with **§5**.
