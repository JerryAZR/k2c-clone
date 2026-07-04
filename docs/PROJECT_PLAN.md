# Kingdom Two Crowns Clone — Project Plan

> Work-in-progress design doc for a Bevy 0.19 tutorial series.

## Core Concept

A side-scrolling 2D kingdom-builder inspired by *Kingdom Two Crowns*:
- The Monarch (player) rides left/right on a horse.
- Builders, archers, farmers, and knights are recruited with coins.
- Days are safe; nights bring waves from one or both sides.
- The goal is to build, expand, and survive.

## Tech Stack

- **Engine:** Bevy 0.19.0
- **Language:** Rust 1.96+
- **Edition:** 2024
- **Rendering:** Bevy 2D sprites (`bevy_sprite` / `bevy_sprite_render`)
- **UI:** Bevy UI (`bevy_ui`, `bevy_ui_widgets`) for HUD/menus
- **Input:** keyboard/mouse via `bevy_input`
- **Audio:** `bevy_audio` for music and SFX
- **State:** `bevy_state` for game states (Menu, Playing, GameOver, etc.)

## Project Phases (tutorial-friendly)

### Phase 0: Setup & Tooling
- [x] Local Bevy 0.19.0 docs generated (`cargo doc`)
- [ ] Minimal window + clear color
- [ ] Asset loading strategy (placeholder sprites vs. real art)
- [ ] Game state scaffold (`Menu`, `InGame`, `Paused`, `GameOver`)

### Phase 1: The World
- [ ] Camera follow / side-scrolling
- [ ] Ground / terrain generation
- [ ] Basic day/night cycle (time of day, schedule)
- [ ] Persistent kingdom bounds (left/right camps, walls, towers)

### Phase 2: The Monarch
- [ ] Player entity with velocity/sprite
- [ ] Horse movement (run left/right, gallop)
- [ ] Coin purse component + UI
- [ ] Drop coins / pick up coins

### Phase 3: Villagers & Jobs
- [ ] Villager base component + job state machine
- [ ] Recruitment at campfire / castle
- [ ] Builders: build/repair walls
- [ ] Archers: stand on walls, shoot at night
- [ ] Farmers: farm during day, return at night
- [ ] Knights: lead squires, attack greed

### Phase 4: Economy & Buildings
- [ ] Trees / forests to harvest
- [ ] Camps (builder, archer, farmer)
- [ ] Walls, towers, farms, castles
- [ ] Upgrade tiers (wood → stone → iron)

### Phase 5: Enemies & Combat
- [ ] Greed basic AI (move toward crown, attack walls)
- [ ] Day/night spawn waves
- [ ] Blood moon / wave escalation
- [ ] Combat resolution (archers, knights, monarch)

### Phase 6: Polish
- [ ] Sound effects and music
- [ ] Particle effects (coin sparkle, blood, fire)
- [ ] Screenshake
- [ ] Save/load run data
- [ ] Menu flow and credits

## Key Bevy 0.19 APIs to research

- `App`, `Plugin`, `Update`, `Startup`, `FixedUpdate` schedules
- `Commands`, `Query`, `Res`, `ResMut`, `EventReader`, `EventWriter`
- `Component`, `Bundle`, `Resource`
- `States` / `SubStates` / `ComputedStates`
- `Transform`, `Sprite`, `Camera2d`, `OrthographicProjection`
- `Input<KeyCode>`, `ButtonInput<MouseButton>`, `TouchInput`
- `AssetServer`, `Handle<Image>`, `Handle<AudioSource>`
- `Text` / `Text2d` / `Node` UI
- `Time` / `Timer` / `Virtual` / `Real`

## Notes

- Keep systems small and composable so each tutorial episode can focus on one concept.
- Use `bevy_state` for high-level flow rather than manual flags.
- Consider a pixel-art aesthetic; render at low resolution and upscale.
- Avoid `bevy_pbr` unless we add 3D elements later.
