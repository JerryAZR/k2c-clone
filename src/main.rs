use bevy::camera::ScalingMode;
use bevy::prelude::*;

mod animation;
mod player;
mod world;

fn main() {
    App::new()
        // Nearest-neighbor sampling keeps the pixel-art sprites crisp.
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(world::WorldPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// Spawns a static camera for now; the follow camera arrives in CP3.
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: world::VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
