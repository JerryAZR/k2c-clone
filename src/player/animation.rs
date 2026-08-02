//! Player animation state machine.
//!
//! This module watches the player's logical input and swaps the monarch's
//! sprite sheet between idle, walk, and run. The shared `animation` module
//! keeps cycling frames; this module just decides which sheet and which
//! frames to cycle through.
//!
//! The state is stored in [`AnimationState`] on the player entity.
//! [`update_animation_state`] derives the state from [`PlayerInput`] and writes
//! it. [`apply_animation`] runs when [`AnimationState`] changes and updates the
//! sprite and animation data.
//!
//! Because the application logic only reads [`AnimationState`], adding a new
//! one-shot animation (hurt, coin toss, etc.) later is an extension, not a
//! rewrite: add a variant to [`AnimationState`], add its handles to
//! [`PlayerAnimations`], and add a system that writes the new state.

use crate::animation::components::SpriteAnimation;
use crate::player::components::{AnimationState, Player, PlayerAnimations};
use crate::player::movement::PlayerInput;
use bevy::prelude::*;
use std::time::Duration;

/// Frame durations for each animation.
pub const IDLE_FRAME_DURATION: Duration = Duration::from_millis(200);
pub const WALK_FRAME_DURATION: Duration = Duration::from_millis(100);
pub const RUN_FRAME_DURATION: Duration = Duration::from_millis(100);

/// Frame counts for each animation.
pub const IDLE_FRAME_COUNT: usize = 5;
pub const WALK_FRAME_COUNT: usize = 6;
pub const RUN_FRAME_COUNT: usize = 6;

/// Updates the monarch's animation state from the current movement input.
///
/// Runs in `Update` after [`PlayerInput`] is gathered. Only writes when the
/// state actually changes, so `Changed<AnimationState>` downstream stays quiet
/// while the player keeps doing the same thing.
pub fn update_animation_state(
    input: Res<PlayerInput>,
    mut player: Single<&mut AnimationState, With<Player>>,
) {
    let desired = if input.move_axis != 0.0 {
        if input.run {
            AnimationState::Run
        } else {
            AnimationState::Walk
        }
    } else {
        AnimationState::Idle
    };

    if **player != desired {
        **player = desired;
    }
}

/// Applies the current animation state to the sprite and animation data.
///
/// Runs in `Update` when [`AnimationState`] changes. Swaps the sprite sheet
/// and layout, resets the atlas index to the first frame, and replaces the
/// [`SpriteAnimation`] so the shared engine starts cycling the new sheet from
/// the beginning.
pub fn apply_animation(
    player: Single<
        (
            &AnimationState,
            &PlayerAnimations,
            &mut Sprite,
            &mut SpriteAnimation,
        ),
        (With<Player>, Changed<AnimationState>),
    >,
) {
    let (state, animations, mut sprite, mut animation) = player.into_inner();

    let (image, layout, frame_count, frame_duration) = match state {
        AnimationState::Idle => (
            &animations.idle,
            &animations.idle_layout,
            IDLE_FRAME_COUNT,
            IDLE_FRAME_DURATION,
        ),
        AnimationState::Walk => (
            &animations.walk,
            &animations.walk_layout,
            WALK_FRAME_COUNT,
            WALK_FRAME_DURATION,
        ),
        AnimationState::Run => (
            &animations.run,
            &animations.run_layout,
            RUN_FRAME_COUNT,
            RUN_FRAME_DURATION,
        ),
    };

    // Swap the sheet and layout, then restart from frame 0.
    sprite.image = image.clone();
    if let Some(atlas) = sprite.texture_atlas.as_mut() {
        atlas.layout = layout.clone();
        atlas.index = 0;
    }

    // Reset the animation data to the new sheet.
    *animation = SpriteAnimation::new(frame_duration, frame_count);
}
