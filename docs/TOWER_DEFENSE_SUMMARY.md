# Tower Defense Tutorial Series Summary

> Subagent exploration of `C:\Users\Jerry\Projects\bevy\bevy-tower-defense`.
> Used as a baseline for the new *Kingdom Two Crowns* clone tutorial.

## What was covered

The existing 26-part tutorial builds a complete top-down tower defense game in **Bevy 0.18.1** + **bevy_ecs_tilemap 0.18.1**.

| Arc | Topics | Tutorial parts |
|-----|--------|----------------|
| **Foundation** | App setup, 2D camera, tilemaps, auto-tiling, refactoring | 01–05 |
| **Level data** | Loading maps and waves from TOML, multi-level architecture | 06–07, 12–13 |
| **Combat loop** | Path-following enemies, tower placement, targeting, damage, waves, win/lose | 07–11 |
| **Economy & data** | Gold resource, passive income, data-driven towers from TOML | 14–16 |
| **UI & feedback** | Tower dock, tower selection, gizmos, custom events | 17–19 |
| **Polish** | Audio, pause, plugins, input abstraction, gamepad | 20–25 |
| **Wrap-up** | Epilogue | 26 |

## Implemented game features

- **Core engine:** Bevy 0.18.1 + `bevy_ecs_tilemap` 0.18.1
- **Camera:** Single static orthographic `Camera2d`
- **Map rendering:** Grid-based tilemap with `TileStorage` and `TilemapBundle`
- **Auto-tiling:** Custom neighbor-matching rule engine (`tiling.rs`)
- **Level data:** 7 TOML levels (`assets/levels/level_01.toml` … `level_07.toml`) with paths, enemy types, and waves
- **Enemies:** Path-following sprites with health, speed, bounty, and wave scheduling
- **Towers:** Two types — instant laser tower and rocket launcher (ammo slots, splash damage)
- **Tower data:** TOML-driven tower registry (`assets/towers.toml`)
- **Placement:** Virtual cursor shared by mouse, keyboard, and gamepad; placement preview; cost validation
- **Economy:** `Gold` resource, passive income, kill bounties, HUD
- **States:** `GameState` (`LevelSelect`, `InGame`, `GameOver`) + `PauseState` (`Running`, `Paused`)
- **UI:** Level-select grid, tower dock, game-over screen, pause overlay
- **Audio:** BGM + SFX via `AudioPlayer`/`PlaybackSettings`
- **Input abstraction:** `GameAction` message enum unifying keyboard, mouse, and gamepad (using `bevy::ecs::message`)
- **Gizmos:** Cursor highlight and tower range visualization
- **Plugins:** `AudioPlugin`, `PausePlugin`, `InputPlugin`

## Notable Bevy 0.18 APIs used

- `bevy::ecs::message::{Message, MessageReader, MessageWriter}` and `App::add_message()`
- `States` / `init_state`
- `SystemSet` / `configure_sets`
- `FixedUpdate` + `Update` schedules
- `Sprite::from_atlas_image` with embedded `TextureAtlas`
- `ImageNode::from_atlas_image`
- `TextureAtlasLayout::from_grid`
- `bevy_ecs_tilemap` tilemap bundles
- `AudioPlayer` / `PlaybackSettings`
- `Gizmos` (`rect_2d`, `circle_2d`)
- `Button` + `Interaction` UI components

## Likely updates for Bevy 0.19

- The `bevy::ecs::message` API (`GameAction`, `PlaceTower`, `PlaySound`) is not in Bevy 0.19 and should be replaced with standard `Event` (`EventReader`/`EventWriter`, `add_event`, `send`) or 0.19 observers.
- `TextureAtlas` is now a separate component rather than a field inside `Sprite`/`ImageNode`; atlas spawning code needs restructuring.
- UI component names and constructors may need adjustment (e.g., `ImageNode` atlas constructors).
- `bevy_ecs_tilemap` would need a 0.19-compatible version.
- `bevy_gilrs` gamepad feature flag may need updating.

## Major gaps for a Kingdom Two Crowns clone

Kingdom Two Crowns is a 2D side-scrolling strategy/exploration game with persistent world progression. The tower defense tutorial covers many Bevy basics but leaves out the core mechanics needed for a clone:

| Needed for KTC clone | Why not covered in this tutorial |
|----------------------|----------------------------------|
| **Side-scrolling world & camera** | Tutorial is top-down, fixed camera |
| **Day/night cycle** | No time-of-day or lighting system |
| **Persistent world state / save-load** | Levels are ephemeral; no save system |
| **Procedural or large handcrafted island** | Maps are small, fixed grids |
| **Building / upgrading walls, farms, archer towers** | Only combat towers, no upgrade chains |
| **Recruiting & AI citizens/archers** | No friendly NPCs, jobs, or AI behavior |
| **Mount/riding mechanics** | No player movement or physics |
| **Enemy waves from a single direction (Greed)** | Enemies follow predefined paths, not spawned from a world edge |
| **Economy beyond gold (workers, farms, coins dropped on ground)** | Gold is just a number; no physical coins |
| **Fog of war / exploration** | Entire map is visible |
| **Sprite animation** | Static sprites only; no `TextureAtlas` animation |
| **Particle effects** | Only muzzle flashes via child sprites |
| **Physics / platforming** | No collision beyond tile checks |
| **Multi-scene persistence / meta-progression** | Level select is stateless; no unlocks |
| **Co-op multiplayer** | Single-player only |

## Suggested natural starting point for the new tutorial series

A good first part would be **"Part 1: A Side-Scrolling Kingdom"** — a minimal foundation that captures the unique feel of Kingdom Two Crowns before adding complexity.

Proposed first project milestone:

1. **Side-scrolling scene** — spawn a long 2D terrain with a parallax background and a camera that follows the player horizontally.
2. **Day/night cycle** — a simple global timer that tints the screen and toggles a "day" / "night" state.
3. **Basic monarch movement** — a player character that can walk left/right and mount/dismount a horse.
4. **First economy loop** — drop a coin on the ground, let the player pick it up, and display a coin purse HUD.

This directly addresses the biggest missing concepts (side-scrolling, day/night, economy, recruitment-adjacent interaction) while keeping the scope small enough to mirror the tutorial's incremental style. It also creates a natural bridge for readers coming from the tower defense series: they already know ECS, states, and UI, so the new series can focus on the *genre-specific* mechanics of Kingdom Two Crowns.
