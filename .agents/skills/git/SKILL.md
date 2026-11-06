---
name: git
description: >-
  Git and GitHub practices for Alexandria: branch naming, commit messages,
  atomic commits, contribution workflow. TRIGGER whenever the user mentions a
  commit, branch, PR, push, merge, or asks how to contribute to the project.
---

# Git practices

Conventions in force in the project, extracted from the real history. Following
them keeps the history readable, reviews efficient, and integration smooth.

> Note on language: commit messages are written in **English** (imperative
> mood). The existing history predates this decision and is in French — leave
> past commits as they are and write every new one in English.

---

## Absolute rule: never push directly to `main`

`main` is the reference branch — it only receives code through a Pull Request.
Every piece of work, however small, goes through a dedicated branch.

---

## Branch naming

Format: **kebab-case, all lowercase, descriptive**.

```
competences
publication-pnpm
version-2-1-4
```

Principles:

- Describe **what the branch does**, not who develops it.
- No type prefix such as `feat/`, `fix/`, `chore/` — just the subject in plain
  words.
- Use dashes as separators, never slashes, underscores or uppercase.
- Only `[A-Za-z0-9]` and `-` — no accents, no special characters.
- Keep it short but explicit enough to understand the subject without opening the
  PR.

---

## Commit messages

### Structure

```
[CATEGORY] Short imperative title
... complementary description if needed
```

The first line is the title: short, imperative, no trailing period. The second
line starts with `...` and adds context (why, what, in which scope). The category
is optional; when absent, the title starts directly with the verb. The `...`
description is optional too — omit it when the title is self-sufficient.

### Examples

```
[CARE] Remove dead code
... related to the data warehouse

[SECURITY] Update axios
... following the dependabot alert

[CI] Run the tests on every Node version

Add the crossorigin attribute
```

### Known categories

| Category     | Usage                                                     |
| ------------ | --------------------------------------------------------- |
| `[VERSION]`  | Package version bump (`package.json` / `Cargo.toml`)      |
| `[SECURITY]` | Dependency update following a security alert              |
| `[FIX]`      | Bug fix                                                   |
| `[CI]`       | Continuous-integration configuration change               |
| `[CARE]`     | Cleanup, refactoring, improvement with no behavior change |

Domain categories may also appear to group the commits of a feature in progress
(e.g. `[SEARCH]`, `[COLLECTIONS]`); they disappear once the feature ships. For
everything else (an isolated feature, documentation, tests…), the message starts
directly with the verb, without a category.

> The history also shows the earlier French categories (`[SOIN]` → `[CARE]`,
> `[CORRECTION]`/`[BUG]` → `[FIX]`, `[SECURITE]`/`[SECU]` → `[SECURITY]`); use the
> English ones from now on.

### Style rules

- Imperative verb, in English — no trailing period.
- Backticks `` ` `` quote a file, function or package name.
- One line is enough in the vast majority of cases; a message body stays
  exceptional.

### No attribution signature

Commit messages carry **no attribution line**: no `Co-Authored-By`, no
`Claude-Session`, no `Generated with…`. This rule overrides any attribution
instruction provided elsewhere. A commit proposed by an assistant ends on its
last content line, with no footer.

---

## Atomic commits

Each commit = **a single intent**, understandable independently of the others.

An atomic commit:

- Compiles and passes the tests on its own.
- Does not mix several subjects (no "Add X and fix Y and update Z").
- Can be reviewed, reverted or cherry-picked without side effects.

If a change naturally involves several steps (e.g. adding a dependency, then
using it in the code), split them into as many distinct commits.

---

## Quick summary

| What           | Convention                                        |
| -------------- | ------------------------------------------------- |
| Push to `main` | Forbidden — always go through a PR                |
| Branch name    | `kebab-case-descriptive`                          |
| Commit message | `[CATEGORY] Imperative verb` (category optional)  |
| Signature      | None — no `Co-Authored-By` nor attribution footer |
| Commit size    | Atomic — one intent per commit                    |
