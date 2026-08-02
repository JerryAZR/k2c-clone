# Part 4: The Animated Monarch — Walk and Run States

> **New concepts:** `Changed<T>`, `.chain()`, the animation state machine pattern

---

## Recap: What We Already Have

In Part 3 we made the monarch controllable: she moves left and right in `FixedUpdate`, runs while `Shift` is held, flips to face her direction, and the camera follows smoothly. But her feet slide — she plays the idle animation no matter how fast she moves.

---

## Goal: What We Will Build

By the end of this part:

- The monarch plays the **walk** animation while moving, the **run** animation while `Shift` is held, and **idle** when stopped.
- The sheet swap is instant on every transition, with no stale frames.
- The state machine is architected so future one-shot animations (hurt, coin toss) extend it without a rewrite.

This completes Milestone 1: a fully animated, controllable monarch striding across the meadow.

---

## New Bevy APIs & Concepts

### `Changed<T>`

`Changed<T>` is a query filter that matches only entities whose component `T` was mutated since the system last ran. Bevy tracks a "change tick" on every component; the filter compares it against the system's last-run tick.

It exists so you don't have to store `previous_state` fields and compare by hand. Our sheet-swap system uses it to run only on transitions instead of re-applying the same animation every frame. (It also fires once when a component is first inserted, so our apply system runs harmlessly at spawn and re-applies the idle sheet.)

> **Pitfall:** `Changed` fires on any mutable access, even if you write back the same value. Upstream systems should only mutate when the value actually differs — otherwise the filter fires every frame and the optimization is lost.

### `.chain()`

`.chain()` is an ordering modifier for a tuple of systems: `(a, b, c).chain()` guarantees they run in that order within the schedule. Without explicit constraints, Bevy is free to run systems in parallel in an arbitrary order — great for throughput, but wrong when one system's output is another system's input.

### The animation state machine pattern

Bevy has no built-in 2D sprite state machine — its official 2D examples only cycle a single sheet, and `bevy_animation` targets *interpolatable* fields (transforms, colors) rather than discrete sprite indices. The community-standard pattern for 2D splits the work into two systems:

1. **Derive** — one system decides *which state we should be in* and writes it to a component.
2. **Apply** — a second system, gated by `Changed`, applies the state to the sprite.

The state component is the single source of truth. Any future system (damage, interactions) can write the same component, and the apply logic never needs to change.

---

## Walkthrough

### Designing the feature in ECS terms

What should the player see after this part?

1. **Walk sheet while moving** → we already know "moving" from `PlayerInput.move_axis`.
2. **Run sheet while Shift is held** → `PlayerInput.run` distinguishes walk from run.
3. **Instant swap on every transition** → swap the image *and* atlas layout, then restart the frame cycle from frame 0.
4. **No work while the state is stable** → apply changes only when the state actually changed.

From this we derive the data:

- An `AnimationState` component (`Idle` / `Walk` / `Run`) — the source of truth.
- A `PlayerAnimations` component holding handles to all three sheets and their atlas layouts.
- Per-sheet frame counts and durations as constants.

And two systems: `update_animation_state` (input → state) and `apply_animation` (state → sprite). The shared `advance_animation` engine from Part 1 keeps cycling frames untouched — it doesn't care which sheet is bound.

### Step 1 — The state component and the asset handles

Create the two new components in `src/player/components.rs`. `AnimationState` is a plain enum; we derive `Default` and mark `Idle` as the default so the monarch starts idle:

```rust
// src/player/components.rs
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Run,
}
```

`PlayerAnimations` stores the three sheet handles and three layout handles. We keep them on the player entity rather than in a resource: the data belongs to the player, and the apply system can fetch everything it needs from one query with no resource lookups:

```rust
// src/player/components.rs
#[derive(Component)]
pub struct PlayerAnimations {
    pub idle: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub walk: Handle<Image>,
    pub walk_layout: Handle<TextureAtlasLayout>,
    pub run: Handle<Image>,
    pub run_layout: Handle<TextureAtlasLayout>,
}
```

