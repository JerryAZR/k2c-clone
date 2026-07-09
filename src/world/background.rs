//! World background: the infinitely-repeating meadow.
//!
//! CP2 pins a horizontally seamless background to the world and aligns the
//! monarch to its horizon. The camera is configured with a fixed world-space
//! viewport height, so the horizon is a constant in world coordinates and the
//! background always fills the window vertically.

use bevy::prelude::*;

/// Background texture size, in source pixels.
const BG_WIDTH: f32 = 2304.0;
const BG_HEIGHT: f32 = 1296.0;

/// The horizon is measured as a ratio from the bottom of the background image.
const HORIZON_RATIO: f32 = 0.35;

/// Fixed world-space height of the viewport. The camera is configured so that
/// this many world units always fill the window vertically.
pub const VIEWPORT_HEIGHT: f32 = 720.0;

/// World-space y coordinate of the ground line (the background horizon).
///
/// This is a constant because the viewport height is fixed; the camera maps it
/// to the window regardless of the actual window size.
pub const HORIZON_Y: f32 = VIEWPORT_HEIGHT * (HORIZON_RATIO - 0.5);

/// Background tiles are snapped to a grid centered on the camera each frame.
#[derive(Component)]
pub struct BackgroundTile;

/// Stores the number of background tiles needed to cover the viewport.
#[derive(Resource)]
pub struct BackgroundStrip {
    pub tile_count: usize,
}

/// Spawns the background tiles needed to cover the viewport.
///
/// Tile count is derived from the initial window aspect ratio so ultra-wide
/// monitors are covered automatically.
pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    let window = window.single().unwrap();
    let bg_scale = VIEWPORT_HEIGHT / BG_HEIGHT;
    let tile_width = BG_WIDTH * bg_scale;
    let viewport_width = VIEWPORT_HEIGHT * (window.width() / window.height());
    let tile_count = ((viewport_width / tile_width).ceil() as usize + 2).max(3);

    commands.insert_resource(BackgroundStrip { tile_count });

    let image = asset_server.load("Summer2.png");

    let half = (tile_count as f32 - 1.0) / 2.0;
    for i in 0..tile_count {
        let x = (i as f32 - half) * tile_width;
        commands.spawn((
            BackgroundTile,
            Sprite {
                image: image.clone(),
                ..default()
            },
            Transform::from_xyz(x, 0.0, -10.0).with_scale(Vec3::splat(bg_scale)),
        ));
    }
}

/// Snaps background tiles to a contiguous block centered on the camera's
/// current tile. Run every frame so the meadow appears infinite.
pub fn update_background(
    strip: Res<BackgroundStrip>,
    camera: Query<&Transform, (With<Camera2d>, Without<BackgroundTile>)>,
    mut tiles: Query<&mut Transform, (With<BackgroundTile>, Without<Camera2d>)>,
) {
    let camera_x = camera.single().unwrap().translation.x;
    let tile_width = BG_WIDTH * (VIEWPORT_HEIGHT / BG_HEIGHT);

    let camera_tile = (camera_x / tile_width).floor() as i32;
    let half = (strip.tile_count as i32 - 1) / 2;
    let start_tile = camera_tile - half;

    for (i, mut transform) in tiles.iter_mut().enumerate() {
        let slot = start_tile + i as i32;
        transform.translation.x = slot as f32 * tile_width;
    }
}
