//! Generic sprite-sheet animation engine.
//!
//! A single system, [`advance_animation`], cycles the `TextureAtlas` frame of
//! every entity that has a [`SpriteAnimation`](components::SpriteAnimation)
//! component plus a [`Sprite`]. This is Bevy's canonical pattern for sprite-sheet
//! animation (see Bevy's `2d/sprite_sheet.rs` and `2d/sprite_animation.rs`
//! examples). The heavier `bevy_animation` (`AnimationPlayer` / `AnimationClip`)
//! system is built around interpolatable fields (color, transform, …) and is
//! reserved for later milestones where it fits (e.g. day/night tinting).
//!
//! Domain code (e.g. the player) only decides *which* sheet is bound and
//! triggers swaps on state changes; it never re-implements frame cycling.

pub mod components;

use bevy::prelude::*;

pub use components::SpriteAnimation;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, advance_animation);
    }
}

/// Advances every animated sprite one frame whenever its timer elapses.
///
/// Generic and entity-agnostic: any entity with [`SpriteAnimation`] + [`Sprite`]
/// is animated. Runs in `Update` because animation is visual feedback, not
/// simulation.
fn advance_animation(time: Res<Time>, mut query: Query<(&mut SpriteAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());
        if !animation.timer.just_finished() {
            continue;
        }

        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.frames > 0 {
            atlas.index = (atlas.index + 1) % animation.frames;
        }
    }
}