A `Resource` would work just as well here — with exactly one player there is exactly one set of handles either way, so the choice is preference, not architecture. (Recall from Part 2 that since Bevy 0.19, resources *are* components under the hood, stored on special singleton entities; `Res<T>` is just an ergonomic way to reach them.) We use a component because the data conceptually belongs to the player and the apply system gets everything from a single entity query. If you'd rather keep the player entity lean, `#[derive(Resource)]` plus `Res<PlayerAnimations>` in `apply_animation` is an equally valid design.

### Step 2 — Deriving the state from input

Create `src/player/animation.rs` and declare it in `src/player/mod.rs` alongside the other player modules:

```rust
// src/player/mod.rs
pub mod animation;
```

The first system reads the logical input and computes the desired state. It writes to the component **only when the value differs** — this is what keeps the `Changed` filter downstream quiet while the player keeps doing the same thing:

```rust
// src/player/animation.rs
pub fn update_animation_state(
    input: Res<PlayerInput>,
    mut player: Single<&mut AnimationState, With<Player>>,
) {
    let desired = if input.move_axis != 0.0 {
        if input.run { AnimationState::Run } else { AnimationState::Walk }
    } else {
        AnimationState::Idle
    };

    if **player != desired {
        **player = desired;
    }
}
```

Queries: `Res<PlayerInput>` is the input produced by `gather_input`; `Single<&mut AnimationState, With<Player>>` is the one player's state. (`**player` dereferences twice: once through `Single`, once through Bevy's `Mut` change-tracking wrapper, to reach the enum itself.)

### Step 3 — Applying the state to the sprite

The second system does the actual sheet swap. Its `Single` carries a `Changed<AnimationState>` filter, so the whole system is skipped on frames where the state didn't change:

```rust
// src/player/animation.rs
pub fn apply_animation(
    player: Single<
        (&AnimationState, &PlayerAnimations, &mut Sprite, &mut SpriteAnimation),
        (With<Player>, Changed<AnimationState>),
    >,
) {
    let (state, animations, mut sprite, mut animation) = player.into_inner();

    let (image, layout, frame_count, frame_duration) = match state {
        AnimationState::Idle => (&animations.idle, &animations.idle_layout, IDLE_FRAME_COUNT, IDLE_FRAME_DURATION),
        AnimationState::Walk => (&animations.walk, &animations.walk_layout, WALK_FRAME_COUNT, WALK_FRAME_DURATION),
        AnimationState::Run => (&animations.run, &animations.run_layout, RUN_FRAME_COUNT, RUN_FRAME_DURATION),
    };
    // ... swap and reset (see below)
}
```

The four-tuple fetches everything the swap needs in one query: `&AnimationState` says *which* sheet, `&PlayerAnimations` provides the handles, `&mut Sprite` receives the new image/layout, and `&mut SpriteAnimation` receives the new frame data. (`into_inner()` consumes the `Single` and hands us the inner tuple, which we destructure.)

The swap itself is three moves. The image and layout are replaced, the atlas index is reset to frame 0, and the `SpriteAnimation` is rebuilt with the new sheet's frame count and speed:

```rust
// src/player/animation.rs (inside apply_animation)
    sprite.image = image.clone();
    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.layout = layout.clone();
        atlas.index = 0;
    }
    *animation = SpriteAnimation::new(frame_duration, frame_count);
```

Resetting `index` to 0 matters: the walk/run sheets have 6 frames but idle has only 5. A stale index of 5 left over from a walk sheet would point past the end of the idle layout. Restarting from frame 0 also looks right — a new action starts at the beginning of its cycle.

The constants live at the top of the same file:

```rust
// src/player/animation.rs
pub const IDLE_FRAME_DURATION: Duration = Duration::from_millis(200);
pub const WALK_FRAME_DURATION: Duration = Duration::from_millis(100);
pub const RUN_FRAME_DURATION: Duration = Duration::from_millis(100);

pub const IDLE_FRAME_COUNT: usize = 5;
pub const WALK_FRAME_COUNT: usize = 6;
pub const RUN_FRAME_COUNT: usize = 6;
```

