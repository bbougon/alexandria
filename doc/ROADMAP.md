# Roadmap

Postponed work and planned evolutions. Linked from [CLAUDE.md](../CLAUDE.md).

## Editable player info panel (postponed on 2026-11-04)

Today the player's info panel (`VideoInfoPanel`, shown by `VideoPlayer`) is
**rich but read-only**: title, artist, song, styles, tags, duration, size and
location. Editing a video is only possible from its card (`VideoCard`), through
hover-to-edit fields and per-row edit icons.

**Why it is postponed.** The player is driven **imperatively**: the `play(video)`
method captures a snapshot of the video when it opens. Making the panel
two-way editable from that model would be fragile (state synchronization between
the card and the player) and would introduce new behavior, whereas the current
visual rework is deliberately limited to presentation.

**What it will take to enable it.**

- Move `VideoPlayer` from **imperative** control (`play()`) to **prop-driven**
  control (video + update callbacks provided by the parent).
- Reuse `EditableField` (title / artist / song, hover-to-edit) and
  `EditableChipRow` + `MusicStyleEdit` / `TagsEdit` (styles / tags) inside
  `VideoInfoPanel`, enabled only when update callbacks are provided (player
  opened from a card = editable; import preview or search result = read-only).
- Wire those callbacks to the same persistence mechanism as the card
  (`selectedCollection.updateVideo` / `removeStyle` / `removeTag` +
  `invoke('update_video', …)`) — no new backend code required.

## Design-system rework — done

The "design system" branches A–E (presentation only) have been **ported to
Tailwind utilities and merged into `main`**; `components.css` was removed (the
Tailwind migration is complete). The `ThemeToggle` sits in the sidebar footer.

The **`DesignSystem` page** has shipped (PR `design-system-page`): reachable from
the sidebar next to the theme toggle, it documents the tokens (palette,
typography, radii, shadows) and the components rendered live (`Button`,
`IconButton`, `StyleTag`, `Tag`, `Input`, `EditableField`, `EditableChipRow`,
`Thumbnail`); the domain-bound components (`VideoCard`, `VideoInfoPanel`,
`VideoPlayer`), coupled to the store and Tauri, are documented without being
rendered in isolation.
