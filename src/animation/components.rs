//! Generic sprite-sheet animation components.
//!
//! These are shared by every animated entity (player, NPCs, enemies, …).
//! The frame-cycling engine lives in [`crate::animation`] (`advance_animation`);
//! per-entity code only decides *which* sheet is bound and how fast it plays.

use bevy::prelude::*;
use std::time::Duration;

/// Drives sprite-sheet animation by cycling the active `TextureAtlas` frame.
///
/// `frames` is the total number of frames in the currently bound sheet; the
/// timer fires once per frame and the index advances modulo `frames`. Any
/// entity with this component plus a [`Sprite`] is animated by the shared
/// [`crate::animation::advance_animation`] system — no per-entity system needed.
#[derive(Component)]
pub struct SpriteAnimation {
    /// Counts down to the next frame advance.
    pub timer: Timer,
    /// Total frame count of the currently bound sprite sheet.
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
