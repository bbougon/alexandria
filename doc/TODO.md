# TODO — documentation to complete

Documentation points to write for the Alexandria project. Linked from
[CLAUDE.md](../CLAUDE.md).

- [x] **Specify the architecture** — front structure (pages via `pageStore`,
      components primitives → molecules → organisms, stores), communication with
      the Tauri backend (`invoke` / `listen`), Rust code organization
      (`CollectionService`, event bus, Tauri commands).
      → [`ARCHITECTURE.md`](ARCHITECTURE.md).
- [x] **Detail the stack** — Tauri 2, Svelte 5 (runes), Tailwind CSS v4
      (`@theme` + tokens), Vite, pnpm, Tantivy; versions and role of each piece.
      → [`ARCHITECTURE.md`](ARCHITECTURE.md#tech-stack).
- [x] **Add Tailwind to the stack** — utility-first approach, `@theme` + color /
      typography tokens, design-system page. → [`ARCHITECTURE.md`](ARCHITECTURE.md).
- [ ] **Code rules** — Svelte 5 conventions (`$state`, `$props`, `$bindable`,
      snippets), using design-system tokens instead of hard-coded colors,
      accessible components, small-component decomposition. Conventions summarized
      in [`AGENTS.md`](../AGENTS.md); a dedicated page is still to be written.
- [x] **Commit rules** — see the `git` skill: descriptive `kebab-case` branches,
      imperative messages `[CATEGORY] Verb …`, atomic commits, **no attribution
      line**, never push to `main`. → `git` skill + [`AGENTS.md`](../AGENTS.md).
- [x] **Skills to use** — `git` (conventions), `tdd` (Red → Green → Refactor),
      `run` (launch the app), `code-review` / `security-review`, `cra`.
      → [`AGENTS.md`](../AGENTS.md).
- [x] **Available commands** — `pnpm` scripts (`check`, `build:frontend`,
      `dev` / `dev:start`, `test:front`, `test:back`) and Rust commands
      (`cargo test`). → [`AGENTS.md`](../AGENTS.md) + [`README`](../README.md).
