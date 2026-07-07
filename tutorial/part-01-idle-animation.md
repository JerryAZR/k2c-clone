# Part 1: The Idle Monarch — Sprite-Sheet Animation

> **Time to read:** ~15 minutes
> **New concepts:** `AssetServer` & `Handle`, `TextureAtlasLayout` & `TextureAtlas`, `Sprite`, `Timer` & `Time`, the `Plugin` trait, `ImagePlugin`
> **Prerequisite:** A working Bevy 0.19 project that opens a window and spawns a `Camera2d` (the minimal `main.rs` we start from).

---

## Recap: What We Already Have

We have a minimal Bevy app: it opens a window, spawns a 2D camera, and runs. Nothing is drawn yet — the world is empty. In this part we put our monarch on screen and make her idle-breathe by cycling frames of a sprite sheet.

---

## Goal: What We Will Build

By the end of this part:

- The monarch (a queen, for now — no horse yet) renders on screen.
- She plays a looping **idle animation**: cycling through 5 frames of `Idle.png` so she appears to breathe.
- The frame-cycling logic lives in a **generic, reusable animation engine** — not bolted to the player — so that later NPCs, enemies, and coins can animate without us rewriting any animation code.

This gives us the first visible, moving thing in the game and sets up the code architecture every future animated entity will build on.

> The `assets/` folder also contains `Walk.png`, `Run.png`, and a background image — those are staged for later parts. For this part we only need `Idle.png`.

---

## New Bevy APIs & Concepts

### `AssetServer` and `Handle<T>`

`AssetServer` is Bevy's asset loader — a *resource* (a singleton you read with `Res<AssetServer>`) that loads files from disk asynchronously. Calling `asset_server.load("Player/Idle.png")` returns a `Handle<Image>` immediately; the actual pixels load in the background. You store the handle on a component; when the image is ready, Bevy renders it. **Pitfall:** A `Handle` is cheap to clone and shares the same underlying asset — pass clones around rather than reloading.

### `TextureAtlasLayout` (the asset) and `TextureAtlas` (the selection)

A sprite sheet is one image containing many frames. Bevy splits the concept in two:

- **`TextureAtlasLayout`** is an *asset* (stored in the `Assets<TextureAtlasLayout>` resource). It describes *how* the sheet is sliced — a list of rectangles. The helper `TextureAtlasLayout::from_grid(tile_size, columns, rows, padding, offset)` generates those rectangles for a regular grid.
- **`TextureAtlas`** is a small struct that picks *one* frame: `{ layout: Handle<TextureAtlasLayout>, index: usize }`. It is **a field on `Sprite`** (`Sprite.texture_atlas: Option<TextureAtlas>`), *not* a standalone component.

> **A common 0.19 misconception:** older tutorials call `TextureAtlas` a separate component. In 0.19 it is a field of `Sprite`; the layout is the asset. Both types live in `bevy::image`.

### `Sprite`

