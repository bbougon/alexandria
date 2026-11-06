---
name: tdd
description: >-
  Apply the TDD cycle (Red → Green → Refactor) to a given task, one behavior at a
  time. TRIGGER whenever the user asks to implement a feature, write tests, follow
  TDD, or mentions Red/Green/Refactor, the TDD cycle, baby steps, or "test first".
---

# Test-Driven Development

Drive the implementation from tests, one behavior at a time. The goal is not
"tests that pass" but a design that **emerges** from small, verified steps.

---

## The cycle

Repeat for every single behavior:

1. **Red** — write the smallest test that expresses the next desired behavior,
   in business terms. Run it and watch it **fail for the right reason** (the
   behavior is missing, not a typo or a compile error you didn't expect).
2. **Green** — write the **minimum** code to make that test pass. Hard-coding or
   an obvious-but-naive implementation is acceptable here; the next test will push
   it to generalize.
3. **Refactor** — with the tests green, improve the design: remove duplication,
   clarify names, extract abstractions. Change structure, never behavior. Keep
   the tests green at every edit.

Then pick the **next** behavior and start again at Red.

---

## Baby steps

- One behavior per cycle. If a test needs a lot of code to pass, the step is too
  big — split it.
- Never write production code without a failing test that requires it.
- Never add a test while another is red.
- Commit on green (atomic commits — see the `git` skill). A refactor is its own
  commit, separate from a behavior change.

---

## Tests that express intent

- **Name tests after the business behavior**, not the implementation
  (`creates_a_collection_from_the_selected_videos`, not `test_add`).
- Assert on **observable behavior**: returned values, persisted state, and
  **published events** — not on internal calls.
- Arrange / Act / Assert: keep each test readable as a small scenario.

---

## Fakes, never mocks

The project injects **fakes**, never mocks:

- Prefer an in-memory / static implementation of a port over a mocking library
  (`CollectionRepositoryMemory`, `MemoryEventBus`, `StaticClock`, a fake index
  writer).
- Install test doubles with the dedicated guards (`with_test_repositories`,
  `with_static_clock`) so each test is isolated and restores state on `Drop`.
- Let the need for a second implementation **grow a port (trait)**: don't design
  abstractions up front, extract them when a test requires a double. See
  [`doc/ARCHITECTURE.md`](../../../doc/ARCHITECTURE.md) (ports & adapters,
  testability).

---

## Running the tests

- Frontend: `pnpm test:front` (Vitest).
- Backend: `pnpm test:back` (`cargo test`).
- Everything: `pnpm test`.

Run the relevant suite on every Red and every Green — the loop is only as tight
as how quickly you see the bar go red then green.
