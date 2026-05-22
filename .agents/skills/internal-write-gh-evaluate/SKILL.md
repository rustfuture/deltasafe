---
name: internal-write-gh-evaluate
description: >-
  Evaluate batch: run umbrella (if any), format (prefer fix/write), lint, test, coverage per configured stacks.
  Sole definition of which verify commands run—@git-review orchestrates; this library holds the matrix.
---

# Internal: Evaluate (format / lint / test / coverage)

**Library.** **§4–§7** analogue for **`@git-review`**: run checks after **[`internal-write-gh-install-dependencies`](../install-dependencies/SKILL.md)** using the plan from **[`internal-read-git-configuration`](../../../read/git/configuration/SKILL.md)**. **Intent:** leave the workspace **working and formatted**, not only report failures.

**Write safety:** when a formatter/fixer will write files, capture a short **Goal**, use **AskQuestion** confirmation when the write set is large, and execute only after explicit **Proceed/confirm** in line with **`internal-write-plan-skill-safety`** and **`internal-write-plan-structured-qa`**.

**Consumers:** **[`@git-review`](../../../git/review/SKILL.md)** §4–7.

---

## 4. Umbrella (when defined)

If README/CI defines a **single** entry (`make check`, `npm run check`, `pnpm run validate`, `task check`, …): run **first**.

- Pass + covers analysis + tests (+ coverage if expected): may **skip** separate format/lint/test **unless** README/CI requires parity for separate steps. Still run **coverage-only** step when configuration says umbrella did not enforce it.
- **Fail:** report command and output; **do not** run **[`internal-write-gh-documentation`](../documentation/SKILL.md)** until green.

---

## 5. Format (if umbrella did not cover or did not run)

Prettier, Ruff format, Black, `cargo fmt`, `gofmt`, … from `package.json` / config.

- Prefer **fix** (write) when the project defines it (`npm run format`, `ruff format` without `--review`, `cargo fmt` without `--review`, …) so sources end **formatted**.
- If only **check** mode exists (`prettier --review`, `cargo fmt --review`, …), run it; on failure, run the matching **fix** command from README or scripts when documented, then re-review.
- **Skip** if no formatter for that stack.

---

## 6. Lint

`cargo clippy`, `ruff check`, `flake8`, `eslint`, `golangci-lint`, `go vet`, … per stack **present**.

---

## 7. Test and coverage

### 7a. Tests

`npm test`, `pnpm test`, `cargo test`, `pytest`, `go test ./...`, … per stack.

**Docs-only:** skip §7a with reason; treat evaluate as passed for documentation step if format/lint skipped or passed.

### 7b. Coverage

Only if discovery found an expectation. Run documented script/Makefile/CI-local commands; report thresholds and artifacts.

---

## Do not

- Install deps (that was Prepare).
- **[`internal-write-gh-documentation`](../documentation/SKILL.md)** — **`@git-review`** **§8a** runs it **after** green Evaluate; do **not** run it inside this skill.

## See also

- **`@git-review`** — orchestrator for **[`internal-write-gh-evaluate`](../evaluate/SKILL.md)** §4–7.
