pub mod camera;

pub use camera::MainCamera;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin);
    }
}
