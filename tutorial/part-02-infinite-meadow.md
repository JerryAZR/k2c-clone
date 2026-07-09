# Part 2: The Infinite Meadow — World-Pinned Background

> **New concepts:** `Window`, `OrthographicProjection`, `ScalingMode`, `Projection`, `Camera2d`, `Resource`, marker components, infinite tiling

---

## Recap

In Part 1 we rendered the monarch and gave her a looping idle animation using a generic `AnimationPlugin`. The camera was a default `Camera2d` at the origin, the player stood at the origin, and the world was empty. In this part we add the meadow she rides through.

---

## Goal: What We Will Build

By the end of this part:

- The meadow background renders behind the monarch.
- The background fills the window vertically and tiles horizontally, forever.
- The monarch stands on the horizon (her feet touch the ground line).
- The camera uses a fixed world-space viewport height, so world coordinates stay stable even when the window is resized.

This gives us the world the monarch will move through in Part 3.

---

## New Bevy APIs & Concepts

### `Window` as a queryable component

`Window` is an ECS component that lives on the primary window entity. We can read it with `Query&lt;&amp;Window&gt;` to get the current window dimensions via `.width()` and `.height()`. Bevy 0.19’s `Query::single()` returns a `Result`, so we `.unwrap()` here (for a single-window game this is fine; in a multi-window game you would filter by `PrimaryWindow` or handle the error).

### `Camera2d` and its required components

`Camera2d` is a marker component that tells Bevy to render a 2D camera. It *requires* several other components, including `Camera`, `Projection`, and `Frustum`. When you spawn `Camera2d`, Bevy auto-adds those defaults. If you want a custom projection, you can provide a `Projection` component in the same spawn bundle and it overrides the default.

### `Projection` and `OrthographicProjection`

`Projection` is a component enum that wraps the camera's projection type. For 2D, we use `Projection::Orthographic(...)`. The `OrthographicProjection` inside it describes how world units map to pixels. Its `scaling_mode` field controls what happens when the window resizes:

- `ScalingMode::WindowSize` (the default): one world unit equals one pixel.
- `ScalingMode::FixedVertical { viewport_height }`: the viewport always shows the same number of world units vertically, regardless of window size. The horizontal size follows the aspect ratio.

We use `FixedVertical` so that our world coordinates (and the ground line) are stable, while the background still fills the window vertically.

### `Resource`

A `Resource` is a global singleton stored on the `World`, accessible through `Res<T>` or `ResMut<T>`. It is useful for data that many systems need but that does not belong to a specific game entity. In Bevy 0.19, the `Resource` trait requires `Component`, so resources are components under the hood; they are stored on a special resource entity (one per resource type) and accessed as world-level singletons rather than being attached to a normal game entity. We’ll use a `BackgroundStrip` resource to store the computed tile count.

### Marker components

A marker component has no data; it only tags an entity so systems can select it with `With<T>`. Our `BackgroundTile` marker is how the snap-to-grid system knows which entities are background tiles.

---

## Walkthrough

### Designing the feature in ECS terms

What should the player see after this part?

1. A meadow behind the monarch. → needs a `Sprite` with the background image.
2. The background fills the window vertically. → needs a scaled sprite and a camera projection that maps a fixed world height to the window.
3. The background repeats horizontally as far as the player can see. → needs multiple tiled sprites, repositioned as the camera moves.
4. The monarch stands on the horizon. → needs to position her at the horizon’s world y.

From this we derive a new `world/` module:

- `WorldPlugin` registers the background systems.
- `src/world/background.rs` holds constants, the `BackgroundTile` marker, the `BackgroundStrip` resource, and the two systems.
- The camera projection in `main.rs` is configured for a fixed world height.
- The player spawn uses the shared `HORIZON_Y` constant from `world`.

### Why a fixed world height?

A common beginner mistake is to scale the background sprite to the window height. That works for one window size, but it makes the horizon’s world y depend on the window size. Then the monarch would have to move up or down every time the window resizes.

Instead, we fix the world height at a design value (720 world units) and let the camera projection map that to the window. The background sprite is scaled to that same fixed world height. Now the horizon stays at one world y and the monarch stands on it permanently.

### Step 1 — Configure the camera projection

In `main.rs`, override the default projection with `ScalingMode::FixedVertical`:

```rust
// src/main.rs
use bevy::camera::ScalingMode;
use bevy::prelude::*;

mod animation;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(world::WorldPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: world::VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
```

`Projection::from(OrthographicProjection { ... })` creates the `Projection` enum variant that `Camera2d` requires. We use `..OrthographicProjection::default_2d()` so the near/far planes are set correctly for 2D.

### Step 2 — The world module and constants

Create `src/world/mod.rs` to export the plugin and the horizon constant:

```rust
// src/world/mod.rs
pub mod background;

pub use background::{HORIZON_Y, VIEWPORT_HEIGHT};

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, background::spawn_background)
            .add_systems(Update, background::update_background);
    }
}
```

And `src/world/background.rs` with the constants and shared calculations:

```rust
// src/world/background.rs
use bevy::prelude::*;

const BG_WIDTH: f32 = 2304.0;
const BG_HEIGHT: f32 = 1296.0;
const HORIZON_RATIO: f32 = 0.35; // from the bottom of the image

pub const VIEWPORT_HEIGHT: f32 = 720.0;
pub const HORIZON_Y: f32 = VIEWPORT_HEIGHT * (HORIZON_RATIO - 0.5);
```

`HORIZON_Y` is a constant because the viewport height is fixed. For `VIEWPORT_HEIGHT = 720` and `HORIZON_RATIO = 0.35`, the horizon is at `720 * (0.35 - 0.5) = -108` world units.

### Step 3 — Spawn the background tiles

