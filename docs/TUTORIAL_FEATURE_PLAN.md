# Bevy 0.19 — Kingdom Two Crowns Clone: Tutorial Feature Plan

> Continuation of the `bevy-tower-defense` series. Assumes viewers know Bevy ECS basics, states, UI, audio, and input handling. Focuses on genre-specific mechanics and 0.19-specific APIs.

## Tutorial philosophy

Each episode answers two questions:

1. **How do I achieve X in the game?** (design/feature goal)
2. **How do I use this Bevy API?** (concrete implementation)

Episodes are short, incremental, and end with a runnable build. Code stays clean and tutorial-friendly, even if it sacrifices some performance polish.

---

## Part 1: Foundation & World Feel

### Episode 1: A Side-Scrolling Monarch
- **Game goal:** App setup, a camera that follows the player, and basic side-scrolling movement. The monarch is always on a horse (cosmetic child entity).
- **Bevy APIs:** `DefaultPlugins`, `Camera2d`, `OrthographicProjection`, `Window`, `Startup`, `Update`, `Time`, `Transform`, `Input<KeyCode>`, `ButtonInput`.
- **How do I achieve X?**
  - Camera follow: update the `Camera2d` transform based on player position, clamped to world bounds.
  - Player movement: modify `Transform.translation.x` with velocity; gallop with a held key.
  - Fixed mount as hierarchy: the horse is a child of the monarch so it moves automatically with the parent.
- **0.19 hook:** Briefly show the traditional `commands.spawn((Sprite, Transform, ...))` first, then introduce `bsn!` for the monarch + horse scene to show why BSN is useful for hierarchies.

### Episode 2: Seeded 1D World Generation
- **Game goal:** Generate a simple side-scrolling island from a seed so the player can walk around and inspect it.
- **Bevy APIs:** `Resource` (seed), `rand`, `Commands`, `Query`, `Transform`, `Sprite`, `Handle<Image>`, `AssetServer`.
- **How do I achieve X?**
  - 1D world generation: use a seeded RNG to produce a sequence of terrain segments along the x-axis (forest, plain, camp slot, etc.). No 2D noise needed.
  - Place trees, the campfire, and the initial castle based on the seed.
  - Walk around to visually verify the generated world.
- **0.19 hook:** Use `bsn!` to spawn terrain chunks and trees with asset paths like `Tree { sprite: "tree.png" }`.

### Episode 3: Dropping and Picking Up Coins
- **Game goal:** The monarch can drop a coin on the ground; dropped coins can be picked back up.
- **Bevy APIs:** `Event`, `EventReader`, `EventWriter`, `SpatialBundle`, `Collider` (custom AABB), `Overlap` query, `despawn`.
- **How do I achieve X?**
  - Physical coin: spawn a small sprite entity with position and a `Coin` component.
  - Pickup: use a broad query with `Aabb` overlap detection against the player.
  - Coin purse: `CoinPurse` resource; UI text updates from it.
- **0.19 hook:** Use `bsn!` to define the `Coin` entity, and show `commands.spawn_scene(coin(x))` with a parameter.

### Episode 4: Time of Day and the Day/Night State Machine
- **Game goal:** A clear day/night cycle drives spawn behavior and lighting tint.
- **Bevy APIs:** `Resource`, `Timer`, `Time`, `ResMut`, `State` / `ComputedStates`, `NextState`, `Schedule`, `Color` interpolation.
- **How do I achieve X?**
  - Global clock: `DayCycle` resource with `day`, `time_of_day`, `is_day`.
  - State transitions: `DayState` / `NightState` via `bevy_state`.
  - Visual feedback: lerp `ClearColor` and a screen overlay color based on time.
- **0.19 hook:** Use `ComputedStates` to derive `DayState` from `DayCycle` automatically.

---

## Part 2: Economy, Recruitment, and Jobs

### Episode 5: Villagers and the Campfire
- **Game goal:** A campfire recruits unemployed villagers when the monarch drops coins near it.
- **Bevy APIs:** `Component`, `Bundle`, `Query`, `Res`, `Commands`, `Event`, `Observer` (new in 0.19), `on()` in BSN.
- **How do I achieve X?**
  - Recruiting: detect coins dropped within a radius of the campfire; consume coin and spawn a `Villager`.
  - Villager AI state: `JobSeeking`.
  - Campfire scene: `bsn!` with `Campfire` component + `RecruitmentZone`.
- **0.19 hook:** Use `on(|evt: On<RecruitCoinDropped>| ...)` observer inside the campfire BSN.

