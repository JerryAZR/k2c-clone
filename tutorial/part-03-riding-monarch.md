# Part 3: The Riding Monarch — Movement and Camera Follow

> **New concepts:** `ButtonInput`, `Time<Fixed>` & `FixedUpdate`, `Single`, `Sprite.flip_x`

---

## Recap: What We Already Have

In Part 2 we gave the monarch a world to stand in: a world-pinned meadow background that fills the window vertically and tiles infinitely, plus a fixed-viewport camera. The monarch stands on the horizon and plays her idle animation, but she cannot move yet and the camera does not follow her.

---

## Goal: What We Will Build

By the end of this part:

- The monarch moves left and right with the keyboard (`A`/`D` or arrow keys).
- Holding `Shift` makes her run.
- She flips to face the direction she is moving.
- The camera follows her horizontally with a smooth lerp.

This gives us the first interactive piece of gameplay: the player can ride across the kingdom. The walk/run animation sheet swap arrives in Part 4.

---

## New Bevy APIs & Concepts

### `ButtonInput<KeyCode>`

`ButtonInput<KeyCode>` is the resource Bevy uses to expose the keyboard state. It tracks which keys are currently pressed, which were just pressed this frame, and which were just released. You read it with `Res<ButtonInput<KeyCode>>` and call `.pressed(KeyCode::...)`, `.just_pressed(...)`, or `.just_released(...)`.

> **Pitfall:** `ButtonInput` is refreshed in `PreUpdate`. Reading it in `Update` (or later) gives the current frame's state, which is what we want for held movement keys.

### `FixedUpdate` and `Time<Fixed>`

`FixedUpdate` is a schedule that runs at a fixed timestep (default 60 Hz). It is the right place for gameplay logic that must be deterministic: movement, physics, AI, etc. `Time<Fixed>` is the resource that tells you how much time elapsed during the fixed tick, so you can compute position changes like `speed * fixed_time.delta_secs()`.

> **Why `FixedUpdate`?** Both `Update` and `FixedUpdate` can use a delta time to make movement frame-rate independent. We put gameplay logic in `FixedUpdate` because it runs at a fixed, deterministic rate — important for physics, AI, replay, and networked play. For a simple 2D side-scroller, `Update` with `Time::delta_secs()` would also work, but `FixedUpdate` is the conventional home for game logic.

### `Single`

`Single<D, F>` is a system parameter that matches *exactly one* entity with the requested data and filter. It is a cleaner alternative to `Query<D, F>` followed by `.single()` or `.single_mut()`. If zero or multiple entities match, `Single` panics — which is appropriate for the player and the camera, because having none or more than one of either is a programmer error in this project.

### `Sprite.flip_x`

`Sprite` has a `flip_x` field. Setting it to `true` mirrors the sprite horizontally, which we use to make the monarch face left or right without changing the animation sheet.

---

## Walkthrough

### Designing the feature in ECS terms

What should the player see after this part?

1. **Horizontal movement** — pressing left/right changes the monarch's world position. → We need a way to capture input intent (`PlayerInput`) and a system that applies it to the `Transform`.
2. **Run modifier** — holding `Shift` moves her faster. → The same input abstraction can carry a `run` flag.
3. **Facing** — she looks left when moving left and right when moving right. → We mutate `Sprite.flip_x` based on the movement direction.
4. **Camera follow** — the view scrolls smoothly with her. → A separate `Update` system reads the player position and lerps the camera.

From this we derive a new `player/movement.rs` module and a `player/camera.rs` module.

### Why split input and movement across schedules?

We gather input in `Update` and apply movement in `FixedUpdate`. This separation is intentional:

- Input is inherently frame-rate dependent: it comes from the OS once per rendered frame.
- Movement is gameplay logic: it should be deterministic and run at a fixed rate.

There is a trade-off: Bevy's default schedule order runs `FixedUpdate` before `Update`, so the movement system reads the *previous* frame's input. That is roughly one frame of latency — acceptable for a 2D side-scroller and a good teaching example of how to bridge two schedules cleanly.

### Step 1 — The input abstraction

Create `src/player/movement.rs` and start with the logical input resource. We keep it as a resource because this is a single-player game. If we ever add local co-op, we would need a device-to-player mapping layer; per-player components are one possible architecture, but the right design depends on how the feature shapes up.

```rust
// src/player/movement.rs
use crate::player::components::Player;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInput {
    /// Horizontal movement axis: -1.0 (left) to +1.0 (right).
    pub move_axis: f32,
    /// Run modifier held.
    pub run: bool,
}
```

`move_axis` is a scalar because this side-scroller only moves horizontally. Using a resource here means we can swap the physical input device later (keyboard, gamepad, touch) without changing the movement system.

### Step 2 — Reading the keyboard in `Update`

