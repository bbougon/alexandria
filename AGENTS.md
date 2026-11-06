# Agent instructions — Alexandria

This file is the **source of truth** for instructions aimed at development
assistants (Claude Code, Codex, Junie…). `CLAUDE.md` is a symbolic link to this
file — every change is made here, in `AGENTS.md`.

## Documentation

- [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) — detailed architecture: stack,
  layers, the front ↔ back tunnel, hexagonal (ports & adapters) design and
  principles.
- [`doc/ROADMAP.md`](doc/ROADMAP.md) — postponed work and planned evolutions.
- [`doc/TODO.md`](doc/TODO.md) — documentation still to complete.

## GOLDEN RULES

- **NEVER** access or attempt to access `.env` and `.envrc` files at the project
  root.
- Never attempt to install any library, package or executable without explicit
  approval.
- For every library, package or executable to install:
  - provide a link to the official documentation.
  - explain the reasons for the installation.
  - explain the benefits of the installation.
  - list any known or potential security risks.
  - check that the version used is the latest stable one.
  - do not propose a version younger than 7 days.
- **NEVER** commit under the name of Claude or Codex. Commit under the user
  configured in git, with **no attribution line** (see the `git` skill).

Note: the last two points are partly tooled already — `pnpm-workspace.yaml` sets
`minimumReleaseAge: 10080` (7 days, in minutes) and Renovate
(`.github/renovate.json`) drives version bumps.

## Project context

Alexandria is a cross-platform **desktop** application (Tauri 2) to build video
collections and search them by title, artist, song, style and tags. It was built
to find videos recorded by a guitar teacher. A Rust core holds the domain logic,
persistence and full-text search; a Svelte 5 WebView renders the UI; the two talk
over Tauri's IPC bridge (`invoke` + events).

## Tech stack

- **Tauri 2** — native shell + IPC bridge; hosts the Rust core and the WebView.
- **Rust** (edition 2021) — backend: domain, persistence (JSON files), full-text
  search (**Tantivy**).
- **Svelte 5 / TypeScript** — frontend, built with **Vite** and managed with
  **pnpm**.
- **Tailwind CSS v4** — styling and design-system tokens (`@theme`).

See [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md#tech-stack) for versions and the
role of each piece.

## Commands

- `pnpm dev` — run the Vite dev server (frontend only, in a browser).
- `pnpm dev:start` — run the full Tauri app (`tauri dev`).
- `pnpm build` — build the Tauri app; `pnpm build:frontend` builds the frontend.
- `pnpm check` — `svelte-check` + `tsc` type checks.
- `pnpm test` — run all tests; `pnpm test:front` (Vitest), `pnpm test:back`
  (`cargo test`).

## Code organization

Ports / adapters (hexagonal) architecture — see
[`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md):

- `src` — frontend sources (Svelte).
- `src-tauri` — backend sources (Rust).

## Conventions

- **Language**: English everywhere — code, identifiers, documentation and commit
  messages (see the `git` skill).
- **Dates**: ISO format.
- **Method**: strict TDD, one behavior at a time → `tdd` skill.
- **Tests**: injected fakes (never mocks), test names expressing the business
  intent.
- **Git**: never push directly to `main` (always a PR); `kebab-case` descriptive
  branches; `[CATEGORY] Imperative verb` messages; atomic commits; no attribution
  line → `git` skill.

## Available skills

Versioned in `.agents/skills/` (`.claude/skills` is a symbolic link to it).
**The repository's skills prevail**: when a skill of the same name exists both in
this repository and in the environment (user-level or built-in), the version from
`.agents/skills/` takes precedence.
Triggered according to their description:

- `tdd` — Red → Green → Refactor cycle, strict baby steps.
- `git` — the team's Git / GitHub conventions (branches, commit messages, atomic
  commits, contribution workflow).

Other skills provided by the environment (`run`, `code-review`,
`security-review`, `cra`…) are used as their descriptions indicate.
