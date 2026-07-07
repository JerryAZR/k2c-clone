//! The player domain: the controllable monarch.
//!
//! CP1 (this checkpoint) delivers only idle animation:
//! - spawn the monarch with the `Idle.png` sprite sheet,
//! - build a `TextureAtlasLayout` for the 128×128 frames,
//! - cycle frames on a timer (via the shared `AnimationPlugin`).
//!
//! Movement (CP3), the walk/run sheets (CP4), and the follow camera (CP3)
//! are added in later checkpoints.

pub mod components;

use crate::animation::SpriteAnimation;
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::prelude::*;
use std::time::Duration;

/// Frame size of the player sprite sheets (square, in source pixels).
const FRAME_SIZE: u32 = 128;

/// Per-frame duration of the idle animation.
const IDLE_FRAME_DURATION: Duration = Duration::from_millis(200);

/// Number of frames in the idle sheet (`Idle.png` is 640×128 → 5 frames).
const IDLE_FRAMES: usize = 5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

/// Spawns the monarch entity with the idle sprite sheet bound.
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("Player/Idle.png");
    let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::splat(FRAME_SIZE),
        IDLE_FRAMES as u32,
        1,
        None,
        None,
    ));

    commands.spawn((
        Name::new("Player"),
        components::Player,
        SpriteAnimation::new(IDLE_FRAME_DURATION, IDLE_FRAMES),
        Sprite {
            image,
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            ..default()
        },
        Transform::default(),
    ));
}
