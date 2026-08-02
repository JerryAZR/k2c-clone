//! The player domain: the controllable monarch.
//!
//! CP1 delivered the idle animation. CP3 added movement and camera follow.
//! CP4 (this checkpoint) adds the walk/run animation state machine:
//! - the monarch swaps between idle, walk, and run sprite sheets based on input,
//! - the shared animation engine keeps cycling frames.
//!
//! One-shot animations (hurt, coin toss, etc.) can be added later by extending
//! `AnimationState` without rewriting the application logic.

pub mod animation;
pub mod camera;
pub mod components;
pub mod movement;

use crate::animation::SpriteAnimation;
use crate::world::HORIZON_Y;
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::prelude::*;

/// Frame size of the player sprite sheets (square, in source pixels).
const FRAME_SIZE: u32 = 128;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<movement::PlayerInput>()
            .add_systems(Startup, spawn_player)
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
            .add_systems(FixedUpdate, movement::apply_movement);
    }
}

/// Spawns the monarch entity with the idle sprite sheet bound.
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load all animation sheets.
    let idle_image = asset_server.load("Player/Idle.png");
    let walk_image = asset_server.load("Player/Walk.png");
    let run_image = asset_server.load("Player/Run.png");

    // Create the atlas layouts (all sheets are 128×128 frames in a single row).
    let idle_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::splat(FRAME_SIZE),
        animation::IDLE_FRAME_COUNT as u32,
        1,
        None,
        None,
    ));
    let walk_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::splat(FRAME_SIZE),
        animation::WALK_FRAME_COUNT as u32,
        1,
        None,
        None,
    ));
    let run_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::splat(FRAME_SIZE),
        animation::RUN_FRAME_COUNT as u32,
        1,
        None,
        None,
    ));

    commands.spawn((
        Name::new("Player"),
        components::Player,
        components::AnimationState::default(),
        components::PlayerAnimations {
            idle: idle_image.clone(),
            idle_layout: idle_layout.clone(),
            walk: walk_image,
            walk_layout,
            run: run_image,
            run_layout,
        },
        SpriteAnimation::new(animation::IDLE_FRAME_DURATION, animation::IDLE_FRAME_COUNT),
        Sprite {
            image: idle_image,
            texture_atlas: Some(TextureAtlas {
                layout: idle_layout,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, HORIZON_Y + FRAME_SIZE as f32 / 2.0, 0.0),
    ));
}
