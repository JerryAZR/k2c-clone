//! Components for the player entity.
//!
//! CP1 introduced only the [`Player`] marker. CP3 kept the player entity lean
//! while adding movement and camera follow. CP4 adds [`AnimationState`] and
//! [`PlayerAnimations`] for the walk/run animation state machine.
//!
//! The shared animation component ([`crate::animation::SpriteAnimation`]) lives
//! in the `animation` module and is reused by all animated entities.

use bevy::prelude::*;

/// Marker component for the player-controlled monarch.
///
/// Intentionally minimal: the `Sprite` we spawn alongside it already requires
/// `Transform` and `Visibility`, so this marker only carries identity.
/// Facing is stored directly on `Sprite.flip_x` in CP3.
#[derive(Component, Default)]
pub struct Player;

/// The current animation state of the monarch.
///
/// This component is the single source of truth for which animation sheet is
/// active. Systems that want to change the monarch's animation update this
/// component; `player/animation.rs` applies the change to the sprite.
///
/// For CP4 we only need movement-derived states. One-shot animations (hurt,
/// coin toss, etc.) can be added later as new variants without rewriting the
/// application logic.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Run,
}

/// Animation assets for the monarch.
///
/// Stores handles to the idle/walk/run sprite sheets and their atlas layouts.
/// Kept on the player entity so the animation state machine can swap sheets
/// without a resource lookup.
#[derive(Component)]
pub struct PlayerAnimations {
    pub idle: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub walk: Handle<Image>,
    pub walk_layout: Handle<TextureAtlasLayout>,
    pub run: Handle<Image>,
    pub run_layout: Handle<TextureAtlasLayout>,
}
