//! Components for the player entity.
//!
//! CP1 introduces only the [`Player`] marker. `PlayerState` / `Facing`
//! (movement + state machine) arrive in CP3.
//!
//! The shared animation component ([`crate::animation::SpriteAnimation`]) lives
//! in the `animation` module and is reused by all animated entities.

use bevy::prelude::*;

/// Marker component for the player-controlled monarch.
///
/// Intentionally minimal: the `Sprite` we spawn alongside it already requires
/// `Transform` and `Visibility`, so this marker only carries identity.
#[derive(Component, Default)]
pub struct Player;