### Episode 6: Jobs — Builder, Archer, Farmer
- **Game goal:** Villagers can be assigned to jobs by purchasing tools at camps.
- **Bevy APIs:** `Enum` as component, `State` pattern on entities, `#[derive(Component)]` enum, `match` in systems, `SceneComponent`.
- **How do I achieve X?**
  - Job camps: builder camp, archer camp, farm camp spawn points.
  - Job assignment: when a coin is dropped near a job camp, nearest idle villager changes `Job`.
  - Job-specific behavior: builders seek construction sites, archers seek walls/towers, farmers seek farms.
- **0.19 hook:** Define each job camp as a `SceneComponent` that spawns its own zone + sprite.

### Episode 7: Trees, Forests, and Harvesting
- **Game goal:** Builders chop trees for coins; forests regenerate over time.
- **Bevy APIs:** `Timer`, `Commands`, `Query`, `Resource`, `AssetServer`, `Handle<Image>`, `Sprite`.
- **How do I achieve X?**
  - Resource nodes: `Tree` component with health / coin reward.
  - Builder AI: move toward nearest tree, reduce tree health, spawn coin when destroyed.
  - Regeneration: after a delay, respawn the tree at the same location.
- **0.19 hook:** BSN asset path: `Tree { sprite: "tree.png" }` resolves the handle automatically.

### Episode 8: Building Walls, Farms, and Towers
- **Game goal:** Builders construct defensive walls and upgrade them; farmers work farms that generate passive income.
- **Bevy APIs:** `BuildSite` component, `ConstructionProgress` timer, `Health`, `UpgradeTier` enum.
- **How do I achieve X?**
  - Construction sites: empty markers that builders walk to and fill in over time.
  - Upgrade tiers: wood → stone → iron wall with different health sprites.
  - Farms: produce a coin every X seconds during the day.
  - Towers: archers automatically occupy them at night.
- **0.19 hook:** Use `SceneComponent` for `Wall`, `Farm`, `Tower` so each building always spawns its required components and child entities.

---

## Part 3: Enemies and Combat

### Episode 9: The Greed — Enemy AI and Escalation
- **Game goal:** Night spawns waves of Greed from one side of the world; they move toward the crown. Waves escalate over days and include occasional blood-moon nights.
- **Bevy APIs:** `State`, `NightState`, `Timer`, `Event`, `Query`, `Transform`, `Velocity`, `Resource`.
- **How do I achieve X?**
  - Wave schedule: define waves per night in a `NightWaves` resource.
  - Spawn at world edge: spawn enemies just off-screen at the attack direction.
  - Basic AI: move toward the monarch / nearest wall / coin purse carrier.
  - Escalation: increase enemy count and speed based on `DayCycle.day`.
- **0.19 hook:** Spawn waves using `commands.spawn_scene_list(wave(day))` returning multiple enemies.

### Episode 10: Archers and Combat
- **Game goal:** Archers on walls/towers shoot arrows; knights (if added) fight in melee.
- **Bevy APIs:** `Projectile` component, `Timer` for fire rate, `Ray` or AABB collision, `despawn_recursive`.
- **How do I achieve X?**
  - Target acquisition: query for nearest enemy in range.
  - Projectile flight: spawn an arrow sprite, move it each frame, detect collision with enemy.
  - Damage: reduce enemy health, spawn hit effect, drop coin bounty on death.
- **0.19 hook:** Use `DelayedCommands` for fire-rate cooldowns instead of manual `Timer` resources.

### Episode 11: Walls, Breaches, and the Crown
- **Game goal:** Enemies attack walls; if a wall falls, they continue toward the crown. If the crown is taken, game over.
- **Bevy APIs:** `Health`, `Attack` component, `CollidingEntities` (custom), `GameOver` state.
- **How do I achieve X?**
  - Wall destruction: enemies within range reduce wall health; wall sprite changes when damaged.
  - Crown steal: if a Greed reaches the monarch, transition to `GameOver`.
  - Respawn: continue from last known camp / save point.
- **0.19 hook:** Observer on `GameOver` event that pauses the world and shows UI.

---

## Part 4: World Persistence and Progression

### Episode 12: Saving and Loading the Kingdom
- **Game goal:** The world state persists between play sessions (day, coins, built walls, recruited villagers).
- **Bevy APIs:** `serde`, `bevy_world_serialization` (old `bevy_scene`), `DynamicWorld`, `AppTypeRegistry`, `AssetServer`.
- **How do I achieve X?**
  - Serialize entities with `Serialize`/`Deserialize` components.
  - Save to disk on exit and load on startup.
  - Filter out transient entities (projectiles, particles) from save.
- **0.19 hook:** Explain the rename: `bevy_scene` is now BSN; old scene serialization is `bevy_world_serialization` / `DynamicWorld`.

