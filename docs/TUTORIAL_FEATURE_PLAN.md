# Bevy 0.19 — From Gamedev Experience to Bevy: Feature & API Checklist

> A gamedev-background checklist. Each row asks: "I know I need to do X in a game engine; how do I do it in Bevy 0.19?"
> The game-specific context (KTC clone) is only the example project where the pattern is demonstrated.
> Use this when planning episodes to ensure you cover the engine patterns a migrating developer would actually look for.

---

## How to use this checklist

- **Gamedev Task / Pattern**: the thing you already know you need from any engine (camera follow, animation, save/load, etc.).
- **How to do it in Bevy**: the Bevy API or pattern that solves it.
- **Where we demonstrate it (game context)**: where the KTC clone uses it, purely for grounding.
- **0.19 specifics / gotchas**: anything new, renamed, or uncertain in Bevy 0.19.
- **Episode**: filled in later when episodes are planned.

---

## 1. Project Setup & Architecture

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Boot the engine, open a window, and run the main loop | `App` + `DefaultPlugins` + schedules (`Startup`, `Update`, `FixedUpdate`) | The entire project starts here. | Assumed prerequisite; not a teaching focus. | |
| Organize code into modules/plugins | `Plugin` trait; register plugins in `App::new().add_plugins(...)` | `WorldPlugin`, `PlayerPlugin`, `EconomyPlugin`, `CombatPlugin`, `UiPlugin`. | Keep plugins small and episode-sized. | |
| Choose fixed vs. variable timestep for gameplay | `FixedUpdate` for simulation; `Update` for rendering/input/camera | Movement in `FixedUpdate`, camera smoothing in `Update`. | | |
| Local engine docs workflow | `cargo doc`, search generated docs with `tools/search-docs.*` | Used throughout the project. | Important for any 0.19 preview build. | |

---

## 2. Testing and CI

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Write Bevy tests using App | Build an `App` in tests, add plugins/systems, call `app.update()`, assert on world state | Test monarch movement, sprite flip, and component spawning from M1. | Use `MinimalPlugins` or feature-gate rendering; avoid needing a real window. | |
| Run CI for Bevy projects (headless/minimal features) | Configure GitHub Actions / CI with a headless Bevy feature set and software rendering | Project CI runs on every push. | Disable `bevy_audio`/`bevy_render` or use `x11`/`wayland` dummy if needed. | |
| Use cargo check/clippy/test/fmt in CI | Standard toolchain checks in CI pipeline | All Rust code in the project. | Fail fast on `cargo check` before tests. | |
| Keep tests for deterministic logic | Test movement math, state transitions, resource values; avoid frame-time assertions | Player movement, coin purse logic, day/night state transitions. | Use `Time` resource manually or `FixedUpdate` in tests. | |

---

## 3. Spawning Objects (Entities / Prefabs / Scenes)

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Spawn a simple object with components | `commands.spawn((ComponentA, ComponentB, ...))` | Trees, coins, simple effects. | Assumed prerequisite. | |
| Compose a reusable scene/prefab | Bevy Scene Notation (`bsn!` macro) | Monarch + horse, campfire, buildings, coin entities. | Verify `bsn!` syntax, return type, and `spawn_scene`. | |
| Spawn a saved scene/prefab at runtime | `commands.spawn_scene(...)` / `world.spawn_scene(...)` | Spawning the monarch scene, a tree scene, a campfire scene. | Verify method signature. | |
| Create a component that auto-spawns a full scene | `SceneComponent` derive | `Wall`, `Tower`, `Campfire`, `Farm` spawn their required children/components automatically. | Verify derive macro exists. | |
| Reference entities by name inside a scene | Named entity references (`#Name`) in BSN | Referencing the horse child from the monarch, or named camera targets. | Verify `#Name` syntax and `EntityTemplate` behavior. | |
| Override fields when composing a scene | Patching / composition in BSN | Generic `wall()` scene with wood/stone/iron tier overrides. | Verify patch syntax. | |
| Load assets by path inside a scene | `Sprite { image: "player.png" }` resolves `Handle<Image>` via `FromTemplate` | Trees, coins, buildings, characters. | Verify asset-path resolution in BSN. | |

---

