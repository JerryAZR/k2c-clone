//! Components for the player entity.
//!
//! CP1 introduced only the [`Player`] marker. That marker remains the sole
//! player-specific component in CP3.
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
