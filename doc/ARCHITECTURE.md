# Architecture

How Alexandria is built: a cross-platform **desktop** application that lets you
build video collections and search them by title, artist, song, style and tags.

Alexandria is a [Tauri 2](https://tauri.app) application. A Rust core hosts the
business logic and talks to the filesystem and a full-text index; a Svelte 5
WebView renders the UI. The two communicate through Tauri's IPC bridge
(commands + events) — the "tunnel" described below.

---

## Tech stack

| Layer             | Technology                                                                                                    | Role                                                                                                                               |
| ----------------- | ------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Shell / runtime   | **Tauri 2** (`tauri` 2.9, `@tauri-apps/api` 2.10)                                                             | Bundles the native window + WebView, exposes the IPC bridge, scopes filesystem access (asset protocol).                            |
| Backend core      | **Rust** (edition 2021, MSRV 1.77)                                                                            | Domain logic, persistence, full-text search. Compiled as `app_lib` and run from `main.rs`.                                         |
| Frontend          | **Svelte 5** (runes) + **TypeScript**                                                                         | Reactive UI. Built with **Vite 8**, package manager **pnpm**.                                                                      |
| Styling           | **Tailwind CSS v4** (`@theme` tokens, utility-first)                                                          | Design system (colors, typography, radii, shadows). See the in-app **Design system** page and the `src/lib/components` primitives. |
| Search            | **Tantivy 0.22**                                                                                              | Full-text index with tokenization, lower-casing and stemming over video metadata.                                                  |
| Supporting crates | `serde`/`serde_json`, `uuid`, `chrono`, `parking_lot`, `once_cell`, `tauri-plugin-dialog`, `tauri-plugin-log` | Serialization, ids, time, synchronization, global registries, native dialogs, logging.                                             |

Frontend lives in `src/`, backend in `src-tauri/`.

---

## Layered overview

The diagram reads top-to-bottom, from the UI down to persistence. Commands flow
**down**; domain events flow **back up** to the WebView.

```
┌─────────────────────────────────────────────────────────────────────┐
│  FRONTEND — WebView (Svelte 5 + TypeScript + Tailwind v4)            │
│                                                                       │
│   Pages (pageStore)  ──  Components (primitives → molecules →         │
│                          organisms)  ──  Svelte stores               │
│                                   │                                   │
│                     Tauri adapters  (src/lib/**/ *.tauri.ts)          │
│              invoke(command, args) ▲        ▲ listen(event)           │
└──────────────────────────┼─────────────────┼────────────────────────┘
                           │   IPC "tunnel"   │     (Tauri runtime)
┌──────────────────────────▼─────────────────┼────────────────────────┐
│  BACKEND — Rust native core                 │                        │
│                                             │                        │
│  Driving adapter:  #[tauri::command] handlers (collections/          │
│      tauri_commands.rs) — scope paths (allow_path), map DTOs,        │
│      build an EventBusManager                                        │
│                     │                                                 │
│                     ▼                                                 │
│  Application services:  CollectionService · SearchService ·          │
│                         VideoFileManager                             │
│                     │                                                 │
│                     ▼                                                 │
│  Domain:  Collection · Video · Style · domain events                 │
│                     │                                                 │
│                     ▼                                                 │
│  Ports (traits):  CollectionRepository · FileManager ·               │
│                   EventBus · Clock · Indexer                         │
│                     │                                                 │
│                     ▼                                                 │
│  Driven adapters:  CollectionRepositoryFile · FileManagerForHardDrive│
│                    · TauriEventBus · SystemClock · TantivyIndexer     │
│                     │                                                 │
│                     ▼                                                 │
│  Persistence:  JSON collection files (app data dir) · Tantivy index  │
└─────────────────────────────────────────────────────────────────────┘
       ▲  domain events (EventBus.publish → emit) travel back to the WebView
```

---

## The front ↔ back tunnel (Tauri IPC)

The frontend never touches the filesystem or the domain directly. Every
interaction crosses the Tauri bridge in one of two directions.

### Request/response — `invoke`

The frontend calls a Rust **command** through `invoke` (wrapped in a
`*.tauri.ts` adapter so the rest of the UI stays Tauri-agnostic). Each command is
a `#[tauri::command]` registered in `src-tauri/src/lib.rs`
(`tauri::generate_handler![…]`).

| Command (`invoke`)         | Arguments                        | Returns           | Emits                               |
| -------------------------- | -------------------------------- | ----------------- | ----------------------------------- |
| `retrieve_videos_data`     | `paths: string[]`                | `VideoDataDTO[]`  | `video_data:retrieved` (per file)   |
| `create_collection`        | `videos: VideoToAdd[]`           | `ThumbnailItem[]` | `collection:created`, `video:added` |
| `add_videos_to_collection` | `collection_id`, `videos`        | `void`            | `video:added`, `collection:updated` |
| `update_video`             | `video: VideoCollectionToUpdate` | `void`            | —                                   |
| `get_collections`          | —                                | `Collection[]`    | —                                   |
| `search_videos`            | `query: string`                  | `void`            | `video:selected` (per hit)          |

Commands exchange **DTOs** (`tauri_dtos.rs`, `VideoToAdd`, `VideoToUpdate`,
`ThumbnailItem`) rather than leaking internal domain types where shaping is
needed; on the TypeScript side `*.tauri.ts` adapters map the JSON payloads to the
frontend `Video`/`Collection` models (`toVideo`, `toCollection`).

### Notifications — events (`EventBus` → `listen`)

Long-running or multi-step work (importing files, creating a collection,
searching) reports progress by **publishing domain events**. The backend goes
through the `EventBus` port; the `TauriEventBus` adapter emits them to the
window, and Svelte components subscribe with `listen(event, handler)`.

| Event                  | Published by                           | Consumed by (frontend)            |
| ---------------------- | -------------------------------------- | --------------------------------- |
| `video_data:retrieved` | `VideoFileManager`                     | creation / details pages          |
| `collection:created`   | `CollectionService::create_collection` | creation page                     |
| `video:added`          | `create_collection` / `add_videos`     | creation / details pages          |
| `collection:updated`   | `CollectionService::add_videos`        | details page                      |
| `video:selected`       | `SearchService::search`                | `Searchbar` → `searchResultStore` |

### Filesystem scoping

Alexandria plays local video files through Tauri's **asset protocol**. Before a
path is used, the command authorizes it for the current session via
`allow_path` (`infra/tauri/path.rs`), which calls
`asset_protocol_scope().allow_file(path)`. This keeps file access explicit and
per-path rather than granting blanket disk access.

---

## Backend architecture — hexagonal (ports & adapters)

The Rust core is organized as a **hexagon**: the domain and application services
depend only on **ports** (traits), never on concrete I/O. Adapters implement the
ports on the outside.

```
          driving side                             driven side
  (what calls the application)            (what the application calls)

  Svelte UI ──invoke──▶ tauri_commands ──▶ ┌───────────────────┐ ──▶ CollectionRepositoryFile (JSON)
                                           │   Application +    │ ──▶ FileManagerForHardDrive (fs)
                                           │   Domain (hexagon) │ ──▶ TantivyIndexer (search)
                                           │                    │ ──▶ TauriEventBus (notify UI)
                                           └───────────────────┘ ──▶ SystemClock (time)
```

### Ports and their adapters

| Port (trait)           | Defined in                   | Production adapter                                | Test double                    |
| ---------------------- | ---------------------------- | ------------------------------------------------- | ------------------------------ |
| `CollectionRepository` | `collections/collections.rs` | `CollectionRepositoryFile` (`infra/repositories`) | `CollectionRepositoryMemory`   |
| `FileManager`          | `collections/video.rs`       | `FileManagerForHardDrive` (`infra/files`)         | `FileManagerMemory` (in tests) |
| `EventBus`             | `event_bus.rs`               | `TauriEventBus` (`infra/event_bus`)               | `MemoryEventBus`               |
| `Clock`                | `clock.rs`                   | `SystemClock`                                     | `StaticClock`                  |
| `Indexer<T, U>`        | `search/search_service.rs`   | `TantivyIndexer`                                  | fake writer (`WriterCommit`)   |

- **Driving adapter** — `collections/tauri_commands.rs`: the only place that
  knows about Tauri. It validates path scope, constructs an `EventBusManager`
  around a `TauriEventBus`, delegates to a service, and maps the result to a DTO.
- **Driven adapters** — everything under `src-tauri/src/infra/`: filesystem
  repository, hard-drive file reader, Tauri event emitter, plus `SystemClock`
  and `TantivyIndexer`.

### Dependency injection / composition

Adapters are wired at the edges, not hard-coded in the domain:

- **Production wiring** happens in `lib.rs` `setup`: it resolves the app data
  directory, calls `init_prod` to register `CollectionRepositoryFile` as the live
  `CollectionRepository`, and runs pending migrations.
- **Global registries** expose the active adapters to services without threading
  them through every call: `repositories()` / `set_repositories` (`repositories.rs`)
  and `clock()` / `set_clock` (`clock.rs`), each guarded by a `RwLock`.
- **Test overrides** swap adapters per test with RAII guards —
  `with_test_repositories(...)` and `with_static_clock(...)` restore the previous
  value on `Drop`, so tests run against in-memory repositories and a frozen clock
  without touching disk or real time.

---

## Frontend architecture

`src/` is a Svelte 5 (runes) app, intentionally layered so the UI stays
decoupled from Tauri:

- **Pages** — selected by a tiny `pageStore` (`src/lib/kit/pages/pageStore.ts`)
  and rendered from a `Page → Component` map in `App.svelte`
  (`CollectionCreationPage`, `CollectionDetailsPage`, `SearchResultPage`,
  `HomePage`, `DesignSystemPage`).
- **Components** — a design-system hierarchy under `src/lib/components`:
  primitives (`Button`, `IconButton`, `Input`, `StyleTag`, `Tag`,
  `EditableField`) → molecules (`EditableChipRow`, `Thumbnail`) → organisms
  (`VideoCard`, `VideoInfoPanel`, `VideoPlayer`). The **Design system** page
  documents them live.
- **Stores** — Svelte stores hold UI state and derive selections
  (`collections.store`, `collection.store`, `searchResult.store`, …).
- **Tauri adapters** — `*.tauri.ts` modules are the _only_ frontend code that
  imports `@tauri-apps/api`. They own `invoke`/`listen` and translate payloads,
  so components depend on plain models, not on the IPC shape.
- **Styling** — Tailwind v4 utilities driven by `@theme` tokens defined in
  `public/assets/styles/theme.css` (light + dark), self-hosted fonts in
  `fonts.css`. No hand-written component CSS layer remains.

---

## Architectural principles

These are the conventions the codebase is held to; new code is expected to
follow them.

### Hexagonal architecture / ports & adapters

The domain and application services depend on **abstractions (traits)**, never
on Tauri, the filesystem, Tantivy or the system clock. Concrete I/O lives in
`infra/` adapters that are plugged in at composition time. This keeps the core
portable and independently testable, and confines framework knowledge to the
edges (`tauri_commands.rs` on the driving side, `infra/` on the driven side).

### SOLID

- **S**ingle responsibility — commands orchestrate, services hold use-case
  logic, the domain holds invariants, adapters do I/O. Each layer has one reason
  to change.
- **O**pen/closed — adding a new storage, event sink or index means adding an
  adapter that implements an existing port, not editing the domain.
- **L**iskov — every adapter (file vs memory repository, system vs static clock,
  Tauri vs memory bus) is substitutable; tests rely on this to swap real I/O for
  doubles.
- **I**nterface segregation — ports are small and focused (`Clock::now`,
  `EventBus::publish`, the three `CollectionRepository` methods) so
  implementers depend only on what they use.
- **D**ependency inversion — high-level policy depends on the port traits;
  low-level adapters depend on those same traits. Wiring is inverted into
  `lib.rs`/`init_prod` and the registries.

### Favor the emergence of interfaces / abstractions

Ports are not designed up front: they **emerge** from need. A capability starts
concrete, and when a second implementation (typically a test double) is required,
the seam is extracted into a trait — as seen with `FileManager`,
`Indexer<T, U>` (generic over the writer so a fake `WriterCommit` can stand in),
`EventBus` and `Clock`. Abstractions stay minimal and justified by a real second
caller.

### Testability

Testability is a first-class design driver, not an afterthought:

- Every port has an **in-memory / static double** (`CollectionRepositoryMemory`,
  `MemoryEventBus`, `StaticClock`, fake index writer).
- **RAII guards** (`with_test_repositories`, `with_static_clock`) install those
  doubles for the duration of a test and restore state on `Drop`, so tests are
  isolated and order-independent.
- Tests assert on **published events** (via `MemoryEventBus`) as well as state,
  pinning the observable behavior of each use case.
- The team practices **strict TDD** (Red → Green → Refactor, one behavior at a
  time) and injects **fakes, never mocks** (see the `tdd` skill and `AGENTS.md`).

---

## Persistence & migrations

- **Collections** are stored as pretty-printed **JSON files** in the OS app-data
  directory (`…/alexandria/collections/`, one `collection-<timestamp>.json` per
  collection). See the README for the per-OS path. The files are
  human-editable and portable.
- **Search index** — Tantivy builds its index from the collections on demand
  when `search_videos` runs.
- **Migrations** — `migrations.rs` runs versioned migrations at startup
  (`MigrationManager::play`, tracked by a `MigrationRepositoryFile`), e.g.
  `VideoDurationMigration` backfilling durations on existing collection files.

---

## Where to look

| Concern                           | Path                                                                                              |
| --------------------------------- | ------------------------------------------------------------------------------------------------- |
| Tauri commands (the tunnel)       | `src-tauri/src/collections/tauri_commands.rs`                                                     |
| App wiring / command registration | `src-tauri/src/lib.rs`                                                                            |
| Domain + collection service       | `src-tauri/src/collections/collections.rs`                                                        |
| Ports                             | `repositories.rs`, `event_bus.rs`, `clock.rs`, `collections/video.rs`, `search/search_service.rs` |
| Driven adapters                   | `src-tauri/src/infra/**`                                                                          |
| Full-text search                  | `src-tauri/src/search/search_service.rs`                                                          |
| Frontend pages & shell            | `src/App.svelte`, `src/lib/kit/**`                                                                |
| Design-system components          | `src/lib/components/**`                                                                           |
| Frontend ↔ Tauri adapters         | `src/lib/**/*.tauri.ts`                                                                           |