## 4. Hierarchies & Transforms

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Parent an object to another so it moves with it | `Children` relationship / `Children [ ... ]` in BSN | Horse is a child of monarch; UI children inside a menu root. | Verify relationship API in 0.19. | |
| Read an entity’s world-space position | `Transform` (local) vs `GlobalTransform` | Camera follow uses the player’s global position. | | |
| Build nested objects cleanly | Multi-level entity hierarchies | Monarch → Horse → Reins; Tower → ArcherSlot → Occupying Archer. | | |
| Transform propagation | Local transforms compose up the hierarchy | Any nested entity in the game. | | |

---

## 5. Camera

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Make a 2D camera follow a player | `Camera2d` + update `Transform` each frame based on a target entity | Camera follows the monarch horizontally. | | |
| Clamp the camera so it doesn’t show past world edges | Clamp camera `translation.x` to world bounds | Island left/right edges. | | |
| Smooth camera motion | Interpolate camera position toward target | Camera follow with lerp. | | |
| Control visible area / pixel scale | `OrthographicProjection` | Pixel-art feel; consistent zoom. | Verify `OrthographicProjection` field names in 0.19. | |
| Parallax background | Move background layers at different speeds | Hills, trees, sky move at different rates as the monarch rides. | | |

---

## 6. State Management

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| High-level game states (menu, playing, paused, game over) | `States` / `init_state` | `MainMenu`, `InGame`, `Paused`, `GameOver`. | | |
| Run setup/teardown when entering/exiting a state | `OnEnter` / `OnExit` schedules | Spawn/despawn menu UI, pause overlay, game world. | | |
| Auto-despawn entities tied to a state | `DespawnOnExit` component | Menu entities despawn on `OnExit(MainMenu)`. | Triggers on same-state transitions in 0.19; use `NextState::set_if_neq()`. | |
| Nested states (e.g. playing → island N) | `SubStates` | `Island1`, `Island2`, etc. under `InGame`. | Verify API exists in 0.19. | |
| Derive a state from data automatically | `ComputedStates` | Derive `DayState` / `NightState` from `DayCycle`. | Verify API exists in 0.19. | |
| Transition states safely | `NextState` + `NextState::set_if_neq()` | Start game, pause, resume, game over. | `set_if_neq()` avoids self-transition bugs. | |

---

## 7. Input Handling

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Read keyboard keys | `ButtonInput<KeyCode>` or `Input<KeyCode>` | Move monarch, drop coin, gallop, pause. | Verify whether `Input` or `ButtonInput` is the current name in 0.19. | |
| Read mouse buttons | `ButtonInput<MouseButton>` | Click menus, drop coins. | | |
| Abstract inputs into logical actions | Map raw input to `Action` enum | `MoveLeft`, `DropCoin`, `Pause`. | Gamepad can be added later. | |
| Touch input (optional) | `TouchInput` | Mobile/tablet controls. | Optional. | |

---

## 8. Timing, Timers & Delays

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Frame-rate-independent updates | `Res<Time>` + `Time.delta_seconds()` | Movement, animation, cooldowns. | | |
| Countdown timers | `Timer` + `Res<Time>` | Day/night cycle, farm income, tree regrowth. | | |
| Schedule a command to run later | `DelayedCommands` | Fire-rate cooldowns, construction stages, delayed despawns. | Verify API exists in 0.19. | |
| Pause/unpause game simulation | `Time<Virtual>` vs `Time<Real>` | Pause the world but keep UI responsive. | | |

---

## 9. Animation

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Sprite sheet animation | `Sprite.texture_atlas` (`Option<TextureAtlas>`) + `TextureAtlasLayout` asset + `Timer` | Monarch running, horse galloping, villager working, Greed walking. | In 0.19 `TextureAtlas` is a *field on `Sprite`*, not a component. `TextureAtlas { layout: Handle<TextureAtlasLayout>, index: usize }`; the layout is an asset in `Assets<TextureAtlasLayout>` built via `from_grid`. Both live in `bevy_image`. | |
| Flip a sprite to face direction | `Sprite.flip_x` | Monarch, villagers, enemies face left/right. | Verified in 0.19: `Sprite.flip_x: bool` exists. | |
| Animate child sprites independently | Child `Transform` + `Sprite` animation | Horse legs, monarch cloak. | | |
| Simple procedural animation (tweens) | Manual `Transform` / `Scale` interpolation | Coin bounce, building placement bounce, screenshake. | | |