We spawn enough tiles to cover the viewport plus a buffer on each side. The tile count depends on the window’s aspect ratio so ultra-wide monitors are covered automatically:

```rust
// src/world/background.rs (continued)
#[derive(Component)]
pub struct BackgroundTile;

#[derive(Resource)]
pub struct BackgroundStrip {
    pub tile_count: usize,
}

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    let window = window.single().unwrap();
    let bg_scale = VIEWPORT_HEIGHT / BG_HEIGHT;
    let tile_width = BG_WIDTH * bg_scale;

    let viewport_width = VIEWPORT_HEIGHT * (window.width() / window.height());
    let tile_count = ((viewport_width / tile_width).ceil() as usize + 2).max(3);

    commands.insert_resource(BackgroundStrip { tile_count });

    let image = asset_server.load("Summer2.png");

    for _ in 0..tile_count {
        commands.spawn((
            BackgroundTile,
            Sprite {
                image: image.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -10.0)
                .with_scale(Vec3::splat(bg_scale)),
        ));
    }
}
```

Because the replacement asset is a PNG (Bevy supports PNG out of the box), no extra Cargo features are needed.

```toml
[dependencies]
bevy = "0.19.0"
```

A few notes on the spawn code:

- The tiles are all spawned at `x = 0.0`. Their initial horizontal positions do not matter, because `update_background` runs every frame and snaps them to the correct grid slots before the first render.

- The replacement image is `Summer2.png` (2304×1296 in this example). Update `BG_WIDTH` and `BG_HEIGHT` if your replacement asset has different dimensions.
- The sprite center is at `y = 0`, so the scaled background extends from `-BG_HEIGHT * bg_scale / 2` to `+BG_HEIGHT * bg_scale / 2`, i.e. `-360` to `+360`. The horizon is at `-360 + 0.35 * 720 = -108`, which equals `HORIZON_Y` (you may need to tune `HORIZON_RATIO` if your replacement image has a different horizon).
- `z = -10` puts the background behind the player (`z = 0`). It is a large enough negative value that later entities — villagers, coins, enemies — will almost certainly sit at `z >= 0`, so the background stays behind everything by default.
- We clone the same image `Handle` for every tile; Bevy shares the underlying texture, so this is cheap.
- `tile_count` is computed from the initial window aspect ratio and stored in the `BackgroundStrip` resource so the update system can reuse it. It is not updated if the window is resized to a much wider aspect ratio later — that is a future polish item.
- The image must be **horizontally seamless** — the right edge must match the left edge — otherwise the snap-to-grid tiling will show visible seams between tiles.

### Step 4 — Snap the tiles to a grid every frame

Rather than recycling tiles when they move far off-screen, we simply compute the tile the camera is currently over and place all tiles in a contiguous block centered on it:

```rust
// src/world/background.rs (continued)
pub fn update_background(
    strip: Res<BackgroundStrip>,
    camera: Query<&Transform, (With<Camera2d>, Without<BackgroundTile>)>,
    mut tiles: Query<&mut Transform, (With<BackgroundTile>, Without<Camera2d>)>,
) {
    let camera_x = camera.single().unwrap().translation.x;
    let tile_width = BG_WIDTH * (VIEWPORT_HEIGHT / BG_HEIGHT);

    let camera_tile = (camera_x / tile_width).floor() as i32;
    let half = (strip.tile_count as i32 - 1) / 2;
    let start_tile = camera_tile - half;

    for (i, mut transform) in tiles.iter_mut().enumerate() {
        let slot = start_tile + i as i32;
        transform.translation.x = slot as f32 * tile_width;
    }
}
```

The `Without` filters are there because both queries access `Transform`. Even though the camera entity never has `BackgroundTile` and vice versa, Bevy’s borrow checker needs an explicit guarantee that the two queries never overlap. The pair `With<Camera2d>` / `Without<BackgroundTile>` on the camera query and `With<BackgroundTile>` / `Without<Camera2d>` on the tile query is what provides that guarantee.

Why this is robust: the texture is seamless, so it does not matter which physical tile occupies which slot. As the camera moves, the tiles slide into their new slots and the meadow appears infinite.

### Step 5 — Reposition the monarch to the horizon

The player’s feet should sit on `HORIZON_Y`. Since the sprite anchor is at its center, the sprite center is 64 pixels above the feet:

```rust
// src/player/mod.rs
use crate::world::HORIZON_Y;
// ...

commands.spawn((
    // ... other components ...
    Transform::from_xyz(0.0, HORIZON_Y + FRAME_SIZE as f32 / 2.0, 0.0),
));
```

Because the horizon is a constant in world space, we only need to set this at spawn. No per-frame alignment system is required.

### Step 6 — Run the checkpoint

After building, run the game. You should see:

- The meadow filling the window vertically.
- The monarch standing on the ground line (her feet at the horizon).
- The idle animation still playing.

The camera is still static, so the infinite tiling is not visibly tested yet — that happens once the camera moves in Part 3. But if you temporarily move the camera in the code, you can confirm the tiles slide along with it.

---

## Summary

- We separated the world into its own `WorldPlugin` so that background logic stays out of the player module.
- We fixed the world-space viewport height with `ScalingMode::FixedVertical` so the horizon stays at a constant world `y` and the background always fills the window.
- The tile count is computed from the initial window aspect ratio, so the background covers everything from normal monitors to ultra-wide displays.
- Instead of a complex wrap/recycle algorithm, we snap all tiles to a grid centered on the camera each frame; the seamless texture makes the jump invisible.
- The monarch is placed at `HORIZON_Y + half sprite height`, so her feet rest on the ground.

In the next part, we’ll make the monarch ride: reading input in `FixedUpdate`, switching between idle/walk/run states, and making the camera follow her with a lerp.
