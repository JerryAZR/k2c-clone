//! Player movement and input handling.
//!
//! CP3 adds controllable movement to the monarch. Input is gathered in
//! [`Update`](bevy::app::Update) and consumed in [`FixedUpdate`](bevy::app::FixedUpdate)
//! so the movement simulation is deterministic and frame-rate independent.
//!
//! The input abstraction (`PlayerInput`) is a resource that separates the raw
//! input devices (keyboard today, gamepad tomorrow) from the gameplay logic.

use crate::player::components::Player;
use bevy::prelude::*;

/// Horizontal movement speed in world units per second.
const WALK_SPEED: f32 = 150.0;
const RUN_SPEED: f32 = 300.0;

/// Logical input produced by the physical input layer.
///
/// This is the bridge between the variable-timestep input gathering and the
/// fixed-timestep gameplay logic. For CP3 we only need horizontal movement and
/// a run modifier; discrete actions like the coin key will be added when that
/// feature is implemented.
#[derive(Resource, Default)]
pub struct PlayerInput {
    /// Horizontal movement axis: -1.0 (left) to +1.0 (right).
    pub move_axis: f32,
    /// Run modifier held.
    pub run: bool,
}

/// Reads raw keyboard input and writes the logical `PlayerInput` state.
///
/// Runs in `Update` so it sees the frame-fresh `ButtonInput`. Because
/// `FixedUpdate` runs before `Update` in Bevy's default schedule, the gameplay
/// logic uses the *previous* frame's input — a one-frame latency that is
/// acceptable for this project and avoids mixing input and simulation schedules.
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

    input.run = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
}

/// Applies movement and facing based on the logical input.
///
/// Runs in `FixedUpdate` so the monarch moves at a deterministic speed.
pub fn apply_movement(
    input: Res<PlayerInput>,
    fixed_time: Res<Time<Fixed>>,
    mut player: Single<(&mut Transform, &mut Sprite), With<Player>>,
) {
    let (transform, sprite) = &mut *player;

    let speed = if input.run { RUN_SPEED } else { WALK_SPEED };
    let velocity = input.move_axis * speed;
    transform.translation.x += velocity * fixed_time.delta_secs();

    // Face the direction of movement. When the player stops, the last facing
    // direction is preserved because we only flip when the axis is non-zero.
    if input.move_axis > 0.0 {
        sprite.flip_x = false;
    } else if input.move_axis < 0.0 {
        sprite.flip_x = true;
    }
}