---

## 10. Asset Loading

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Load images, fonts, audio | `AssetServer` + `Handle<T>` | Sprites, fonts, music, SFX. | | |
| Load assets with custom settings | `AssetServer::load_builder()` | Override sampler or label. | Replaces `load_with_settings_override`. | |
| Loading screen / ensure assets are ready | Asset loading state | Preload all placeholder assets before `InGame`. | Optional. | |

---

## 11. UI

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Layout UI with boxes, anchors, and flex | `Node` + `Val::Px` / `Val::Percent` + flex | HUD, menus, pause overlay. | | |
| Display text in UI | `Text` + `TextFont` | Coin counter, day counter, game-over text. | | |
| Font handling (0.19) | `TextFont`, `FontSize`, `FontSource` | HUD text, labels, buttons. | `font: handle.into()`, `font_size: FontSize::Px(24.)`. | |
| Buttons and click handling | `Button` + `Interaction` | Main menu, resume, restart, quit. | | |
| Style UI nodes (background, border) | `BackgroundColor`, `BorderColor` | HUD panels, menu panels, button highlights. | | |
| Responsive UI for different window sizes | Relative `Val` units and flex | HUD stays anchored. | | |
| Localize UI text | Asset-based string table or localization crate | M16 | Bevy 0.19 has no built-in localization; verify crate support. | |

---

## 12. Audio

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Play background music | `AudioPlayer` + `PlaybackSettings` | Day/night music tracks. | | |
| Play one-shot sound effects | Spawn audio entities on events | Coin pickup, arrow shoot, building complete. | Could use BSN observer `on(|evt: On<PlaySound>| ...)`. | |
| Loop audio and control volume | `PlaybackSettings` | Looping music, attenuated SFX. | | |
| Swap music with state changes | React to state transitions | Calm day music, tense night music. | | |

---

## 13. Event-Driven & Reactive Systems

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Send events between systems | `Event` + `EventWriter` / `EventReader` | `DropCoin`, `Damage`, `GameOver`. | | |
| React to events with self-contained handlers | `Observer` | Damage, recruitment, game over. | | |
| Attach observers directly to scenes | `on(...)` in BSN | Campfire reacts to `RecruitCoinDropped`; building reacts to `ConstructionComplete`. | Verify syntax. | |
| Run observers conditionally | `run_if(...)` on observers | Observer only runs when not paused. | Verify API exists. | |
| Decide: observer vs. event system | Self-contained reaction → observer; broad logic → system | Recruitment (observer), wave spawning (system). | | |

---

## 14. Queries, Filtering & ECS Logic

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Find entities that match criteria | `Query` with component tuples | All archers, all idle villagers, all enemies. | | |
| Exclude entities from a query | `Without`, `With` filters | Archers on walls vs. archers on the ground. | | |
| Tag entities for special handling | Marker components (`Campfire`, `BuildSite`, `Transient`) | Recruitment zones, build sites, save filtering. | | |
| Read/write resources from systems | `Res`, `ResMut` | `CoinPurse`, `DayCycle`, `WorldSeed`. | | |
| Resources are components in 0.19 | Singleton entities behind `Res`/`ResMut` | Explain how `DayCycle`/`CoinPurse` work under the hood. | Don’t derive both `Component` and `Resource` on the same type. | |
| Immutable resources (0.19) | Generic `Resource<Mutability = Mutable>` requirement | When generic code needs `ResMut`. | Verify exact API. | |

---

## 15. Save / Load & Serialization

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Make data serializable | `serde` derive (`Serialize` + `Deserialize`) | `DayCycle`, `Wall`, `Villager`, `CoinPurse`. | | |
| Register types for reflection/serialization | `AppTypeRegistry` | Save/load system knows about custom components. | | |
| Write game state to disk | `serde_json` / `ron` + file I/O | Save kingdom to a file. | | |
| Use Bevy’s built-in world serialization (0.19) | `bevy_world_serialization` | Save and load the persistent world. | Old `bevy_scene` serialization moved here; `Scene` is now `WorldAsset`. | |
| Serialize/deserialize world assets | `DynamicWorld` / `WorldAsset` | Load a saved world from disk. | Verify exact names. | |
| Filter which entities get saved | `WorldFilter` / manual component filtering | Exclude arrows, particles, enemies; keep walls, farms, villagers. | Verify `WorldFilter` API. | |
| Save on exit and load on startup | `AppExit` + `Startup` | Save when window closes; load on `Startup` if a save exists. | | |

