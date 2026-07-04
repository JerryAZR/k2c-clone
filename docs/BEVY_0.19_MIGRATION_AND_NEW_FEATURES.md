# Bevy 0.19 Migration Guide + New Features Cheat Sheet

> Local summary for the *Kingdom Two Crowns* clone tutorial. Full migration guides are shipped with the Bevy 0.19 crate source.

## Where the official migration guides live

The Bevy 0.19 crate includes draft migration guides in:

```
~/.cargo/registry/src/index.crates.io-*/bevy-0.19.0/_release-content/migration-guides/
```

The top-level process doc is `migration_guides.md`. Each breaking change has its own `.md` file. Release notes are in `_release-content/release-notes/`.

You can also read the polished version on the Bevy website once it's published.

---

## Big-ticket new features for this tutorial

### 1. Bevy Scene Notation (BSN) — `bsn!` macro

This is the headline feature of 0.19. It replaces the old bundle-heavy spawning boilerplate with a composable, Rust-like scene syntax.

- Return type: `impl Scene` for a single entity, `impl SceneList` for multiple entities.
- Spawning: `commands.spawn_scene(bsn! { ... })` / `world.spawn_scene(...)`.
- **Patching**: only specify fields you want to override; others keep defaults or earlier values.
- **Composition**: call scene functions inside other scenes.
- **Children / relationships**: `Children [ A, B ]` spawns related entities inline.
- **Named entity references**: `#Name` sets `Name("Name")` and lets other entities reference it as `EntityTemplate`.
- **Observers**: `on(|evt: On<MyEvent>| { ... })` attaches entity observers inline.
- **Asset paths**: `Sprite { image: "player.png" }` automatically resolves to a `Handle<Image>` via `FromTemplate`.
- **SceneComponent**: derive a component that automatically spawns a whole scene with it.
- **Caveat**: no built-in `.bsn` file loader yet. The code-driven workflow works today; asset-driven `.bsn` files are planned for the next release.

Minimal example:

```rust
use bevy::prelude::*;

fn level() -> impl Scene {
    bsn! {
        #Player
        Sprite { image: "player.png" }
        Children [
            Camera2d
        ]
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(level());
}
```

### 2. Resources as Components

Resources are now stored as components on singleton entities. Practically, this means:

- `#[derive(Resource)]` now also implies `Component`.
- **Don't derive both** `Component` and `Resource` on the same type. Split them if you need both.
- Resources can be queried with broad queries, but this can conflict with normal resource access. Use `Without<IsResource>` to filter resource entities if needed.
- Resources can be immutable: generic code using `ResMut<R>` may need `R: Resource<Mutability = Mutable>`.
- `World::clear_entities` now also clears resources; `World::clear_all` clears entities, resources, and non-send data.

### 3. Text system overhaul (Parley)

`bevy_text` now uses Parley instead of Cosmic Text.

- `TextFont::font` is now a `FontSource` (not `Handle<Font>`). Use `.into()` on a handle, or use `FontSource::Family`, `FontSource::Monospace`, etc.
- `TextFont::font_size` is now `FontSize::Px(24.0)` instead of a bare `f32`.
- New font properties: `weight`, `width`, `style`.
- Responsive font sizes: `FontSize::Vw`, `FontSize::Vh`, `FontSize::Rem`, etc.
- New `LetterSpacing` component.
- System font discovery requires the `bevy/system_font_discovery` feature.

### 4. Observer run conditions

Observers can now use `.run_if(...)` just like systems:

```rust
app.add_observer(
    on_damage.run_if(|paused: Res<GamePaused>| !paused.0)
);
```

### 5. Delayed commands

Schedule commands to run later without manual timers:

```rust
fn example(mut commands: Commands) {
    commands.delayed().secs(1.0).spawn(DummyComponent);
    let mut delayed = commands.delayed();
    let id = delayed.secs(0.5).spawn_empty().id();
    delayed.secs(1.5).entity(id).insert(DummyComponent);
}
```

---

## Breaking changes most likely to affect a 2D game

### Resources / ECS

| 0.18 | 0.19 |
|------|------|
| `#[derive(Component, Resource)]` on one type | Split into two types |
| `world.init_non_send_resource::<R>()` | `world.init_non_send::<R>()` |
| `world.insert_non_send_resource(value)` | `world.insert_non_send(value)` |
| `world.remove_non_send_resource::<R>()` | `world.remove_non_send::<R>()` |
| `world.non_send_resource::<R>()` | `world.non_send::<R>()` |
| `world.non_send_resource_mut::<R>()` | `world.non_send_mut::<R>()` |
| `world.get_non_send_resource::<R>()` | `world.get_non_send::<R>()` |
| `world.get_non_send_resource_mut::<R>()` | `world.get_non_send_mut::<R>()` |
| Same on `App`, `DeferredWorld`, `UnsafeWorldCell` | Same rename pattern |

### Scenes / world serialization