### Episode 13: Boat Travel and New Islands
- **Game goal:** The monarch can build a boat and travel to a new island, carrying some coins and upgrades. The new island is generated from a fresh seed.
- **Bevy APIs:** `State`, `Scene` transitions, `SubStates`, `Resource` carry-over, `OnEnter` / `OnExit` systems, seeded `WorldGen`.
- **How do I achieve X?**
  - Boat construction site: builders work on it over multiple days.
  - Travel event: save persistent data, unload current world, generate a new 1D island from a new seed.
  - Meta-progression: keep crown upgrades, ship upgrades, and coin purse across islands.
- **0.19 hook:** Use `SubStates` to model `Island1` / `Island2` / `Island5` under `InGame`.

---

## Part 5: UI, Polish, and Audio

### Episode 14: HUD — Day, Coins, and Health
- **Game goal:** Persistent UI showing day/night timer, coin count, and crown/health status.
- **Bevy APIs:** `Node`, `Text`, `TextFont`, `FontSize`, `FontSource`, `BackgroundColor`, `BorderColor`, `Val`.
- **How do I achieve X?**
  - Layout with `Node` and `Val::Px`/`Val::Percent`.
  - Update text from resources each frame.
- **0.19 hook:** Cover `TextFont` changes: `font_size: FontSize::Px(24.)`, `font: handle.into()`.

### Episode 15: Menus, Pause, and Game Over
- **Game goal:** Main menu, pause overlay, and game-over screen with restart.
- **Bevy APIs:** `State`, `OnEnter`, `OnExit`, `DespawnOnExit`, `Button`, `Interaction`, `Pointer` events.
- **How do I achieve X?**
  - Menu state machine: `MainMenu`, `InGame`, `Paused`, `GameOver`.
  - UI buttons: start game, resume, restart, quit.
- **0.19 hook:** `DespawnOnExit` now triggers on same-state transitions; use `NextState::set_if_neq()` to avoid it.

### Episode 16: Sound, Music, and Particle Sparkles
- **Game goal:** Day/night music, coin pickup sounds, building completion, explosion particles.
- **Bevy APIs:** `AudioPlayer`, `PlaybackSettings`, `bevy_audio`, `Sprite` particles, `Timer` for particle lifetime.
- **How do I achieve X?**
  - Spawn audio entities on events.
  - Simple CPU particles: spawn short-lived sprites with fade-out.
- **0.19 hook:** Use BSN `on(|evt: On<PlaySound>| ...)` observers for self-contained audio triggers.

---

## Part 6: Advanced / Optional Episodes

### Episode 17: Local/Online Co-op Multiplayer
- **Game goal:** Two monarchs on the same screen, shared camera and resources.
- **Bevy APIs:** Player-specific input mapping, shared camera bounds, resource sharing between players, network abstractions if online.
- **Why teach it:** Local multiplayer shows how to handle multiple input sources and shared camera design; online adds networking concerns.
- **Note:** This is likely a multi-episode mini-series on its own, not a single episode.

---

## Cross-cutting 0.19 themes to weave in

| Theme | Where to introduce | Why it matters |
|-------|---------------------|----------------|
| `bsn!` macro | Episode 1 | Replaces verbose bundle spawning for scenes/hierarchies. |
| `SceneComponent` | Episode 5 | Guarantees that buildings/NPCs always spawn with their full scene. |
| `Resources as Components` | Episode 4 | Explain `DayCycle`/`CoinPurse` internals and immutable-resource gotchas. |
| `Observer` + `on()` | Episode 5 | Self-contained event reactions in BSN. |
| `DelayedCommands` | Episode 10 | Clean fire-rate and construction timers. |
| `FontSize` / `FontSource` | Episode 14 | New text API. |
| `bevy_world_serialization` | Episode 12 | Old scene serialization got renamed. |
| `ComputedStates` | Episode 4 | Derive `DayState` from `DayCycle` automatically. |

---

## Suggested recording order

1. Episode 1 → Episode 2 → Episode 3 → Episode 4 (foundation: camera/player, seeded 1D world, coins, day/night)
2. Episode 5 → Episode 6 → Episode 7 → Episode 8 (economy & jobs)
3. Episode 9 → Episode 10 → Episode 11 (combat, including escalating waves and blood-moon logic)
4. Episode 12 → Episode 13 (persistence and new seeded islands)
5. Episode 14 → Episode 15 → Episode 16 (UI/polish)
6. Optional: Episode 17 (local/online co-op, likely expanded into its own mini-series).

Each episode should end with a `cargo run` and a short recap of the API-to-feature mapping.