The input system reads raw keys and writes the logical state. It runs in `Update` so it sees the current frame's `ButtonInput`.

```rust
// src/player/movement.rs
pub fn gather_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<PlayerInput>,
) {
    let mut axis: f32 = 0.0;
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        axis -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        axis += 1.0;
    }
    input.move_axis = axis;

    input.run = keyboard.pressed(KeyCode::ShiftLeft)
        || keyboard.pressed(KeyCode::ShiftRight);
}
```

For keyboard input the axis can only be `-1.0`, `0.0`, or `1.0`. If we later add analog input (gamepad stick), the same abstraction holds: the axis will be a fractional value between `-1.0` and `1.0`.

### Step 3 — Applying movement in `FixedUpdate`

The movement system reads `PlayerInput` and updates the player's `Transform` and `Sprite` facing. It runs in `FixedUpdate` for deterministic speed.

```rust
// src/player/movement.rs
const WALK_SPEED: f32 = 150.0;
const RUN_SPEED: f32 = 300.0;

pub fn apply_movement(
    input: Res<PlayerInput>,
    fixed_time: Res<Time<Fixed>>,
    mut player: Single<(&mut Transform, &mut Sprite), With<Player>>,
) {
    let (transform, sprite) = &mut *player;

    let speed = if input.run { RUN_SPEED } else { WALK_SPEED };
    let velocity = input.move_axis * speed;
    transform.translation.x += velocity * fixed_time.delta_secs();

    if input.move_axis > 0.0 {
        sprite.flip_x = false;
    } else if input.move_axis < 0.0 {
        sprite.flip_x = true;
    }
}
```

A few things to notice:

- We use `Single<...>` because there must be exactly one player. It is more concise than `Query<...>` plus `.single_mut().expect(...)`.
- We destructure `&mut *player` to get mutable references to the transform and sprite.
- We only flip when the axis is non-zero, so when the player stops she keeps facing the last direction.
- The speed is multiplied by `fixed_time.delta_secs()` so the movement is timestep-independent.

> **Run the game now.** Press `A`/`D` or arrow keys. The monarch should slide left and right across the meadow while her idle animation plays. Hold `Shift` to move faster. She should flip to face her movement direction.

### Step 4 — Camera follow

Create `src/player/camera.rs`. The camera follows the player in `Update` because camera follow is visual feedback, not simulation logic. We use an exponential lerp so it feels smooth and is frame-rate independent.

```rust
// src/player/camera.rs
use crate::player::components::Player;
use bevy::prelude::*;

const CAMERA_FOLLOW_SPEED: f32 = 6.0;

pub fn follow_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_x = player.translation.x;
    let camera = &mut *camera;

    let target = Vec3::new(player_x, camera.translation.y, camera.translation.z);
    let t = 1.0 - (-CAMERA_FOLLOW_SPEED * time.delta_secs()).exp();
    camera.translation = camera.translation.lerp(target, t);
}
```

The `Without<Player>` filter is needed because both queries access `Transform`. Even though the camera entity is not the player, Bevy's borrow checker requires an explicit guarantee that the two queries do not overlap.

We only follow the `x` axis; the `y` axis stays fixed at its initial value. This keeps the horizon locked in place and gives the side-scrolling feel.

### Step 5 — Wiring the plugin

Update `src/player/mod.rs` to register the new systems and resources:

```rust
// src/player/mod.rs
pub mod camera;
pub mod components;
pub mod movement;

// ...

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<movement::PlayerInput>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (movement::gather_input, camera::follow_camera))
            .add_systems(FixedUpdate, movement::apply_movement);
    }
}
```

The resource is initialized with `init_resource` so it exists before any system reads it. The order of systems in each schedule does not matter much here: `gather_input` produces input, `apply_movement` consumes it, and `follow_camera` observes the result.

### Simplifications

- We hardcode the keyboard keys and movement speeds as constants. This keeps the tutorial focused on the schedule bridge and input abstraction. In a larger game you would load an input map from a config file or asset so players can rebind keys.
- We only support horizontal movement. The monarch will not jump or climb until later parts, so a scalar `move_axis` is enough.
- We store `PlayerInput` as a resource. For a single-player game this is the pragmatic choice; local co-op would need a way to map devices to players, which would likely change the architecture.

---

## Summary

- We introduced a logical `PlayerInput` resource that decouples raw keyboard reading from gameplay logic.
- Input is gathered in `Update` and consumed in `FixedUpdate` so gameplay logic is deterministic and stable.
- We used `Single<...>` to cleanly access the one player entity and the one camera entity.
- We flip `Sprite.flip_x` to face the movement direction, preserving the last facing when the player stops.
- The camera follows the player with an exponential lerp, keeping the vertical position fixed.

In the next part, we'll swap the idle sheet for walk/run sheets when the monarch moves, so her feet actually animate instead of sliding.