---

## 16. Randomness & Procedural Generation

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Deterministic randomness from a seed | `rand` + `SeedableRng` (e.g. `ChaCha8Rng`) | `WorldSeed` generates the same island for the same seed. | | |
| Pick random positions/counts/directions | `rand` distributions | Tree placement, attack direction, blood-moon nights. | | |
| Regenerate the world from a seed | Re-run generation using stored seed | New islands, loading saved games. | | |

---

## 17. Lightweight 2D Physics & Collision

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Simple AABB overlap checks | Manual rectangle overlap | Coin pickup, recruitment zones, enemy attack range. | | |
| Distance/range checks | `Vec2` distance and direction | Villager job camp radius, archer range. | | |
| Ray/line-rectangle intersection | Basic line-AABB math | Arrow hitting an enemy. | | |
| Avoid pulling in a full physics engine | No `bevy_rapier2d` / `avian` | Keep the project dependency-light for tutorial clarity. | Optional callout. | |

---

## 18. Debugging & Developer Experience

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Draw debug shapes in the world | `Gizmos` | Recruitment zones, attack ranges, camera bounds. | | |
| Name entities for debugging | `Name` component | `#Player`, `#Campfire`, `#CameraRig`. | | |
| Add logging | `tracing` (`info!`, `warn!`, `error!`) | State transitions, recruitment, save events. | | |

---

## 19. Game-Specific Mechanisms (Demonstration Only, Not Teaching Focus)

These are not Bevy patterns; they are the game mechanics that happen to exercise the patterns above. They exist so the project is a complete game, not because a Unity/Godot developer needs to learn “how to gallop” in Bevy.

| Game Mechanism | Bevy Patterns It Exercises | Notes |
|----------------|----------------------------|-------|
| Monarch left/right movement | Input, `Time`, `Transform` | Trivial combination; not a teaching focus. |
| Gallop / sprint | Input, `Time`, `Transform` | Multiplies velocity; not a teaching focus. |
| Terrain spawning | `Commands`, `Transform`, `rand`, BSN | Exercises asset spawning and seeded generation. |
| Tree chopping / harvest | `Timer`, `Commands`, `Query` | Exercises timers and despawning. |
| Farm passive income | `Timer`, `Resource`, `Event` | Exercises resources and events. |
| Wall upgrade tiers | `Component`, `match`, `AssetServer` | Exercises component data and asset switching. |
| Greed move toward crown | `Query`, `Transform`, `Time` | Exercises queries and transform math. |
| Wave escalation | `Resource`, `rand`, `State` | Exercises resources and state. |
| Game over on crown loss | `Event`, `State`, `Observer` | Exercises observers and state transitions. |

---

## 20. Optional / Advanced Topics

| Gamedev Task / Pattern | How to do it in Bevy | Where We Demonstrate It (Game Context) | 0.19 Specifics / Gotchas | Episode |
|------------------------|----------------------|------------------------------------------|--------------------------|---------|
| Optimize large-scale entity queries with spatial indexing | Custom `Resource` + `Query` + `SystemSet` ordering (e.g. grid, spatial hash, or quadtree) | Optional milestone: Greed/archer targeting with a 1D x-axis grid. | Not a Bevy built-in; build it from scratch. | |

---

## Episode Planning Worksheet (to fill in later)

| Episode | Gamedev Task / Pattern | Bevy API Focus | Notes |
|---------|------------------------|----------------|-------|
| 1 | | | |
| 2 | | | |
| 3 | | | |
| 4 | | | |
| 5 | | | |
| 6 | | | |
| 7 | | | |
| 8 | | | |
| 9 | | | |
| 10 | | | |
| 11 | | | |
| 12 | | | |
| 13 | | | |
| 14 | | | |
| 15 | | | |
| 16 | | | |
| 17 | | | |
