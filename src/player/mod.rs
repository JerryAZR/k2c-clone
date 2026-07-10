//! The player domain: the controllable monarch.
//!
//! CP1 delivered the idle animation. CP3 (this checkpoint) adds movement and
//! camera follow:
//! - the monarch moves left/right in `FixedUpdate`,
//! - input is gathered in `Update` through a logical `PlayerInput` resource,
//! - the camera follows her smoothly with a lerp.
//!
//! The walk/run sheet swap arrives in CP4.

pub mod camera;
pub mod components;
pub mod movement;

use crate::animation::SpriteAnimation;
use crate::world::HORIZON_Y;
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
        app.init_resource::<movement::PlayerInput>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (movement::gather_input, camera::follow_camera))
            .add_systems(FixedUpdate, movement::apply_movement);
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
        Transform::from_xyz(0.0, HORIZON_Y + FRAME_SIZE as f32 / 2.0, 0.0),
    ));
}
