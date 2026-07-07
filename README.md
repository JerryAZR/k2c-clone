# Kingdom Two Crowns Clone

A Bevy 0.19 tutorial series building a 2D side-scrolling kingdom-builder inspired by *Kingdom Two Crowns*.

## About this project

This repository contains the source code and planning documents for a tutorial series that teaches Bevy 0.19 by building a *Kingdom Two Crowns*-like game.

You play as a monarch on horseback. Ride across a generated island, drop coins to recruit wandering villagers, assign them jobs, build walls and farms, and survive nightly waves of Greed.

## Project structure

- `src/` — Rust source code.
- `assets/` — placeholder sprites, fonts, and audio used by the tutorial.
- `docs/`
  - `DEVELOPMENT_PLAN.md` — milestone-based development plan.
  - `TUTORIAL_FEATURE_PLAN.md` — Bevy feature/API checklist used when planning what to teach.
  - `TOWER_DEFENSE_SUMMARY.md` — summary of the prior Bevy 0.18 tower-defense tutorial series this series builds on.
  - `BEVY_0.19_MIGRATION_AND_NEW_FEATURES.md` — Bevy 0.19 migration notes and new-feature cheat sheet.
  - `LOCAL_BEVY_DOCS.md` — guide to generating and searching local Bevy docs.
- `tools/` — helper scripts for searching generated Bevy docs.

## Tech stack

- **Engine:** Bevy 0.19
- **Language:** Rust 1.96+
- **Edition:** 2024

## Build and run

```bash
cargo run
```

To generate local Bevy docs:

```bash
cargo doc
```

See `docs/LOCAL_BEVY_DOCS.md` for how to search them efficiently.

## Status

Planning phase. Milestone 1 (*The Moving Monarch*) is not yet implemented.
