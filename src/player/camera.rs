//! Player camera: smooth follow for the monarch.
//!
//! The camera follows the player horizontally and keeps a fixed vertical
//! position. It runs in [`Update`](bevy::app::Update) because camera follow is
//! visual feedback, not simulation logic.

use crate::player::components::Player;
use bevy::prelude::*;

/// Higher values make the camera snap to the player faster.
const CAMERA_FOLLOW_SPEED: f32 = 6.0;

/// Smoothly interpolates the camera toward the player's horizontal position.
pub fn follow_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_x = player.translation.x;
    let camera = &mut *camera;

    // Follow only the x-axis; y stays fixed for this side-scroller.
    let target = Vec3::new(player_x, camera.translation.y, camera.translation.z);

    // Frame-rate independent exponential lerp.
    let t = 1.0 - (-CAMERA_FOLLOW_SPEED * time.delta_secs()).exp();
    camera.translation = camera.translation.lerp(target, t);
}