Walk and run cycle faster than idle so the feet match the higher movement speed.

One aside: that nested `Single` type in `apply_animation` is long enough to trip clippy's `type_complexity` lint. The usual fix — a `type` alias — breaks Bevy's system-trait inference, so the project instead allows the lint project-wide in `Cargo.toml`, where Bevy's routinely-complex query types make it mostly noise:

```toml
# Cargo.toml
[lints.clippy]
type_complexity = "allow"
```

### Step 4 — Loading all three sheets at spawn

`spawn_player` in `src/player/mod.rs` now loads all three sheets, builds all three atlas layouts, and inserts the two new components. The idle sheet stays bound initially, matching `AnimationState::default()`:

```rust
// src/player/mod.rs (inside spawn_player)
    let idle_image = asset_server.load("Player/Idle.png");
    let walk_image = asset_server.load("Player/Walk.png");
    let run_image = asset_server.load("Player/Run.png");

    let idle_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::splat(FRAME_SIZE), animation::IDLE_FRAME_COUNT as u32, 1, None, None,
    ));
    // ... same for walk_layout (6 frames) and run_layout (6 frames)
```

All three sheets are 128×128 frames in a single row, so one `from_grid` call per sheet is all it takes. The spawn bundle gains `AnimationState::default()` and the `PlayerAnimations` struct holding all six handles; the initial `Sprite` and `SpriteAnimation` still point at idle. See `fn spawn_player` in `src/player/mod.rs` for the full function.

### Step 5 — Wiring with explicit ordering

The three `Update` systems form a pipeline: input → state → sprite. We register them with `.chain()`:

```rust
// src/player/mod.rs
    .add_systems(
        Update,
        (
            movement::gather_input,
            animation::update_animation_state,
            animation::apply_animation,
        )
            .chain(),
    )
    .add_systems(Update, camera::follow_camera)
```

Why the chain? Each system's output is the next system's input. Bevy would serialize them anyway (they access the same data), but in an *arbitrary order* — if `apply_animation` happened to run before `update_animation_state`, the sheet swap would lag a frame behind the input. The chain guarantees the whole pipeline propagates within a single frame.

`follow_camera` stays outside the chain: it touches only `Transform` and `Time`, shares no data with the pipeline, and is free to run in parallel with it.

> **Run the game now.** Walk with `A`/`D` — the monarch's legs should move. Hold `Shift` — she breaks into a run. Release everything — she settles back to idle. Every transition should be instant, with no flicker or out-of-place frames.

### Simplifications

- **Instant swaps, no blending.** A more polished game might cross-fade or preserve the cycle position between walk and run so the feet don't "jump." For crisp pixel art at this scale, restarting from frame 0 looks fine.
- **Hardcoded frame counts and durations.** This keeps the tutorial focused on the state machine pattern. A larger project would define animations as data — an asset file or a resource mapping states to sheets, layouts, and timings — so designers can tune without recompiling.
- **State derived from input, not velocity.** If the monarch is ever pushed by knockback or blocked by a wall, input and actual motion can disagree. When that day comes, deriving the state from the `Transform` delta instead is a one-system change — the apply side is unaffected.

---

## Summary

- `AnimationState` is the single source of truth; `PlayerAnimations` holds the sheet and layout handles on the player entity.
- `update_animation_state` *derives* the state from input and writes only on change; `apply_animation` *applies* it, gated by `Changed<AnimationState>` so it runs only on transitions.
- `.chain()` orders the input → state → sprite pipeline so transitions land in the same frame the keys change.
- The shared `advance_animation` engine from Part 1 needed no changes — it cycles whatever sheet is currently bound.
- The architecture is ready for one-shot animations: a hurt or coin-toss animation is a new `AnimationState` variant, new handles, and one system that writes the state — the apply logic stays untouched.

Milestone 1 is complete: the monarch idles, walks, runs, flips, and roams an infinite meadow with the camera in tow. In the next part we'll step back from gameplay to set up testing and CI (Milestone 2), so the project grows on a safety net.
