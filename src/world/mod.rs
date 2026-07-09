//! World domain: the kingdom's physical space (ground, sky, background).

pub mod background;

pub use background::{HORIZON_Y, VIEWPORT_HEIGHT};

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, background::spawn_background)
            .add_systems(Update, background::update_background);
    }
}