`Sprite` is the component that draws a 2D image. Its key fields: `image: Handle<Image>` (what to draw), `texture_atlas: Option<TextureAtlas>` (which sub-rectangle of a sheet to draw), and `flip_x: bool` / `flip_y: bool` (we'll use `flip_x` in Part 3 to face the monarch left/right).

### `Timer` and `Time`

A `Timer` counts down a duration and reports when it finishes. `TimerMode::Repeating` makes it restart automatically — perfect for animation. We tick it each frame with `timer.tick(time.delta())`, where `Time` is a resource giving us the elapsed time since the last frame (`time.delta()`). `timer.just_finished()` tells us a full period elapsed this tick.

### The `Plugin` trait

A `Plugin` is a bundle of related systems, resources, and setup registered onto the `App` in one `build` method. Organizing code into plugins (`PlayerPlugin`, `AnimationPlugin`) keeps `main.rs` tiny and makes each game *domain* self-contained. We `add_plugins(SomePlugin)` on the `App`.

### `ImagePlugin` and nearest-neighbor sampling

By default Bevy samples textures with linear filtering, which blurs pixel art. `ImagePlugin::default_nearest()` switches to nearest-neighbor sampling so our crisp 128px frames stay crisp when scaled.

> **A note on *which* animation system to use.** Bevy ships a heavier `bevy_animation` system (`AnimationPlayer` / `AnimationClip`) built around *interpolating* continuous values like transforms and colors. Sprite-sheet frames are discrete integers (frame 3 → frame 4), which don't interpolate, so Bevy's own sprite examples use the simple `Timer` + `TextureAtlas.index` pattern we use here. We'll reach for `bevy_animation` later, where it fits — for example, tinting the world during the day/night cycle.

---

## Walkthrough

### Designing the feature in ECS terms

Before writing code, think about what the player should see, then derive the data:

1. **The monarch appears** — a sprite drawn on screen. → needs a `Sprite` (image + which frame) and a `Transform` (position).
2. **She's identifiable as the player** — → a `Player` marker component.
3. **She cycles idle frames forever** — → something that tracks *when* to advance and *how many* frames exist: a `SpriteAnimation { timer, frames }` component.
4. **The animation engine is reusable** — → the system that advances frames must **not** filter on `Player`. Any entity with `SpriteAnimation` + `Sprite` should animate.

From this we get two plugins:

- **`AnimationPlugin`** (generic): owns the `SpriteAnimation` component and the `advance_animation` system. Knows nothing about the player.
- **`PlayerPlugin`** (domain-specific): spawns the monarch with the idle sheet bound, and later (Part 4) will swap sheets when her state changes.

This separation is the whole point: the *engine* is shared; the *control* is per-domain.

### Step 1 — Project structure and the animation component

We create two modules: `animation/` (the shared engine) and `player/` (the monarch). Let's start with the reusable animation component.

The component holds a repeating `Timer` (when to advance) and a `frames` count (how many frames the current sheet has, so we can wrap around with modulo):

```rust
// src/animation/components.rs
use bevy::prelude::*;
use std::time::Duration;

/// Drives sprite-sheet animation by cycling the active `TextureAtlas` frame.
#[derive(Component)]
pub struct SpriteAnimation {
    pub timer: Timer,
    pub frames: usize,
}

impl SpriteAnimation {
    /// Create a looping animation that advances one frame every `frame_duration`.
    pub fn new(frame_duration: Duration, frames: usize) -> Self {
        Self {
            timer: Timer::new(frame_duration, TimerMode::Repeating),
            frames,
        }
    }
}
```

Note `frames` is a runtime value, not a constant — that's deliberate. The same component will serve a 5-frame idle sheet, a 6-frame walk sheet, and a 6-frame run sheet; the *count* travels with the component instance.

### Step 2 — The generic animation system

Now the engine itself: a plugin that registers one system in the `Update` schedule.

```rust
// src/animation/mod.rs
pub mod components;

use bevy::prelude::*;
pub use components::SpriteAnimation;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, advance_animation);
    }
}
```

The system uses one `Query` that fetches two components from each matching entity:

- `&mut SpriteAnimation` — to tick the timer and (implicitly) read `frames`.
- `&mut Sprite` — to write the new `texture_atlas.index`.

```rust
// src/animation/mod.rs (continued)
fn advance_animation(time: Res<Time>, mut query: Query<(&mut SpriteAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());
        if !animation.timer.just_finished() {
            continue;
        }

        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue; // entity has no atlas bound — nothing to advance
        };
        if animation.frames > 0 {
            atlas.index = (atlas.index + 1) % animation.frames;
        }
    }
}
```

Notice what this query does **not** contain: there is no `With<Player>` filter. That is the design decision that makes the engine reusable. When villagers and Greed arrive in later milestones, they'll just spawn with a `SpriteAnimation` and a `Sprite`, and this exact system will animate them — no new system, no changes here.

The system runs in `Update` (not `FixedUpdate`) because animation is *visual feedback*, not simulation. We'll put gameplay logic like movement in `FixedUpdate` starting in Part 3.

### Step 3 — The player marker

The player module starts small — just a marker component that says "this entity is the monarch." It derives `Default` so we can construct it trivially when spawning:

```rust
// src/player/components.rs
use bevy::prelude::*;

/// Marker component for the player-controlled monarch.
#[derive(Component, Default)]
pub struct Player;
```

Notice we do **not** add `#[require(Transform, Visibility)]` here, even though the monarch needs both to render. Bevy's *required components* system means a component can declare others that always come with it — and `Sprite` already requires `Transform`, `Visibility`, and more. Since we always spawn the monarch with a `Sprite` (next step), those requirements are satisfied automatically. Requiring them again on `Player` would be redundant. The lesson: keep a marker component minimal — it carries identity, not rendering requirements.

### Step 4 — Spawning the monarch with the idle sheet

The `PlayerPlugin` registers a `Startup` system that builds and spawns the monarch entity. To draw a sprite-sheet frame we need three things: the image handle (loaded from disk), a layout asset (how the sheet is sliced), and a `TextureAtlas` selecting frame `0`.

`Idle.png` is 640×128, made of five 128×128 frames in a single row, with no padding. `TextureAtlasLayout::from_grid` describes exactly that:

```rust
// src/player/mod.rs
use crate::animation::SpriteAnimation;
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::prelude::*;
use std::time::Duration;

const FRAME_SIZE: u32 = 128;
const IDLE_FRAME_DURATION: Duration = Duration::from_millis(200);
const IDLE_FRAMES: usize = 5; // 640px / 128px

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
```

The spawn system requests three things from the world: `Commands` (to spawn), the `AssetServer` (to load the image), and `Assets<TextureAtlasLayout>` (to store the grid layout we create). Adding the layout to that resource returns a `Handle` we can hand to the sprite:

```rust
// src/player/mod.rs (continued)
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("Player/Idle.png");
    let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::splat(FRAME_SIZE),
        IDLE_FRAMES as u32,
        1,
        None,
        None,
    ));

    commands.spawn((
        Name::new("Player"),
        components::Player,
        SpriteAnimation::new(IDLE_FRAME_DURATION, IDLE_FRAMES),
        Sprite {
            image,
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            ..default()
        },
        Transform::default(),
    ));
}
```

The entity bundles five pieces: the `Player` marker, a `SpriteAnimation` (so the shared engine animates it), the `Sprite` (image + the starting frame `0`), a `Transform` (positioned at the origin for now), and a `Name` for debugging (it shows up in editors and logs).

> **Simplification, honestly:** we hardcode `FRAME_SIZE`, `IDLE_FRAMES`, and `IDLE_FRAME_DURATION` as constants. That keeps the spawn code readable while we focus on getting a sprite on screen. In a larger project you might describe every animation (sheet path, frame size, frame count, fps) in a data file or a `SpriteAnimation` asset so artists can tune timing without recompiling. We'll revisit this if the per-sheet constants start to repeat across idle/walk/run.

### Step 5 — Wiring it together in `main.rs`

Finally, `main.rs` declares the two modules and registers their plugins. We also configure `ImagePlugin::default_nearest()` so the pixel art stays sharp:

```rust
// src/main.rs
use bevy::prelude::*;

mod animation;
mod player;

fn main() {
    App::new()
        // Nearest-neighbor sampling keeps the pixel-art sprites crisp.
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// Spawns a static camera for now; the follow camera arrives in CP3.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

The camera is still static and centered on the origin, which is exactly where the monarch spawns — so she'll be visible. (She's drawn at the default `Transform`, centered on the origin; we'll align her to a ground line once the background exists in the next part.)

> **Run the game now.** You should see the queen standing at the center of the window, cycling through her five idle frames roughly every 200ms — a subtle breathing motion. If she looks blurry, double-check the `ImagePlugin::default_nearest()` line.

---

## Summary

- We split the code into two plugins: a **generic `AnimationPlugin`** (the shared engine) and a **`PlayerPlugin`** (the monarch domain).
- We loaded a sprite sheet with `AssetServer`, sliced it into frames with `TextureAtlasLayout::from_grid`, and selected a frame via `Sprite.texture_atlas` (a `TextureAtlas` is a *field* on `Sprite`, not a component).
- The `advance_animation` system is intentionally player-agnostic — any entity with `SpriteAnimation` + `Sprite` animates, which sets up villagers and enemies to reuse it for free.
- We enabled nearest-neighbor sampling so pixel art stays crisp.

In the next part, we'll give the monarch a world to stand in: an infinitely-repeating, world-pinned meadow background that scrolls as she moves.
