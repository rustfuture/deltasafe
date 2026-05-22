---
name: internal-write-gh-install-dependencies
description: >-
  Prepare batch: install deps and build per README/CI for stacks present. Used by @git-review §3.
  Destructive cleans require user confirm per internal-write-plan-structured-qa.
---

# Internal: Install dependencies (prepare)

**Library.** Single **Prepare** batch for **`@git-review`**: install/build so format, lint, and tests can run.
**Safety gate:** When the agent executes network installs or destructive cleans, require **Goal + structured confirm** via **[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** and **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** before execution.
**Master-confirm rule:** **`SKIP_QA_*`** and **`SKIP_QA_WRITE`** do not bypass session-level confirm for destructive cleans or agent-run install/write actions.

**Consumers:** **[`@git-review`](../../../git/review/SKILL.md)** §3, after **[`internal-read-git-discover-dependencies`](../../read/git/discover-dependencies/SKILL.md)** + gap map.

**Default posture:** **Prompt the user** with **copy-paste Terminal command batches** and a **one-line reason**; the user runs installs **by hand**. The agent **may** run installs only when the user has **explicitly** asked the agent to execute prepare in this session—otherwise treat **network GET / package installs** as **user-run**. **Destructive** wipes (`rm -rf node_modules`, full venv delete) still require **structured confirm** regardless.

---

## Commands (by stack — minimal defaults)

When the user will run commands themselves, print the exact block(s). When executing as agent, use **in README order** when documented; else:

- **Node:** Lockfile-first (`npm ci`, `yarn install --frozen-lockfile`, `pnpm install --frozen-lockfile`) else `npm install` / `yarn` / `pnpm`; respect workspaces.
- **Python:** `pip install -e .`, `uv sync`, `poetry install`, `pip install -r requirements.txt`, … per `pyproject.toml` / README.
- **Rust:** `cargo fetch` / `cargo build` as needed for Clippy/tests.
- **Go:** `go mod download`; `go build ./...` if required.
- **Ruby / JVM / .NET:** follow README.

**Before** `rm -rf node_modules`, full venv wipe, or similar: **structured confirm** (**[`internal-write-plan-skill-safety`](../../../write/plan/skill-safety/SKILL.md)** destructive-install row + **[`internal-write-plan-structured-qa`](../../../write/plan/structured-qa/SKILL.md)** **§3a**)—not chat-only yes/no.

---

## Do not

- Run format, lint, or test **as verification**—that is **[`internal-write-gh-evaluate`](../evaluate/SKILL.md)**.
- Commit or push.

## See also

- **[`internal-read-git-configuration`](../../read/git/configuration/SKILL.md)** — next step: resolve evaluate commands.