The old `bevy_scene` crate is now `bevy_world_serialization` because the `bevy_scene` name is used for BSN.

| 0.18 | 0.19 |
|------|------|
| `Scene` | `WorldAsset` |
| `SceneRoot` | `WorldAssetRoot` |
| `DynamicScene` | `DynamicWorld` |
| `DynamicSceneBuilder` | `DynamicWorldBuilder` |
| `DynamicSceneRoot` | `DynamicWorldRoot` |
| `SceneSpawner` | `WorldInstanceSpawner` |
| `SceneInstanceReady` | `WorldInstanceReady` |
| `ScenePlugin` | `WorldSerializationPlugin` |
| `SceneLoader` | `WorldAssetLoader` |
| `SceneFilter` | `WorldFilter` |

For GLTF scene spawning:

```rust
// 0.18
commands.spawn(SceneRoot(asset_server.load("scene.gltf#Scene0")));

// 0.19
commands.spawn(WorldAssetRoot(asset_server.load("scene.gltf#Scene0")));
```

### Asset loading

The many `AssetServer::load_*` variants are replaced by a builder:

```rust
// 0.18
asset_server.load_with_settings_override(path, settings);

// 0.19
asset_server
    .load_builder()
    .with_settings(settings)
    .override_unapproved()
    .load(path);
```

Plain `asset_server.load(path)` still works unchanged.

### Text / UI

| 0.18 | 0.19 |
|------|------|
| `TextFont { font: handle, font_size: 35., .. }` | `TextFont { font: handle.into(), font_size: FontSize::Px(35.), .. }` |
| `UiWidgetsPlugins` + `DefaultPlugins` | `DefaultPlugins` alone now includes widgets |
| `InputDispatchPlugin` + `DefaultPlugins` | `DefaultPlugins` alone now includes it |
| `experimental_bevy_ui_widgets` feature | `bevy_ui_widgets` feature |
| `CoreScrollbarThumb` | `ScrollbarThumb` |
| `CoreScrollbarDragState` | `ScrollbarDragState` |
| `CoreSliderDragState` | `SliderDragState` |

### Commands / error handling

- The `Command` trait now has an associated `type Out = ...` instead of a generic `Command<Result>`.
- `HandleError` and `CommandWithEntity` traits are folded into `Command` and `EntityCommand`.

### State

- `NextState::set()` now triggers `DespawnOnEnter` / `DespawnOnExit` even for same-state transitions.
- Use `NextState::set_if_neq()` to skip transition schedules when the target state is already current.

### Features

- `bevy_window`, `bevy_input_focus`, `custom_cursor` moved from the `default_app` feature collection to `common_api`, `ui_api`, and `default_platform` respectively.
- The `ui` feature is no longer implied by the `2d` or `3d` feature collections. If you opt out of default features, add `ui` explicitly if you need UI.

---

## Deprecations worth knowing

- `App::insert_non_send_resource` → `App::insert_non_send`
- `App::init_non_send_resource` → `App::init_non_send`
- `World::*non_send_resource*` → `World::*non_send*`
- `DeferredWorld::*non_send_resource*` → `DeferredWorld::*non_send*`
- `UnsafeWorldCell::*non_send_resource*` → `UnsafeWorldCell::*non_send*`
- `EntityCommands::remove_child*` / `remove_children` / `clear_children` → `detach_child*` / `detach_children` / `detach_all_children`
- `AssetServer::load_acquire`, `load_erased`, `load_untyped`, `load_untyped_async`, etc. → `load_builder()` chain
- `DefaultErrorHandler` → `FallbackErrorHandler` (deprecated alias still exists for one release)
- `System::type_id` → `System::system_type` (renamed to avoid shadowing `Any::type_id`)
- `WeakHandle!` macro → `uuid_handle!` macro

---

## What to cover in the tutorial?

Given the scope of a *Kingdom Two Crowns* clone, the most impactful 0.19 topics to highlight are:

1. **BSN** — replacing bundle spawning for the monarch, villagers, buildings, and UI.
2. **Resources as Components** — how `DayCycle`, `GameState`, `CoinPurse`, etc. now work under the hood.
3. **Text/Font changes** — HUD, coin counters, labels.
4. **Delayed commands** — easy scheduling for wave spawns, construction timers, etc.
5. **Observer run conditions** — nice but optional; only if we use observers for events.

Less critical for this project but good to mention:
- `AssetServer::load_builder()` if you need custom asset loading.
- Scene/world-serialization rename if you load GLTF scenes.

---

## Quick reference: searching locally

```bash
# Search all migration guides
rg -i "some_topic" ~/.cargo/registry/src/index.crates.io-*/bevy-0.19.0/_release-content/migration-guides/

# Search release notes
rg -i "some_topic" ~/.cargo/registry/src/index.crates.io-*/bevy-0.19.0/_release-content/release-notes/

# Search generated Bevy docs
./tools/search-docs.sh "SomeType"
```
