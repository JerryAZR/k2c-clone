use bevy::prelude::*;

mod animation;
mod player;

fn main() {
    App::new()
        // Nearest-neighbor sampling keeps the pixel-art sprites crisp.
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// Spawns a static camera for now; the follow camera arrives in CP3.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
